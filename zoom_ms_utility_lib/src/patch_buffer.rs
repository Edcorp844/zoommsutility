/* patch_buffer.rs
 *
 * Copyright 2026 Frost
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::apatch::Apatch;
use crate::decoded_slot::DecodedSlot;
use crate::effects::{self, EffectDb, EffectDef};
use crate::patch_info::PatchInfo;
use crate::sysex::SysExCommandBuilder;

/// Represents a patch in the library
#[derive(Debug, Clone)]
pub struct PatchBuffer {
    pub name: String,
    pub data: Apatch,
    pub slot: Option<u8>,
}

impl PatchBuffer {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        //minimum bytes valid length is 10
        if bytes.len() < 10 {
            return Err("Data too short".to_string());
        }

        // Look for the start of the patch data
        let start = if bytes.len() > 6 && bytes[0] == 0xF0 && bytes[1] == 0x52 {
            // Try to find where the data starts
            let mut data_start = 5; // Default after header

            // Some responses have the data after the command byte
            if bytes.len() > 5 && bytes[4] == 0x28 || bytes[4] == 0x29 {
                data_start = 5;
            } else if bytes.len() > 6 && bytes[5] == 0x28 || bytes[5] == 0x29 {
                data_start = 6;
            }

            // Try to find the actual data by looking for 7-bit encoded data
            // The data typically starts with a header byte 0x00-0x7F
            for i in data_start..bytes.len().min(data_start + 10) {
                if bytes[i] < 0x80 {
                    data_start = i;
                    break;
                }
            }
            data_start
        } else {
            0
        };

        // Find the end of the data
        let end = if start > 0 {
            // Look for the end marker
            let mut end_pos = bytes.len();
            for i in start..bytes.len() {
                if bytes[i] == 0xF7 {
                    end_pos = i;
                    break;
                }
            }
            end_pos
        } else {
            bytes.len()
        };

        if end <= start {
            return Err("No valid data found".to_string());
        }

        // Decode the 7-bit data
        let raw_data = SysExCommandBuilder::decode_7bit(&bytes[start..end]);

        // Ensure we have enough data
        if raw_data.len() < 10 {
            return Err("Decoded data too short".to_string());
        }

        let mut patch = Apatch::new();
        patch.read_bin(&raw_data);

        // Make sure we have a valid name
        if patch.name.is_empty() {
            patch.name = "Unnamed".to_string();
        }

        Ok(PatchBuffer {
            name: patch.name.clone(),
            data: patch,
            slot: None,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn decode_slot(&self, slot: usize) -> Option<DecodedSlot> {
        if slot >= 6 {
            return None;
        }

        let fx_slot = &self.data.fx[slot];

        Some(DecodedSlot {
            raw_effect_id: fx_slot[1] as u32,
            is_enabled: fx_slot[0] != 0,
            params: [
                fx_slot[2] as u16,
                fx_slot[3] as u16,
                fx_slot[4] as u16,
                fx_slot[5] as u16,
                fx_slot[6] as u16,
                fx_slot[7] as u16,
                fx_slot[8] as u16,
                fx_slot[9] as u16,
                fx_slot[10] as u16,
            ],
        })
    }

    pub fn is_slot_enabled(&self, slot: usize) -> Option<bool> {
        if slot >= 6 {
            return None;
        }
        Some(self.data.fx[slot][0] != 0)
    }

    pub fn set_slot_enabled(&mut self, slot: usize, enabled: bool) {
        if slot < 6 {
            self.data.fx[slot][0] = if enabled { 1 } else { 0 };
        }
    }

    pub fn set_slot_param(&mut self, slot: usize, param: usize, value: u16) {
        if slot < 6 && param < 9 {
            self.data.fx[slot][param + 2] = value as i32;
        }
    }

    pub fn analyse_patch(&self) -> Result<PatchInfo, Box<dyn std::error::Error>> {
        let name = self.name.clone();
        let mut effect_ids = [0u32; 6];
        let mut active = 0usize;
        let mut warnings = Vec::new();

        for s in 0..6 {
            if let Some(ds) = self.decode_slot(s) {
                effect_ids[s] = ds.raw_effect_id;
                if ds.is_enabled && ds.raw_effect_id != 0 {
                    active += 1;
                }
            }
        }

        let estimated_dsp = active as f32 * 0.18;
        if estimated_dsp > 0.95 {
            warnings.push("High DSP".into());
        }

        if let Some(slot) = self.slot {
            Ok(PatchInfo {
                number: slot,
                name,
                buffer: self.clone(),
                active_slots: active,
                effect_ids,
                estimated_dsp,
                warnings,
                errors: Vec::new(),
                is_empty: active == 0,
            })
        } else {
            Err(format!("Found no patch slot number").into())
        }
    }

    pub fn get_patch_effects(&self) -> Result<Vec<EffectDef>, Box<dyn std::error::Error>> {
        let db = effects::build_effect_db();
        let mut effects_vec = Vec::new();
        for slot in 0..6 {
            if let Some(decoded) = self.decode_slot(slot) {
                match db.get(&decoded.raw_effect_id) {
                    Some(def) => {
                        effects_vec.push(def.clone());
                    }
                    None => return Err(format!("Could not decode effect at slot {slot}").into()),
                }
            }
        }

        Ok(effects_vec)
    }

    /// Full 6-bit mask
    pub fn dsp_state_mask(&self) -> u8 {
        (self.data.dspstate & 0x3f) as u8
        // or if PatchBuffer *is* the logical patch:
        // (self.dspstate & 0x3f) as u8
    }

    /// Per-slot: 1 = pedal marked this slot DSP-full / silent
    pub fn is_dsp_killed(&self, slot: usize) -> bool {
        if slot >= 6 {
            return false;
        }
        self.data.get_dsp_state(slot as u32) != 0
    }

    pub fn show_full_details(&self, db: &EffectDb) {
        println!("\n==============================================");
        println!(" Patch: \"{}\"", self.name());
        println!("==============================================");

        for slot in 0..6 {
            if let Some(decoded) = self.decode_slot(slot) {
                let state = if decoded.is_enabled { "ON " } else { "OFF" };
                if decoded.raw_effect_id == 0 {
                    println!("Slot {} [{state}] THRU / Empty", slot + 1);
                    continue;
                }
                match db.get(&decoded.raw_effect_id) {
                    Some(def) => {
                        println!("\nSlot {} [{state}] {} ({})", slot + 1, def.name, def.group);
                        println!("  {}", def.title);
                        println!("  DSP: {:.2}", def.dsp);
                        for (i, p) in def.params.iter().enumerate() {
                            let raw = decoded.params.get(i).copied().unwrap_or(0);
                            println!("   {:<10} {:>5}", p.name, p.format_param(raw));
                        }
                    }
                    None => {
                        println!(
                            "\nSlot {} [{state}] Unknown (0x{:08X})",
                            slot + 1,
                            decoded.raw_effect_id
                        );
                    }
                }
            }
        }
        println!("==============================================\n");
    }
}
