use crate::model::PedalModel;
use crate::patch_info::PatchInfo;

/// Library of patches - dynamically sized based on pedal model
#[derive(Debug, Clone)]
pub struct PatchLibrary {
    pub model: PedalModel,
    pub patches: Vec<Option<PatchInfo>>,
}

impl PatchLibrary {
    pub fn new(model: PedalModel) -> Self {
        let max_patches = model.max_patches();
        Self {
            model,
            patches: vec![None; max_patches],
        }
    }

    pub fn patches(&self) -> &[Option<PatchInfo>] {
        &self.patches
    }

    pub fn get(&self, index: u8) -> Option<&PatchInfo> {
        self.patches.get(index as usize)?.as_ref()
    }

    pub fn get_mut(&mut self, index: u8) -> Option<&mut PatchInfo> {
        self.patches.get_mut(index as usize)?.as_mut()
    }

    pub fn count(&self) -> usize {
        self.patches.iter().filter(|p| p.is_some()).count()
    }

    pub fn max_patches(&self) -> usize {
        self.patches.len()
    }

    pub fn is_valid_slot(&self, slot: u8) -> bool {
        (slot as usize) < self.patches.len()
    }

    /// Resize the library if needed
    pub fn resize(&mut self, new_size: usize) {
        if new_size > self.patches.len() {
            self.patches.resize(new_size, None);
        }
    }

    /// Get all non-empty patches
    pub fn non_empty(&self) -> Vec<&PatchInfo> {
        self.patches.iter().filter_map(|p| p.as_ref()).collect()
    }

    /// Find a patch by name (case-insensitive partial match)
    pub fn find_by_name(&self, name: &str) -> Option<&PatchInfo> {
        let name_lower = name.to_lowercase();
        self.patches.iter().find_map(|p| {
            p.as_ref()
                .filter(|info| info.name.to_lowercase().contains(&name_lower))
        })
    }
}
