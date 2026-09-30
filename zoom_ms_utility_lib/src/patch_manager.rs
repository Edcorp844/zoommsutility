use crate::apatch::Apatch;
use crate::effects::{EffectDb, EffectDef, ParamDisp, build_effect_db};
use crate::midi::MidiTransport;
use crate::mock_trait::MockData;
use crate::model::PedalModel;
use crate::patch_buffer::PatchBuffer;
use crate::patch_info::PatchInfo;
use crate::patch_library::PatchLibrary;
use crate::sysex::SysExCommandBuilder;
use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

/// Manages patches and communication with the Zoom pedal.
pub struct PatchManager {
    transport: MidiTransport,
    model: Option<PedalModel>,
    current_patch: Option<PatchBuffer>,
    current_slot: Option<u8>,
    library: Option<PatchLibrary>,
    effect_db: EffectDb,
    tuner_on: bool,
    is_ready: AtomicBool,
}

impl PatchManager {
    pub fn new() -> Self {
        Self {
            transport: MidiTransport::new(),
            model: None,
            current_patch: None,
            current_slot: None,
            library: None,
            effect_db: build_effect_db(),
            tuner_on: false,
            is_ready: AtomicBool::new(false),
        }
    }

    pub fn transport(&self) -> &MidiTransport {
        &self.transport
    }

    pub fn transport_mut(&mut self) -> &mut MidiTransport {
        &mut self.transport
    }

    pub fn setup_with_transport(&mut self) -> Result<(), Box<dyn Error>> {
        self.auto_detect_model()?;
        self.is_ready.store(true, Ordering::SeqCst);
        Ok(())
    }

    pub fn is_ready(&self) -> bool {
        self.is_ready.load(Ordering::SeqCst)
    }

    pub fn model(&self) -> Option<PedalModel> {
        self.model
    }

    pub fn auto_detect_model(&mut self) -> Result<(), Box<dyn Error>> {
        info!("Sending identity request to detect pedal model...");

        self.flush();
        self.transport
            .send(&SysExCommandBuilder::identity_request())?;

        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(1) {
            if let Some(msg) = self.transport.recv() {
                if let Some(model) = PedalModel::from_identity_reply(&msg) {
                    info!("Detected pedal model: {}", model);
                    self.model = Some(model);
                    return Ok(());
                }
            }
            std::thread::sleep(Duration::from_millis(10));
        }

        info!("Identity request timed out: no MIDI response received");
        self.model = None;
        Err("Failed to detect pedal: identity request timed out".into())
    }

    pub fn connect_to_port(&mut self, port_name: &str) -> Result<(), Box<dyn Error>> {
        self.is_ready.store(false, Ordering::SeqCst);
        self.transport.connect_to_port(port_name)?;
        self.setup_with_transport()?;
        Ok(())
    }

    pub fn flush(&mut self) {
        while self.transport.recv().is_some() {}
    }

    pub fn send(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        self.transport.send(data)?;
        Ok(())
    }

    pub fn recv(&mut self) -> Option<Vec<u8>> {
        self.transport.recv()
    }

    /// Send a command and wait for a response
    pub fn send_and_wait<F>(
        &mut self,
        request: &[u8],
        timeout_ms: u64,
        response_matcher: F,
    ) -> Result<Vec<u8>, Box<dyn Error>>
    where
        F: Fn(&[u8]) -> bool,
    {
        self.flush();
        self.send(request)?;

        let start = Instant::now();
        let mut buf = Vec::new();
        let timeout = Duration::from_millis(timeout_ms);

        while start.elapsed() < timeout {
            if let Some(msg) = self.recv() {
                buf.extend_from_slice(&msg);

                let mut i = 0;
                while i < buf.len() {
                    if buf[i] == 0xF0 {
                        if let Some(end) = buf[i..].iter().position(|&b| b == 0xF7) {
                            let frame = &buf[i..=i + end];
                            if response_matcher(frame) {
                                return Ok(frame.to_vec());
                            }
                            i += end + 1;
                            continue;
                        }
                    }
                    i += 1;
                }

                if buf.len() > 1024 {
                    buf = buf[buf.len() - 1024..].to_vec();
                }
            }
            std::thread::sleep(Duration::from_millis(2));
        }

        Err("Timeout waiting for response".into())
    }

