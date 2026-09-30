/// Decoded information for a single effect slot
#[derive(Debug, Clone)]
pub struct DecodedSlot {
    pub raw_effect_id: u32,
    pub is_enabled: bool,
    pub params: [u16; 9],
}
