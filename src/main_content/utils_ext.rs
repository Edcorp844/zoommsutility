/* main_content/utils_ext.rs
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

use zoom_ms::effects::{EffectDef, ParamDef, ParamDisp};

use crate::{components::led_indicator::LedState, main_content::model::MainContentModel};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotChainState {
    /// Empty / group == THRU
    Thru,
    /// Running and fits budget
    Active,
    /// DSP full – silent / utility shows THRU
    ForcedThru,
    /// User bypassed (on/off = off)
    Bypassed,
}

///DSP WarningLevel

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WarningLevel {
    Ok,
    Caution,
    Critical,
}

pub(crate) trait MainContentUtils {
    fn hw_to_ui(&self, hw_idx: usize) -> Option<usize>;
    fn ui_to_hw(&self, ui_idx: usize) -> usize;
    fn enabled_flags(&self) -> Vec<bool>;
    fn effect_dsp_cost(&self, fx: &EffectDef) -> f32;
    fn hardware_dsp_bits(&self) -> Option<u8>;
    fn analyse_chain(&self) -> (f32, Vec<SlotChainState>);
    fn led_state_for_slot(&self, state: SlotChainState) -> LedState;
    fn slot_enabled(&self, slot: usize) -> bool;
    fn overall_warning(&self, used: f32, states: &[SlotChainState]) -> WarningLevel;
    fn decode_html_entities(s: &str) -> String;
    fn display_value(param: &ParamDef, val: u16) -> String;
    fn is_labeled_switch(&self, param: &ParamDef) -> bool;
}

impl MainContentUtils for MainContentModel {
    fn hw_to_ui(&self, hw_idx: usize) -> Option<usize> {
        if hw_idx >= 2 {
            Some(hw_idx - 2)
        } else {
            None
        }
    }

    fn ui_to_hw(&self, ui_idx: usize) -> usize {
        ui_idx + 2
    }

    /// Unit cost for one effect (fraction of total DSP)
    /// if dsp <= 0.0 or >= 9000.0 then it is a THRU sentinel or invalid
    fn effect_dsp_cost(&self, fx: &EffectDef) -> f32 {
        if fx.group == "THRU" {
            return 0.0;
        }
        let dsp = fx.dsp as f32;
        if dsp <= 0.0 || dsp >= 9000.0 {
            return 0.0;
        }
        (1.0 / dsp).clamp(0.0, 1.0)
    }

    /// Hardware DSP-full bits (same as apatch.dspstate), if PatchBuffer exposes them.
    fn hardware_dsp_bits(&self) -> Option<u8> {
        self.patch
            .as_ref()
            .map(|patch_buffer| patch_buffer.dsp_state_mask())
    }

    fn enabled_flags(&self) -> Vec<bool> {
        (0..self.effects.len())
            .map(|i| self.slot_enabled(i))
            .collect()
    }

    fn slot_enabled(&self, slot: usize) -> bool {
        self.patch
            .as_ref()
            .and_then(|patch_buffer| patch_buffer.is_slot_enabled(slot))
            .unwrap_or_else(|| {
                self.effects
                    .get(slot)
                    .map(|effect_def| effect_def.group != "THRU")
                    .unwrap_or(false)
            })
    }

    fn analyse_chain(&self) -> (f32, Vec<SlotChainState>) {
        let hw_bits = self.hardware_dsp_bits();

        let mut remaining = 1.0_f32;
        let mut used = 0.0_f32;
        let mut states = Vec::with_capacity(self.effects.len());

        for (i, fx) in self.effects.iter().enumerate() {
            if fx.group == "THRU" {
                states.push(SlotChainState::Thru);
                continue;
            }

            let on = self.enabled_flags().get(i).copied().unwrap_or(true);
            let cost = self.effect_dsp_cost(fx);
            let hw_killed = hw_bits.map(|bit| (bit >> i) & 1 == 1).unwrap_or(false);

            if hw_killed || cost > remaining + 1e-4 {
                states.push(SlotChainState::ForcedThru);
                continue;
            }

            remaining = (remaining - cost).max(0.0);
            used += cost;
            states.push(if on {
                SlotChainState::Active
            } else {
                SlotChainState::Bypassed
            });
        }

        (used.clamp(0.0, 2.0), states)
    }

    fn led_state_for_slot(&self, state: SlotChainState) -> LedState {
        match state {
            SlotChainState::Active => LedState::Ok,
            SlotChainState::Bypassed | SlotChainState::Thru => LedState::Off,
            SlotChainState::ForcedThru => LedState::Critical,
        }
    }

    fn overall_warning(&self, used: f32, states: &[SlotChainState]) -> WarningLevel {
        let forced = states
            .iter()
            .filter(|s| **s == SlotChainState::ForcedThru)
            .count();
        if forced > 0 || used > 0.98 {
            WarningLevel::Critical
        } else if used > 0.70 {
            WarningLevel::Caution
        } else {
            WarningLevel::Ok
        }
    }

    fn decode_html_entities(s: &str) -> String {
        let mut out = String::with_capacity(s.len());
        let mut rest = s;
        while let Some(start) = rest.find("&#") {
            out.push_str(&rest[..start]);
            rest = &rest[start + 2..];
            let end = match rest.find(';') {
                Some(i) => i,
                None => {
                    out.push_str("&#");
                    break;
                }
            };
            let body = &rest[..end];
            rest = &rest[end + 1..];
            let code = if let Some(hex) = body.strip_prefix('x').or_else(|| body.strip_prefix('X'))
            {
                u32::from_str_radix(hex, 16).ok()
            } else {
                body.parse::<u32>().ok()
            };
            if let Some(cp) = code.and_then(char::from_u32) {
                out.push(cp);
            } else {
                out.push_str("&#");
                out.push_str(body);
                out.push(';');
            }
        }
        out.push_str(rest);
        out
    }

    fn display_value(param: &ParamDef, val: u16) -> String {
        match &param.disp {
            ParamDisp::None => val.to_string(),
            ParamDisp::Offset(offset) => (val as i32 + *offset as i32).to_string(),
            ParamDisp::Labels(labels) if !labels.is_empty() => {
                let idx = val as usize;
                if idx < labels.len() {
                    Self::decode_html_entities(&labels[idx])
                } else {
                    val.to_string()
                }
            }
            ParamDisp::Time { min, max, list } => {
                if list.is_empty() {
                    return val.to_string();
                }
                let max_u = *max as i32;
                let list_len = list.len() as i32;
                let first_note = max_u - list_len + 1;
                if (val as i32) >= first_note {
                    let idx = (val as i32 - first_note) as usize;
                    if idx < list.len() {
                        return Self::decode_html_entities(&list[idx]);
                    }
                }
                if (*min as u16) > 0 && val < *min as u16 {
                    return (*min).to_string();
                }
                val.to_string()
            }
            _ => val.to_string(),
        }
    }

    fn is_labeled_switch(&self, param: &ParamDef) -> bool {
        param.max == 1 && matches!(&param.disp, ParamDisp::Labels(labels) if labels.len() == 2)
    }
}