    /// Check if a message is patch data (0x28 response)
    pub fn is_patch_data(msg: &[u8]) -> bool {
        msg.len() >= 6 && msg[0] == 0xF0 && msg[1] == 0x52 && msg[4] == 0x28
    }

    /// Extract patch name from raw data
    fn extract_name_from_data(data: &[u8]) -> String {
        let len = data.len();
        let base = if len >= 146 { 132 } else { 91 };
        let mut name = String::new();

        if len > base + 13 {
            for j in 0..13 {
                let idx = base + j;
                if idx < len {
                    let c = data[idx];
                    if c != 0 && c >= 0x20 && c <= 0x7E {
                        name.push(c as char);
                    } else if c == 0 {
                        break;
                    }
                }
            }
        }
        name.trim_end_matches(' ').to_string()
    }

    /// Read a patch from raw bytes
    fn read_patch_from_bytes(bytes: &[u8]) -> Result<Apatch, String> {
        if bytes.len() < 10 {
            return Err(format!("Data too short: {} bytes", bytes.len()));
        }

        if bytes[0] != 0xF0 || bytes[1] != 0x52 {
            return Err(format!(
                "Invalid SysEx frame: {:02X} {:02X}",
                bytes[0], bytes[1]
            ));
        }

        let mut patch = Apatch::new();
        patch.read_bin(bytes);

        if patch.name.is_empty() {
            patch.name = Self::extract_name_from_data(bytes);
        }

        if patch.name.is_empty() {
            patch.name = "Unnamed".to_string();
        }

        Ok(patch)
    }

