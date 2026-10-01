/* patch_info.rs
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
