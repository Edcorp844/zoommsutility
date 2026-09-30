use crate::patch_buffer::PatchBuffer;

/// Information about a patch in the library
#[derive(Debug, Clone)]
pub struct PatchInfo {
    pub number: u8,
    pub name: String,
    pub buffer: PatchBuffer,
    pub active_slots: usize,
    pub effect_ids: [u32; 6],
    pub estimated_dsp: f32,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub is_empty: bool,
}