    /// Scan library using sequential request sequence
    pub fn scan_library(&mut self) -> Result<&PatchLibrary, Box<dyn Error>> {
        if self.model.is_none() {
            return Err("Pedal model not set".into());
            // let lib = PatchLibrary::mock();
            // self.library = Some(lib);
            // return Ok(self.library.as_ref().unwrap());
        }

        let model = self.model.unwrap();
        let max_patches = model.max_patches();
        let mut lib = PatchLibrary::new(model);

        info!("Starting scan of {} patches", max_patches);

        // Enable parameter edit mode
        info!("Enabling parameter edit mode...");
        self.flush();

        let enable_cmd = [0xF0, 0x52, 0x00, model.device_id(), 0x50, 0xF7];
        self.send(&enable_cmd)?;
        std::thread::sleep(Duration::from_millis(50));

        // Send program request to query active hardware slot index
        let request_prog = [0xF0, 0x52, 0x00, model.device_id(), 0x33, 0xF7];
        self.send(&request_prog)?;
        std::thread::sleep(Duration::from_millis(50));

        // Capture initial hardware program change byte if available
        let initial_hw_slot = if let Some(msg) = self.recv() {
            if msg.len() >= 2 && msg[0] == 0xC0 {
                Some(msg[1])
            } else {
                None
            }
        } else {
            None
        };

        let mut scan_counter = 0u8;
        let mut responses: HashMap<u8, Vec<u8>> = HashMap::new();

        // Send initial request
        info!("Sending patch requests...");
        self.send(&[0xC0, scan_counter])?;
        std::thread::sleep(Duration::from_millis(2));
        let request = [0xF0, 0x52, 0x00, model.device_id(), 0x29, 0xF7];
        self.send(&request)?;
        std::thread::sleep(Duration::from_millis(2));

        let start = Instant::now();
        let timeout = Duration::from_secs(3);
        let mut received = 0;

        info!("Collecting responses...");
        while start.elapsed() < timeout && received < max_patches {
            if let Some(msg) = self.recv() {
                let mut i = 0;
                while i < msg.len() {
                    if msg[i] == 0xF0 {
                        if let Some(end) = msg[i..].iter().position(|&b| b == 0xF7) {
                            let frame = &msg[i..=i + end];

                            if Self::is_patch_data(frame) {
                                info!("Received response for patch {}", scan_counter);
                                responses.insert(scan_counter, frame.to_vec());
                                received += 1;
                                print!("\rReceived {}/{} patches...", received, max_patches);
                                std::io::stdout().flush()?;

                                scan_counter += 1;
                                if scan_counter < max_patches as u8 {
                                    self.send(&[0xC0, scan_counter])?;
                                    std::thread::sleep(Duration::from_millis(1));
                                    let next_request =
                                        [0xF0, 0x52, 0x00, model.device_id(), 0x29, 0xF7];
                                    self.send(&next_request)?;
                                    std::thread::sleep(Duration::from_millis(1));
                                }
                            }
                            i += end + 1;
                            continue;
                        }
                    }
                    i += 1;
                }
            }
            std::thread::sleep(Duration::from_millis(1));
        }

        println!("\r✓ Received {}/{} patch responses", received, max_patches);
        info!("Collected {} responses", received);

        // Process frame data into library models
        for (num, frame) in responses {
            info!("Processing patch {}...", num);
            match Self::read_patch_from_bytes(&frame) {
                Ok(mut patch) => {
                    if patch.name.is_empty() {
                        patch.name = format!("Patch {}", num);
                    }
                    info!("Patch {}: '{}'", num, patch.name);

                    let active_count = patch
                        .fx
                        .iter()
                        .filter(|&fx| fx[0] != 0 && fx[1] != 0)
                        .count();

                    let buffer = PatchBuffer {
                        name: patch.name.clone(),
                        data: patch,
                        slot: Some(num),
                    };

                    let info_struct = PatchInfo {
                        number: num,
                        name: buffer.name.clone(),
                        buffer: buffer.clone(),
                        active_slots: active_count,
                        effect_ids: {
                            let mut ids = [0u32; 6];
                            for i in 0..6 {
                                ids[i] = buffer.data.fx[i][1] as u32;
                            }
                            ids
                        },
                        estimated_dsp: 0.0,
                        warnings: Vec::new(),
                        errors: Vec::new(),
                        is_empty: active_count == 0,
                    };
                    lib.patches[num as usize] = Some(info_struct);
                }
                Err(e) => {
                    error!("Failed to parse patch {}: {}", num, e);
                    let empty_patch = PatchInfo {
                        number: num,
                        name: "<empty>".to_string(),
                        buffer: PatchBuffer {
                            name: "".to_string(),
                            data: Apatch::new(),
                            slot: Some(num),
                        },
                        active_slots: 0,
                        effect_ids: [0; 6],
                        estimated_dsp: 0.0,
                        warnings: vec![],
                        errors: vec![e.to_string()],
                        is_empty: true,
                    };
                    lib.patches[num as usize] = Some(empty_patch);
                }
            }
        }

        // Fill missing patch slots
        for n in 0..max_patches {
            if lib.patches[n].is_none() {
                info!("Patch {}: no response received", n);
                let empty_patch = PatchInfo {
                    number: n as u8,
                    name: "<empty>".to_string(),
                    buffer: PatchBuffer {
                        name: "".to_string(),
                        data: Apatch::new(),
                        slot: Some(n as u8),
                    },
                    active_slots: 0,
                    effect_ids: [0; 6],
                    estimated_dsp: 0.0,
                    warnings: vec![],
                    errors: vec!["No response".to_string()],
                    is_empty: true,
                };
                lib.patches[n] = Some(empty_patch);
            }
        }

        let count = lib.count();
        println!("✓ Scanned {} patches", count);
        info!("Scan complete: {} patches", count);

        self.library = Some(lib.clone());

        // Restore or select active hardware patch slot instead of forcing slot 0
        let target_slot = initial_hw_slot.unwrap_or(0);
        if let Some(Some(info)) = lib.patches.get(target_slot as usize) {
            info!(
                "Aligning active patch state to slot {}: '{}'",
                target_slot, info.name
            );
            self.current_patch = Some(info.buffer.clone());
            self.current_slot = Some(target_slot);

            // Return hardware back to target slot program state
            let _ = self.send(&[0xC0, target_slot]);
        } else {
            self.current_slot = Some(0);
            if let Some(info) = lib.get(0) {
                self.current_patch = Some(info.buffer.clone());
            }
        }

        // Disable parameter edit mode
        let disable_cmd = [0xF0, 0x52, 0x00, model.device_id(), 0x51, 0xF7];
        let _ = self.send(&disable_cmd);

        // Fetch live parameter buffer to guarantee state alignment
        let _ = self.fetch_current();

        Ok(self.library.as_ref().unwrap())
    }

