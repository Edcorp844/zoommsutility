/* lib.rs
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

pub mod apatch;
pub mod decoded_slot;
pub mod effects;
pub mod midi;
pub mod mock_trait;
pub mod model;
pub mod patch_buffer;
pub mod patch_info;
pub mod patch_library;
pub mod patch_manager;
pub mod sysex;
pub mod sysex_client;

#[macro_use]
extern crate log;

use std::error::Error;
use std::sync::{
    Arc, Mutex,
    mpsc::{Receiver, channel},
};
use std::time::Duration;

use midi::MidiTransport;
use model::PedalModel;
use patch_buffer::PatchBuffer;
use patch_info::PatchInfo;
use patch_library::PatchLibrary;
use patch_manager::PatchManager;

/// Real-time hardware event received from the pedal.
#[derive(Debug, Clone)]
pub enum ZoomMidiEvent {
    ParamEdit { slot: u8, param_idx: u8, value: u16 },
    PatchChange { slot: u8, patch_buffer: PatchBuffer },
    SlotStateChanged { slot: u8, enabled: bool },
    RawSysEx(Vec<u8>),
}

/// Main client for interacting with a Zoom Audio Processor pedal.
/// This is the primary interface for UI applications.
pub struct ZoomClient {
    patch_manager: Arc<Mutex<PatchManager>>,
    event_rx: Arc<Mutex<Option<Receiver<ZoomMidiEvent>>>>,
}

impl ZoomClient {
    /// Create a new client instance and initialize event listener pipes.
    pub fn new() -> Self {
        Self {
            patch_manager: Arc::new(Mutex::new(PatchManager::new())),
            event_rx: Arc::new(Mutex::new(None)),
        }
    }

    /// Takes ownership of the MIDI event stream receiver.
    pub fn take_midi_receiver(&self) -> Option<Receiver<ZoomMidiEvent>> {
        self.event_rx.lock().unwrap().take()
    }

    /// Spawns the internal listener thread to continuously read hardware SysEx updates.
    fn start_hardware_listener(
        &self,
        (tx, rx): (
            std::sync::mpsc::Sender<ZoomMidiEvent>,
            Receiver<ZoomMidiEvent>,
        ),
    ) {
        *self.event_rx.lock().unwrap() = Some(rx);
        let patch_manager = Arc::clone(&self.patch_manager);

        std::thread::spawn(move || {
            loop {
                let msg = {
                    let mut manager = patch_manager.lock().unwrap();
                    manager.transport_mut().recv()
                };

                if let Some(bytes) = msg {
                    // Parse MS Utility specific SysEx events
                    if bytes.len() >= 6 && bytes[0] == 0xF0 && bytes[1] == 0x52 {
                        match bytes[4] {
                            // Live Parameter Edit (0x31) -> [F0, 52, 00, dev, 31, slot, param, lsb, msb, F7]
                            0x31 if bytes.len() >= 9 => {
                                let slot = bytes[5];
                                let param_idx = bytes[6];
                                let val_lsb = bytes[7] as u16;
                                let val_msb = bytes[8] as u16;
                                let value = (val_msb << 7) | val_lsb;

                                if param_idx == 0 {
                                    let _ = tx.send(ZoomMidiEvent::SlotStateChanged {
                                        slot,
                                        enabled: value != 0,
                                    });
                                } else {
                                    let _ = tx.send(ZoomMidiEvent::ParamEdit {
                                        slot,
                                        param_idx,
                                        value,
                                    });
                                }
                            }
                            // Hardware Program/Patch Change (0xC0 command stream)
                            _ => {
                                let _ = tx.send(ZoomMidiEvent::RawSysEx(bytes));
                            }
                        }
                    }
                }
                std::thread::sleep(Duration::from_millis(5));
            }
        });
    }

    /// Connect to a Zoom pedal. Auto-detects the pedal model and scans the library.
    pub fn connect(&self) -> Result<ConnectionStatus, Box<dyn Error>> {
        let mut manager = self.patch_manager.lock().unwrap();

        if manager.is_ready() {
            return Ok(ConnectionStatus::AlreadyConnected);
        }

        // Try to connect the transport
        let connected = manager.transport_mut().try_connect()?;

        if connected {
            manager.setup_with_transport()?;
            drop(manager);

            let (tx, rx) = channel();
            self.start_hardware_listener((tx, rx));

            Ok(ConnectionStatus::Connected)
        } else {
            Ok(ConnectionStatus::NoDeviceFound)
        }
    }

    /// Connect with retry attempts.
    pub fn connect_with_retry(
        &self,
        max_attempts: u8,
        delay_ms: u64,
    ) -> Result<ConnectionStatus, Box<dyn Error>> {
        for attempt in 0..max_attempts {
            match self.connect() {
                Ok(ConnectionStatus::Connected) => return Ok(ConnectionStatus::Connected),
                Ok(ConnectionStatus::AlreadyConnected) => {
                    return Ok(ConnectionStatus::AlreadyConnected);
                }
                Ok(ConnectionStatus::NoDeviceFound) => {
                    if attempt < max_attempts - 1 {
                        print!(".");
                        std::io::Write::flush(&mut std::io::stdout())?;
                        std::thread::sleep(Duration::from_millis(delay_ms));
                    }
                }
                Err(e) => return Err(e),
            }
        }
        println!();
        Ok(ConnectionStatus::NoDeviceFound)
    }

    /// Connects to a selected MIDI port by name.
    pub fn connect_to_port(&self, port_name: String) -> Result<ConnectionStatus, Box<dyn Error>> {
        let mut manager = self.patch_manager.lock().unwrap();

        match manager.connect_to_port(&port_name) {
            Ok(_) => {
                drop(manager);
                let (tx, rx) = channel();
                self.start_hardware_listener((tx, rx));
                Ok(ConnectionStatus::Connected)
            }
            Err(error) => {
                error!("Error connecting to port: {}", error.to_string());
                Ok(ConnectionStatus::NoDeviceFound)
            }
        }
    }

    /// Check if connected to a pedal.
    pub fn is_connected(&self) -> bool {
        let manager = self.patch_manager.lock().unwrap();
        manager.is_ready()
    }

    /// Get the connected pedal model.
    pub fn model(&self) -> Option<PedalModel> {
        let manager = self.patch_manager.lock().unwrap();
        manager.model()
    }

    /// Scan all patches from the pedal.
    pub fn scan_library(&self) -> Result<(), Box<dyn Error>> {
        let mut manager = self.patch_manager.lock().unwrap();
        manager.scan_library()?;
        Ok(())
    }

    /// Get the library.
    pub fn library(&self) -> Option<PatchLibrary> {
        let manager = self.patch_manager.lock().unwrap();
        manager.library().cloned()
    }

    /// Select a patch by slot number.
    pub fn select_patch(&self, slot: u8) -> Result<PatchBuffer, Box<dyn Error>> {
        info!("SELECTING {slot} ");
        let mut manager = self.patch_manager.lock().unwrap();
        let patch = manager.select_patch(slot)?;
        Ok(patch.clone())
    }

    /// Select a patch by name (fuzzy match).
    pub fn select_patch_by_name(&self, name: &str) -> Result<PatchBuffer, Box<dyn Error>> {
        let mut manager = self.patch_manager.lock().unwrap();
        let patch = manager.select_patch_by_name(name)?;
        Ok(patch.clone())
    }

    /// Get the current patch.
    pub fn current_patch(&self) -> Option<PatchBuffer> {
        let manager = self.patch_manager.lock().unwrap();
        manager.current_patch().cloned()
    }

    /// Get the current slot number.
    pub fn current_slot(&self) -> Option<u8> {
        let manager = self.patch_manager.lock().unwrap();
        manager.current_slot()
    }

    /// Refresh the current patch from the pedal.
    pub fn refresh_current(&self) -> Result<(), Box<dyn Error>> {
        let mut manager = self.patch_manager.lock().unwrap();
        manager.refresh_current()
    }

    /// Write the current patch to the edit buffer.
    pub fn write_current_patch(&self) -> Result<(), Box<dyn Error>> {
        let mut manager = self.patch_manager.lock().unwrap();
        manager.write_current()
    }

    /// Store the current patch to a memory slot.
    pub fn store_patch(&self, slot: u8) -> Result<(), Box<dyn Error>> {
        let mut manager = self.patch_manager.lock().unwrap();
        manager.store_patch(slot)
    }

    /// Rename the current patch.
    pub fn rename_current_patch(&self, name: &str) -> Result<(), Box<dyn Error>> {
        let mut manager = self.patch_manager.lock().unwrap();
        manager.rename_current_patch(name)
    }

    /// Rename a patch in the library.
    pub fn rename_patch(&self, slot: u8, name: &str) -> Result<(), Box<dyn Error>> {
        let mut manager = self.patch_manager.lock().unwrap();
        manager.rename_patch(slot, name)
    }

    /// Toggle an effect slot on/off.
    pub fn toggle_slot(&self, slot: usize) -> Result<bool, Box<dyn Error>> {
        let mut manager = self.patch_manager.lock().unwrap();
        manager.toggle_slot(slot)
    }

    /// Select slot to edit
    pub fn select_slot_for_edit(&self, slot: usize) -> Result<bool, Box<dyn Error>> {
        let mut manager = self.patch_manager.lock().unwrap();
        manager.select_slot_for_edit(slot as i32)
    }

    /// Edit a parameter.
    pub fn edit_parameter(
        &self,
        slot: u8,
        param_idx: u8,
        value: u16,
    ) -> Result<(), Box<dyn Error>> {
        let mut manager = self.patch_manager.lock().unwrap();
        manager.edit_parameter(slot, param_idx, value)
    }

    /// Toggle the tuner.
    pub fn toggle_tuner(&self) -> Result<bool, Box<dyn Error>> {
        let mut manager = self.patch_manager.lock().unwrap();
        manager.toggle_tuner()
    }

    /// Get the effect database.
    pub fn effect_db(&self) -> effects::EffectDb {
        let manager = self.patch_manager.lock().unwrap();
        manager.effect_db().clone()
    }

    /// List effects by group.
    pub fn list_effects(&self, group: Option<&str>) -> Vec<effects::EffectDef> {
        let manager = self.patch_manager.lock().unwrap();
        manager
            .list_effects(group)
            .iter()
            .map(|&e| e.clone())
            .collect()
    }

    /// Get a patch by name from the library.
    pub fn get_patch_by_name(&self, name: &str) -> Option<PatchInfo> {
        let manager = self.patch_manager.lock().unwrap();
        manager.get_patch_by_name(name).cloned()
    }

    /// List all patches in the library.
    pub fn list_library(&self) {
        let manager = self.patch_manager.lock().unwrap();
        manager.list_library()
    }

    /// Show current patch details.
    pub fn show_current(&self) -> Result<(), Box<dyn Error>> {
        let mut manager = self.patch_manager.lock().unwrap();
        manager.show_current()
    }

    /// Convenience function to list available MIDI ports.
    pub fn list_midi_ports() -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
        MidiTransport::list_ports()
    }
}

impl Default for ZoomClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Connection status result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Connected,
    AlreadyConnected,
    NoDeviceFound,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = ZoomClient::new();
        assert!(!client.is_connected());
    }

    #[test]
    fn test_connection_status_display() {
        assert_eq!(format!("{:?}", ConnectionStatus::Connected), "Connected");
        assert_eq!(
            format!("{:?}", ConnectionStatus::NoDeviceFound),
            "NoDeviceFound"
        );
    }
}
