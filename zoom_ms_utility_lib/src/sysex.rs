use crate::model::PedalModel;

pub struct SysExCommandBuilder;

impl SysExCommandBuilder {
    pub const SYSEX_START: u8 = 0xF0;
    pub const SYSEX_END: u8 = 0xF7;

    /// Universal Identity Request
    pub fn identity_request() -> Vec<u8> {
        vec![Self::SYSEX_START, 0x7E, 0x7F, 0x06, 0x01, Self::SYSEX_END]
    }

    /// Request the current edit-buffer patch (command 0x29)
    pub fn request_current_patch(model: PedalModel) -> Vec<u8> {
        vec![
            Self::SYSEX_START,
            0x52,
            0x00,
            model.device_id(),
            0x29,
            Self::SYSEX_END,
        ]
    }

    /// Request a memory slot patch using 0x09 command (for reading stored patches)
    /// Response will be 0x08 with 7-bit encoded data
    pub fn request_memory_slot(model: PedalModel, number: u8) -> Vec<u8> {
        vec![
            Self::SYSEX_START,
            0x52,
            0x00,
            model.device_id(),
            0x09,
            0x00,
            0x00,
            number,
            Self::SYSEX_END,
        ]
    }

    /// Request data dump (command 0x28) - for edit buffer
    pub fn data_request(model: PedalModel, number: u8) -> Vec<u8> {
        vec![
            Self::SYSEX_START,
            0x52,
            0x00,
            model.device_id(),
            0x28,
            0x00,
            number,
            Self::SYSEX_END,
        ]
    }

    /// Enable parameter editing / editor mode (command 0x50)
    pub fn editor_on(model: PedalModel) -> Vec<u8> {
        vec![
            Self::SYSEX_START,
            0x52,
            0x00,
            model.device_id(),
            0x50,
            Self::SYSEX_END,
        ]
    }

    /// Disable parameter editing (command 0x51)
    pub fn editor_off(model: PedalModel) -> Vec<u8> {
        vec![
            Self::SYSEX_START,
            0x52,
            0x00,
            model.device_id(),
            0x51,
            Self::SYSEX_END,
        ]
    }

    ///Request editor mode for slot
    pub fn select_slot_for_edit(model: PedalModel, slot: u8) -> Vec<u8> {
        vec![
            Self::SYSEX_START,
            0x52,
            0x00,
            model.device_id(),
            0x50, // Parameter Edit Mode / Active Slot command
            slot, // Targeted slot number (0x00 - 0x05)
            Self::SYSEX_END,
        ]
    }

    /// Live parameter edit (command 0x31)
    /// slot      : 0-5
    /// param_idx : 0 = on/off, 2-10 = knobs
    pub fn parameter_edit(model: PedalModel, slot: u8, param_idx: u8, value: u16) -> Vec<u8> {
        let val_lsb = (value & 0x7F) as u8;
        let val_msb = ((value >> 7) & 0x7F) as u8;

        vec![
            Self::SYSEX_START,
            0x52,
            0x00,
            model.device_id(),
            0x31,
            slot,
            param_idx,
            val_lsb,
            val_msb,
            Self::SYSEX_END,
        ]
    }

    //Save commmand
    pub fn save_current_patch(model: PedalModel, target_patch_idx: u8) -> Vec<u8> {
        vec![
            0xF0,
            0x52,
            0x00,
            model.device_id(),
            0x32,             // Patch Store / Write to Memory opcode
            target_patch_idx, // Patch slot (0-49)
            0xF7,
        ]
    }

    /// Write a complete patch to the current edit buffer (command 0x28)
    pub fn send_patch(model: PedalModel, raw_patch: &[u8]) -> Vec<u8> {
        let mut msg = vec![Self::SYSEX_START, 0x52, 0x00, model.device_id(), 0x28];
        msg.extend(Self::encode_7bit(raw_patch));
        msg.push(Self::SYSEX_END);
        msg
    }

    /// Force store current edit buffer to a memory slot (command 0x32)
    pub fn store_patch(model: PedalModel, number: u8) -> Vec<u8> {
        vec![
            Self::SYSEX_START,
            0x52,
            0x00,
            model.device_id(),
            0x32,
            0x01,
            0x00,
            0x00,
            number,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            Self::SYSEX_END,
        ]
    }

    /// Tuner on/off via CC#74
    pub fn tuner_control(_model: PedalModel, enable: bool) -> Vec<u8> {
        vec![
            0xB0, // Control Change, channel 1
            0x4A, // CC 74
            if enable { 0x7F } else { 0x00 },
        ]
    }

    // -----------------------------------------------------------------------------
    // 7-bit packing / unpacking (Zoom MIDI format)
    // -----------------------------------------------------------------------------

    /// Pack 8-bit data into Zoom's 7-bit MIDI format.
    pub fn encode_7bit(src: &[u8]) -> Vec<u8> {
        let mut dst = Vec::with_capacity((src.len() * 8 + 6) / 7);

        for chunk in src.chunks(7) {
            let mut header = 0u8;
            let mut data = Vec::with_capacity(chunk.len());

            for (i, &b) in chunk.iter().enumerate() {
                if b & 0x80 != 0 {
                    header |= 1 << i;
                }
                data.push(b & 0x7F);
            }

            dst.push(header);
            dst.extend(data);
        }

        dst
    }

    /// Unpack Zoom 7-bit MIDI data back to 8-bit.
    pub fn decode_7bit(src: &[u8]) -> Vec<u8> {
        let mut dst = Vec::new();
        let mut i = 0;

        while i < src.len() {
            let header = src[i];
            i += 1;

            for bit in 0..7 {
                if i >= src.len() {
                    break;
                }
                let mut b = src[i] & 0x7F;
                if header & (1 << bit) != 0 {
                    b |= 0x80;
                }
                dst.push(b);
                i += 1;
            }
        }

        dst
    }
}