    pub fn fetch_current(&mut self) -> Result<&PatchBuffer, Box<dyn Error>> {
        if self.model.is_none() {
            return Err("Pedal model not set".into());
        }

        let model = self.model.unwrap();

        // Enable parameter edit mode first
        let enable_cmd = [0xF0, 0x52, 0x00, model.device_id(), 0x50, 0xF7];
        self.send(&enable_cmd)?;
        std::thread::sleep(Duration::from_millis(20));

        let request = [0xF0, 0x52, 0x00, model.device_id(), 0x29, 0xF7];
        let frame = self.send_and_wait(&request, 500, Self::is_patch_data)?;

        match Self::read_patch_from_bytes(&frame) {
            Ok(patch) => {
                info!("Fetched patch: '{}'", patch.name);
                let buffer = PatchBuffer {
                    name: patch.name.clone(),
                    data: patch,
                    slot: self.current_slot,
                };
                self.current_patch = Some(buffer);
                Ok(self.current_patch.as_ref().unwrap())
            }
            Err(e) => Err(format!("Error parsing patch data: {}", e).into()),
        }
    }

    pub fn select_patch(&mut self, number: u8) -> Result<&PatchBuffer, Box<dyn Error>> {
        let lib = self
            .library
            .as_ref()
            .ok_or("Library not scanned. Use scan_library first.")?;

        if !lib.is_valid_slot(number) {
            return Err(format!(
                "Patch number must be 0-{}, got {}",
                lib.max_patches() - 1,
                number
            )
            .into());
        }

        if let Some(info) = lib.get(number) {
            info!("Selecting patch {}: '{}'", number, info.name);
            self.current_patch = Some(info.buffer.clone());
            self.current_slot = Some(number);

            self.send(&[0xC0, number])?;
            std::thread::sleep(Duration::from_millis(100));

            if let Err(e) = self.fetch_current() {
                info!("Could not verify selection: {}", e);
            }

            return Ok(self.current_patch.as_ref().unwrap());
        }

        Err(format!("Patch {} not found", number).into())
    }

    pub fn select_patch_by_name(&mut self, name: &str) -> Result<&PatchBuffer, Box<dyn Error>> {
        let lib = self
            .library
            .as_ref()
            .ok_or("Library not scanned. Use scan_library first.")?;

        let name_lower = name.to_lowercase();
        for (i, slot) in lib.patches.iter().enumerate() {
            if let Some(info) = slot {
                if info.name.to_lowercase().contains(&name_lower) {
                    return self.select_patch(i as u8);
                }
            }
        }

        Err(format!("No patch found with name containing '{}'", name).into())
    }

