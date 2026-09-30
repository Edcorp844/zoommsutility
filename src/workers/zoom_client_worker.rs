/* workers/zoom_client_worker.rs
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

use relm4::{ComponentSender, Worker};
use std::thread;
use std::time::Duration;
use zoom_ms::{model::PedalModel, patch_buffer::PatchBuffer, ZoomClient, ZoomMidiEvent};

pub struct ZoomClientWorker {
    client: ZoomClient,
    known_ports: Vec<String>,
    current_patch_number: Option<u8>,
}

#[derive(Debug, Clone)]
pub enum ZoomClientWorkerInput {
    AutoDetectDevice,
    SelectPort(String),
    RenamePatch(u8, String),
    SelectPatch(u8),
    StorePatch(u8),
    WriteCurrentPatch,
    FetchMidiPorts,
    CheckDeviceState,
    ScanLibrary,
    SelectSlot(usize),

    /// UI -> Worker: App UI controls turned by user (Edits temporary buffer)
    UiParamEdit {
        slot: usize,
        param_idx: usize,
        val: f64,
    },
}

#[derive(Debug, Clone)]
pub enum ZoomClientWorkerOutput {
    MidiPortsList(Vec<String>),
    DeviceStatusUpdated {
        port_name: Option<String>,
        pedal_model: Option<PedalModel>,
    },
    ScanProgress(f64),
    PatchesScanned(Vec<String>),
    NotifyUiError(String),
    RenamedPatch(u8, String),
    SelectedPatch(PatchBuffer, u8),
    SelectedSlotForEdit(bool),

    /// Worker -> App: Physical hardware knob turned on pedal (Syncs UI component state)
    HardwareParamUpdated {
        slot: usize,
        param_idx: usize,
        val: f64,
    },
    HardwarePatchChanged {
        slot: u8,
        patch_buffer: PatchBuffer,
    },
    HardwareSlotToggled {
        slot: usize,
        enabled: bool,
    },
}

impl ZoomClientWorker {
    fn restore_hardware_state(&mut self, sender: &ComponentSender<Self>) {
        if let Some(slot) = self.current_patch_number {
            match self.client.select_patch(slot) {
                Ok(patch_buffer) => {
                    let _ =
                        sender.output(ZoomClientWorkerOutput::SelectedPatch(patch_buffer, slot));
                }
                Err(err) => {
                    let _ = sender.output(ZoomClientWorkerOutput::NotifyUiError(format!(
                        "Failed to restore patch at slot {}: {}",
                        slot, err
                    )));
                }
            }
        }
    }

    /// Spawns background task reading physical hardware messages -> sends HardwareParamUpdated to App
    fn spawn_hardware_listener(&mut self, sender: ComponentSender<Self>) {
        if let Some(rx) = self.client.take_midi_receiver() {
            tokio::task::spawn_blocking(move || {
                while let Ok(event) = rx.recv() {
                    match event {
                        ZoomMidiEvent::ParamEdit {
                            slot,
                            param_idx,
                            value,
                        } => {
                            let _ = sender.output(ZoomClientWorkerOutput::HardwareParamUpdated {
                                slot: slot as usize,
                                param_idx: param_idx as usize,
                                val: value as f64,
                            });
                        }
                        ZoomMidiEvent::PatchChange { slot, patch_buffer } => {
                            let _ = sender.output(ZoomClientWorkerOutput::HardwarePatchChanged {
                                slot,
                                patch_buffer,
                            });
                        }
                        ZoomMidiEvent::SlotStateChanged { slot, enabled } => {
                            let _ = sender.output(ZoomClientWorkerOutput::HardwareSlotToggled {
                                slot: slot as usize,
                                enabled,
                            });
                        }
                        ZoomMidiEvent::RawSysEx(_) => {}
                    }
                }
            });
        }
    }
}

impl Worker for ZoomClientWorker {
    type Init = ();
    type Input = ZoomClientWorkerInput;
    type Output = ZoomClientWorkerOutput;

    fn init(_init: Self::Init, sender: ComponentSender<Self>) -> Self {
        let sender_clone = sender.clone();

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_millis(2000)).await;
                sender_clone.input(ZoomClientWorkerInput::CheckDeviceState);
            }
        });

        Self {
            client: ZoomClient::new(),
            known_ports: Vec::new(),
            current_patch_number: None,
        }
    }

    fn update(&mut self, input: Self::Input, sender: ComponentSender<Self>) {
        match input {
            ZoomClientWorkerInput::AutoDetectDevice => match ZoomClient::list_midi_ports() {
                Ok((input_ports, _output_ports)) => {
                    self.known_ports = input_ports.clone();
                    let _ =
                        sender.output(ZoomClientWorkerOutput::MidiPortsList(input_ports.clone()));

                    match self.client.connect() {
                        Ok(_) => {
                            let model = self.client.model();
                            let _ = sender.output(ZoomClientWorkerOutput::DeviceStatusUpdated {
                                port_name: input_ports.first().cloned(),
                                pedal_model: model,
                            });
                            self.spawn_hardware_listener(sender.clone());
                            self.restore_hardware_state(&sender);
                        }
                        Err(_) => {
                            let _ = sender.output(ZoomClientWorkerOutput::DeviceStatusUpdated {
                                port_name: None,
                                pedal_model: None,
                            });
                        }
                    }
                }
                Err(e) => {
                    let _ = sender.output(ZoomClientWorkerOutput::DeviceStatusUpdated {
                        port_name: None,
                        pedal_model: None,
                    });
                    let _ = sender.output(ZoomClientWorkerOutput::NotifyUiError(e.to_string()));
                }
            },

            ZoomClientWorkerInput::SelectPort(port_name) => {
                match self.client.connect_to_port(port_name.clone()) {
                    Ok(_) => {
                        let model = self.client.model();

                        let _ = sender.output(ZoomClientWorkerOutput::DeviceStatusUpdated {
                            port_name: Some(port_name),
                            pedal_model: model,
                        });
                        self.spawn_hardware_listener(sender.clone());
                        self.restore_hardware_state(&sender);
                    }
                    Err(e) => {
                        let _ = sender.output(ZoomClientWorkerOutput::DeviceStatusUpdated {
                            port_name: Some(port_name),
                            pedal_model: None,
                        });
                        let _ = sender.output(ZoomClientWorkerOutput::NotifyUiError(e.to_string()));
                    }
                }
            }

            ZoomClientWorkerInput::FetchMidiPorts => {
                if let Ok((input_ports, _)) = ZoomClient::list_midi_ports() {
                    self.known_ports = input_ports.clone();
                    let _ = sender.output(ZoomClientWorkerOutput::MidiPortsList(input_ports));
                }
            }

            ZoomClientWorkerInput::CheckDeviceState => {
                let current_ports = match ZoomClient::list_midi_ports() {
                    Ok((input_ports, _)) => input_ports,
                    Err(_) => Vec::new(),
                };

                if current_ports != self.known_ports {
                    let device_disconnected = self.known_ports.len() > current_ports.len();
                    self.known_ports = current_ports.clone();
                    let _ =
                        sender.output(ZoomClientWorkerOutput::MidiPortsList(current_ports.clone()));

                    if device_disconnected && !self.client.is_connected() {
                        let _ = sender.output(ZoomClientWorkerOutput::DeviceStatusUpdated {
                            port_name: None,
                            pedal_model: None,
                        });
                    }
                }
            }

            ZoomClientWorkerInput::ScanLibrary => {
                
                if !self.client.is_connected() {
                    let _ = sender.output(ZoomClientWorkerOutput::NotifyUiError(
                        "Cannot scan: No active Zoom client connected.".to_string(),
                    ));
                    let _ = sender.output(ZoomClientWorkerOutput::PatchesScanned(Vec::new()));
                    return;
                }

                if let Err(e) = self.client.scan_library() {
                    let _ = sender.output(ZoomClientWorkerOutput::NotifyUiError(format!(
                        "Scan failed: {}",
                        e
                    )));
                    let _ = sender.output(ZoomClientWorkerOutput::PatchesScanned(Vec::new()));
                    return;
                }

                let total_patches = self.client.model().map(|m| m.max_patches()).unwrap_or(50);
                let mut patches = Vec::with_capacity(total_patches);

                if let Some(library) = self.client.library() {
                    for i in 0..total_patches {
                        let progress = (i as f64) / (total_patches as f64);
                        let _ = sender.output(ZoomClientWorkerOutput::ScanProgress(progress));

                        if let Some(patch) = library.get(i as u8) {
                            patches.push(patch.name.clone());
                        } else {
                            patches.push(format!("{:02}: [Empty / Error]", i + 1));
                        }

                        thread::sleep(Duration::from_millis(15));
                    }
                }

                let _ = sender.output(ZoomClientWorkerOutput::ScanProgress(1.0));
                let _ = sender.output(ZoomClientWorkerOutput::PatchesScanned(patches));

                let target_slot = self.current_patch_number.unwrap_or(0);
                match self.client.select_patch(target_slot) {
                    Ok(patch_buffer) => {
                        let _ = sender.output(ZoomClientWorkerOutput::SelectedPatch(
                            patch_buffer,
                            target_slot,
                        ));
                    }
                    Err(err) => {
                        let _ = sender.output(ZoomClientWorkerOutput::NotifyUiError(format!(
                            "Failed to select patch at slot {}: {}",
                            target_slot, err
                        )));
                    }
                }
            }

            ZoomClientWorkerInput::RenamePatch(slot, name) => {
                match self.client.rename_patch(slot, &name) {
                    Ok(_) => {
                        let _ = sender.output(ZoomClientWorkerOutput::RenamedPatch(slot, name));
                    }
                    Err(error) => {
                        let _ = sender.output(ZoomClientWorkerOutput::NotifyUiError(format!(
                            "Rename Patch Error: {}",
                            error
                        )));
                    }
                }
            }

            ZoomClientWorkerInput::SelectPatch(slot) => {
                self.current_patch_number = Some(slot);

                match self.client.select_patch(slot) {
                    Ok(patch_buffer) => {
                        let _ = sender
                            .output(ZoomClientWorkerOutput::SelectedPatch(patch_buffer, slot));
                    }
                    Err(err) => {
                        let _ = sender.output(ZoomClientWorkerOutput::NotifyUiError(format!(
                            "Failed to select patch at slot {}: {}",
                            slot, err
                        )));
                    }
                }
            }

            ZoomClientWorkerInput::StorePatch(slot) => match self.client.store_patch(slot) {
                Ok(_) => {
                    self.current_patch_number = Some(slot);
                }
                Err(error) => {
                    let _ = sender.output(ZoomClientWorkerOutput::NotifyUiError(format!(
                        "Store Patch Error: {}",
                        error
                    )));
                }
            },

            ZoomClientWorkerInput::WriteCurrentPatch => {
                if let Err(error) = self.client.write_current_patch() {
                    let _ = sender.output(ZoomClientWorkerOutput::NotifyUiError(format!(
                        "Error Writing Current Patch: {}",
                        error
                    )));
                }
            }

            ZoomClientWorkerInput::SelectSlot(slot) => {
                match self.client.select_slot_for_edit(slot) {
                    Ok(val) => {
                        let _ = sender.output(ZoomClientWorkerOutput::SelectedSlotForEdit(val));
                    }
                    Err(error) => {
                        let _ = sender.output(ZoomClientWorkerOutput::NotifyUiError(format!(
                            "Error Selecting Slot: {}",
                            error
                        )));
                    }
                }
            }

            ZoomClientWorkerInput::UiParamEdit {
                slot,
                param_idx,
                val,
            } => {
                if let Err(error) =
                    self.client
                        .edit_parameter(slot as u8, param_idx as u8, val as u16)
                {
                    let _ = sender.output(ZoomClientWorkerOutput::NotifyUiError(format!(
                        "Error Editing Parameter: {}",
                        error
                    )));
                }
            }
        }
    }
}
