use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PedalModel {
    MS50G,
    MS60B,
    MS70CDR,
    Unknown,
}

impl PedalModel {
    /// Returns the Zoom Manufacturer Sub-ID / Model ID byte used in SysEx headers.
    pub fn device_id(&self) -> u8 {
        match self {
            PedalModel::MS50G => 0x58,
            PedalModel::MS60B => 0x5F,
            PedalModel::MS70CDR => 0x61,
            PedalModel::Unknown => 0x00,
        }
    }

    /// Total raw patch payload size in bytes.
    pub fn patch_length(&self) -> usize {
        match self {
            PedalModel::MS50G | PedalModel::MS60B | PedalModel::MS70CDR | PedalModel::Unknown => {
                146
            }
        }
    }

    /// Total available effect slots.
    pub fn slot_count(&self) -> usize {
        match self {
            PedalModel::MS50G | PedalModel::MS60B | PedalModel::MS70CDR | PedalModel::Unknown => 6,
        }
    }

    /// Maximum number of patches the pedal can store.
    pub fn max_patches(&self) -> usize {
        match self {
            PedalModel::MS50G | PedalModel::MS60B | PedalModel::MS70CDR | PedalModel::Unknown => 50,
        }
    }

    /// Parse device model from an incoming Universal Non-Real Time MIDI Identity Reply payload.
    /// Expected format: F0 7E <chan> 06 02 52 <device_id> ... F7
    pub fn from_identity_reply(payload: &[u8]) -> Option<Self> {
        if payload.len() < 7 {
            return None;
        }

        // Validate Universal Non-Realtime Identity Reply header
        if payload[0] != 0xF0 || payload[1] != 0x7E || payload[3] != 0x06 || payload[4] != 0x02 {
            return None;
        }

        // Check Zoom Manufacturer ID (0x52)
        if payload[5] != 0x52 {
            return None;
        }

        match payload[6] {
            0x58 => Some(PedalModel::MS50G),
            0x5F => Some(PedalModel::MS60B),
            0x61 => Some(PedalModel::MS70CDR),
            _ => Some(PedalModel::Unknown),
        }
    }

    /// Parse model from SysEx data response
    pub fn from_sysex_data(payload: &[u8]) -> Option<Self> {
        if payload.len() < 4 {
            return None;
        }
        if payload[0] != 0xF0 || payload[1] != 0x52 {
            return None;
        }
        match payload[3] {
            0x58 => Some(PedalModel::MS50G),
            0x5F => Some(PedalModel::MS60B),
            0x61 => Some(PedalModel::MS70CDR),
            _ => Some(PedalModel::Unknown),
        }
    }
}

impl Default for PedalModel {
    fn default() -> Self {
        PedalModel::Unknown
    }
}

impl fmt::Display for PedalModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PedalModel::MS50G => write!(f, "Zoom MS-50G"),
            PedalModel::MS60B => write!(f, "Zoom MS-60B"),
            PedalModel::MS70CDR => write!(f, "Zoom MS-70CDR"),
            PedalModel::Unknown => write!(f, "Unknown Device"),
        }
    }
}