    pub fn write_current(&mut self) -> Result<(), Box<dyn Error>> {
        if self.model.is_none() {
            return Err("Pedal model not set".into());
        }
        if let Some(patch) = &self.current_patch {
            info!("Writing current patch '{}' to pedal", patch.name);
            let effect_list: Vec<EffectDef> = self.effect_db.values().cloned().collect();
            let data = patch
                .data
                .make_bin(self.model.unwrap().device_id(), &effect_list);
            self.send(&data)?;
            std::thread::sleep(Duration::from_millis(50));
            Ok(())
        } else {
            Err("No current patch to write".into())
        }
    }

    pub fn store_patch(&mut self, number: u8) -> Result<(), Box<dyn Error>> {
        if self.model.is_none() {
            return Err("Pedal model not set".into());
        }

        let lib = self
            .library
            .as_ref()
            .ok_or("Library not scanned. Use scan_library first.")?;

        if !lib.is_valid_slot(number) {
            return Err(format!(
                "Patch number must be 0-{}, got {}",
                lib.max_patches() - 1,
                number
            )
            .into());
        }

        info!("Storing current patch to slot {}", number);
        self.write_current()?;
        self.send(&SysExCommandBuilder::store_patch(
            self.model.unwrap(),
            number,
        ))?;
        std::thread::sleep(Duration::from_millis(100));

        //self.scan_library()?;
        Ok(())
    }

    pub fn save_patch(&mut self, patch_idx: u8) -> Result<(), Box<dyn Error>> {
        if self.model.is_none() {
            return Err("Pedal model not set".into());
        }

        info!("Saving active patch to slot {}", patch_idx);
        self.send(&SysExCommandBuilder::save_current_patch(
            self.model.unwrap(),
            patch_idx,
        ))?;
        std::thread::sleep(Duration::from_millis(50));
        Ok(())
    }

    pub fn toggle_tuner(&mut self) -> Result<bool, Box<dyn Error>> {
        if self.model.is_none() {
            return Err("Pedal model not set".into());
        }

        self.tuner_on = !self.tuner_on;
        info!("Tuner: {}", if self.tuner_on { "ON" } else { "OFF" });
        self.send(&SysExCommandBuilder::tuner_control(
            self.model.unwrap(),
            self.tuner_on,
        ))?;
        Ok(self.tuner_on)
    }

    pub fn select_slot_for_edit(&mut self, slot: i32) -> Result<bool, Box<dyn Error>> {
        if self.model.is_none() {
            return Err("Pedal model not set".into());
        }

        if !(0..6).contains(&slot) {
            return Err(format!("Slot index out of range (0-5 required), got {}", slot).into());
        }

        let model = self.model.unwrap();
        info!("Selecting slot {} for editing on {}", slot, model);

        // Send the actual hardware slot target command. The pedal only edits the
        // currently selected effect slot when this command is active.
        self.send(&SysExCommandBuilder::select_slot_for_edit(
            model, slot as u8,
        ))?;
        std::thread::sleep(Duration::from_millis(20));

        if let Some(patch) = &mut self.current_patch {
            patch.data.curfx = slot;
        }

        // Keep the current patch in sync with the pedal's active effect slot so
        // subsequent payload writes target the same slot without corrupting other
        // preset slots.
        self.write_current()?;
        self.current_slot = Some(slot as u8);
        std::thread::sleep(Duration::from_millis(10));

        Ok(true)
    }

    pub fn edit_parameter(
        &mut self,
        slot: u8,
        param_idx: u8, // hardware pp from the UI
        value: u16,
    ) -> Result<(), Box<dyn Error>> {
        if self.model.is_none() {
            return Err("Pedal model not set".into());
        }
        if slot >= 6 || param_idx > 10 {
            return Err("Slot or Param index out of bounds".into());
        }

        let model = self.model.unwrap();

        // Convert hardware pp → 0-based index into EffectDef.params
        let storage_idx = if param_idx >= 2 {
            (param_idx - 2) as usize
        } else {
            0
        };

        info!(
            "[edit_parameter] slot={} hw_pp={} storage_idx={} value={}",
            slot, param_idx, storage_idx, value
        );

        // 1. Always update the local buffer
        if let Some(patch) = &mut self.current_patch {
            patch.data.curfx = slot as i32;
            patch.set_slot_param(slot as usize, storage_idx, value);
            info!("[edit_parameter] local buffer updated");
        } else {
            warn!("[edit_parameter] no current_patch – cannot update buffer");
        }

        // 2. Focus the slot
        self.send(&SysExCommandBuilder::select_slot_for_edit(model, slot))?;
        std::thread::sleep(Duration::from_millis(15));

        // 3. Fast Parameter-Edit (works for slots 0-2, first page)
        let param_edit_result = self.send(&SysExCommandBuilder::parameter_edit(
            model, slot, param_idx, value,
        ));
        info!(
            "[edit_parameter] parameter_edit result: {:?}",
            param_edit_result.is_ok()
        );

        // 4. Full patch write when needed
        //    - any parameter beyond the first page (pp > 4)
        //    - OR any slot beyond the first three (ParamEdit is unreliable there)
        let needs_full_write = param_idx > 4 || slot >= 3;

        if needs_full_write {
            info!(
                "[edit_parameter] full patch write (slot={}, pp={})",
                slot, param_idx
            );
            match self.write_current() {
                Ok(()) => info!("[edit_parameter] write_current OK"),
                Err(e) => {
                    error!("[edit_parameter] write_current FAILED: {}", e);
                    return Err(e);
                }
            }
        }

        Ok(())
    }

    pub fn toggle_slot(&mut self, slot: usize) -> Result<bool, Box<dyn Error>> {
        if slot >= 6 {
            return Err(format!("Slot must be 0-5, got {}", slot).into());
        }

        self.fetch_current()?;

        if let Some(patch) = &mut self.current_patch {
            let current_state = patch.is_slot_enabled(slot).unwrap_or(false);
            let new_state = !current_state;
            info!("Toggling slot {}: {} -> {}", slot, current_state, new_state);
            patch.set_slot_enabled(slot, new_state);
            self.edit_parameter(slot as u8, 0, if new_state { 1 } else { 0 })?;
            Ok(new_state)
        } else {
            Err("No current patch loaded".into())
        }
    }

    pub fn rename_current_patch(&mut self, new_name: &str) -> Result<(), Box<dyn Error>> {
        let slot = self.current_slot.ok_or("No current slot selected")?;
        self.rename_patch(slot, new_name)
    }

    pub fn rename_patch(&mut self, number: u8, new_name: &str) -> Result<(), Box<dyn Error>> {
        let lib_clone = self.library.clone();
        let lib = lib_clone
            .as_ref()
            .ok_or("Library not scanned. Use scan_library first.")?;

        if !lib.is_valid_slot(number) {
            return Err(format!(
                "Patch number must be 0-{}, got {}",
                lib.max_patches() - 1,
                number
            )
            .into());
        }

        if lib.get(number).is_none() {
            return Err(format!("Patch {} not found", number).into());
        }

        let truncated_name: String = new_name.chars().take(10).collect();
        info!("Renaming patch {} to '{}'", number, truncated_name);

        self.select_patch(number)?;

        if let Some(patch) = &mut self.current_patch {
            patch.name = truncated_name.clone();
            patch.data.name = truncated_name.clone();
        }

        self.write_current()?;
        self.store_patch(number)?;

        Ok(())
    }

    pub fn refresh_current(&mut self) -> Result<(), Box<dyn Error>> {
        info!("Refreshing current patch");
        self.fetch_current()?;
        Ok(())
    }

    pub fn current_patch(&self) -> Option<&PatchBuffer> {
        self.current_patch.as_ref()
    }

    pub fn current_slot(&self) -> Option<u8> {
        self.current_slot
    }

    pub fn library(&self) -> Option<&PatchLibrary> {
        self.library.as_ref()
    }

    pub fn effect_db(&self) -> &EffectDb {
        &self.effect_db
    }

    pub fn get_patch_by_name(&self, name: &str) -> Option<&PatchInfo> {
        let lib = self.library.as_ref()?;
        lib.find_by_name(name)
    }

    pub fn list_effects(&self, group: Option<&str>) -> Vec<&EffectDef> {
        let mut effects: Vec<_> = self.effect_db.values().collect();
        if let Some(group) = group {
            effects.retain(|e| e.group == group);
        }
        effects.sort_by_key(|e| (e.group.clone(), e.order));
        effects
    }

    pub fn list_library(&self) {
        let lib = match &self.library {
            Some(l) => l,
            None => {
                println!("Library not scanned. Use scan_library first.");
                return;
            }
        };

        println!("\n=== Library Summary ===");
        if lib.count() == 0 {
            println!("No patches found in library.");
            return;
        }

        for (i, slot) in lib.patches.iter().enumerate() {
            if let Some(info) = slot {
                let selected = if self.current_slot == Some(i as u8) {
                    " *"
                } else {
                    ""
                };
                println!(
                    "  {:02}: {} ({} effects){}",
                    i, info.name, info.active_slots, selected
                );
            } else {
                println!("  {:02}: <empty>", i);
            }
        }
        println!("Total: {} patches", lib.count());
        if let Some(slot) = self.current_slot {
            println!("Currently selected: slot {}", slot);
        }
    }

    pub fn show_current(&mut self) -> Result<(), Box<dyn Error>> {
        self.fetch_current()?;
        if let Some(patch) = &self.current_patch {
            println!("\n{}", self.format_patch(patch));
        }
        Ok(())
    }

    pub fn format_patch(&self, patch: &PatchBuffer) -> String {
        let mut result = String::new();
        let slot_info = if let Some(slot) = self.current_slot {
            format!(" (Slot {})", slot)
        } else {
            String::new()
        };

        result.push_str(&format!("Patch: {}{}\n", patch.name, slot_info));
        result.push_str(&format!(
            "Effects: {}/{}\n",
            patch.data.curfx + 1,
            patch.data.maxfx
        ));
        result.push_str("  Slots:\n");

        for slot in 0..6 {
            let fx_slot = &patch.data.fx[slot];
            let state = fx_slot[0];
            let effect_id = fx_slot[1] as u32;
            let status = if state != 0 { "ON" } else { "OFF" };

            let (name, title) = if let Some(eff) = self.effect_db.get(&effect_id) {
                (eff.name.clone(), eff.title.clone())
            } else if effect_id == 0 {
                ("THRU".to_string(), "".to_string())
            } else {
                (format!("Unknown(0x{:08X})", effect_id), "".to_string())
            };

            result.push_str(&format!(
                "    Slot {}: {} [{}] - {}\n",
                slot + 1,
                name,
                status,
                title
            ));

            if let Some(eff) = self.effect_db.get(&effect_id) {
                for (i, param_def) in eff.params.iter().enumerate() {
                    let raw_val = fx_slot[i + 2];
                    let display = Self::format_param_value(raw_val, param_def);
                    result.push_str(&format!("        {}: {}\n", param_def.name, display));
                }
            }
        }

        result
    }

    fn format_param_value(value: i32, param_def: &crate::effects::ParamDef) -> String {
        match &param_def.disp {
            ParamDisp::None => value.to_string(),
            ParamDisp::Offset(offset) => (value as i16 + offset).to_string(),
            ParamDisp::Labels(labels) => {
                let idx = value as usize;
                if idx < labels.len() {
                    labels[idx].clone()
                } else {
                    value.to_string()
                }
            }
            ParamDisp::Time { min, max, list } => {
                if value >= *min && value as usize <= *max as usize {
                    let idx = (value - *min) as usize;
                    if idx < list.len() {
                        return list[idx].clone();
                    }
                }
                format!("{}ms", value)
            }
        }
    }
}
