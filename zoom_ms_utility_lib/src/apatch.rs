/* apatch.rs
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

//! Rust translation of the JS `apatch` SysEx patch model.
//!
//! Notes on the port:
//! - JS bit-descriptor tuples `[byte, mask, shift]` / `[byte, mask, "NZ"]`
//!   become the `BitField` enum, built with the `bf!` macro below.
//! - The JS code reads an implicit global `effectlist` when no local list is
//!   passed to `MakeBin`. Rust has no ambient globals, so `make_bin` takes an
//!   explicit `effect_list: &[EffectDef]` slice (indexed the same way the JS
//!   `fx[f][1]` effect-id values index into your effect list).
//! - Packed values are stored as `i32` (not `u8`) because some fields (e.g.
//!   the effect ID) are assembled from bits scattered across several bytes
//!   and don't fit in a single byte once shifted.

use crate::effects::EffectDef;

/// A single bit-field descriptor: either a shifted/masked numeric field,
/// or a "non-zero" flag field (JS's `"NZ"` marker).
#[derive(Clone, Copy, Debug)]
pub enum BitField {
    /// Extract/insert `mask` bits at `byte`, shifting left by `shift` if
    /// `shift >= 0`, or right by `-shift` if `shift < 0`.
    Field { byte: usize, mask: i32, shift: i32 },
    /// Write `mask` into `byte` if the logical value is non-zero, else 0.
    /// (Read side contributes nothing to the assembled value, matching the
    /// original `GetBits` behavior.)
    NonZero { byte: usize, mask: i32 },
}

macro_rules! bf {
    ($byte:expr, $mask:expr, NZ) => {
        BitField::NonZero {
            byte: $byte,
            mask: $mask,
        }
    };
    ($byte:expr, $mask:expr, $shift:expr) => {
        BitField::Field {
            byte: $byte,
            mask: $mask,
            shift: $shift,
        }
    };
}

/// One parameter's list of bit-field descriptors.
type ParamBits = Vec<BitField>;
/// One effect slot's 11 parameters.
type EffectBits = [ParamBits; 11];
/// All 6 effect slots.
type AllBits = [EffectBits; 6];

fn bits_table() -> AllBits {
    [
        // eff0
        [
            vec![bf!(6, 1, 0)], // State
            vec![
                bf!(5, 0x40, 24),
                bf!(6, 0x7e, 16),
                bf!(7, 0x07, 8),
                bf!(9, 0x1f, 0),
            ], // ID
            vec![
                bf!(9, 0x60, -5),
                bf!(5, 0x8, -1),
                bf!(10, 0x7f, 3),
                bf!(5, 0x4, 8),
                bf!(11, 0x1, 11),
            ],
            vec![bf!(11, 0x7c, -2), bf!(5, 0x2, 4), bf!(12, 0x1f, 6)],
            vec![
                bf!(5, 0x1, 0),
                bf!(14, 0x7f, 1),
                bf!(13, 0x40, 2),
                bf!(15, 0x3, 9),
            ],
            vec![bf!(15, 0x70, -4), bf!(13, 0x20, -2), bf!(16, 0x0f, 4)],
            vec![bf!(16, 0x70, -4), bf!(13, 0x10, -1), bf!(17, 0x0f, 4)],
            vec![bf!(17, 0x70, -4), bf!(13, 0x08, 0), bf!(18, 0x0f, 4)],
            vec![bf!(18, 0x70, -4), bf!(13, 0x04, 1), bf!(19, 0x0f, 4)],
            vec![
                bf!(19, 0x70, -4),
                bf!(13, 0x02, 2),
                bf!(20, 0x1f, 4),
                bf!(23, 0x40, NZ),
            ],
            vec![bf!(24, 0x7f, 0), bf!(21, 0x10, 3)],
        ],
        // eff1
        [
            vec![bf!(26, 1, 0)],
            vec![
                bf!(21, 0x4, 28),
                bf!(26, 0x7e, 16),
                bf!(27, 0x07, 8),
                bf!(30, 0x1f, 0),
            ],
            vec![
                bf!(30, 0x60, -5),
                bf!(29, 0x40, -4),
                bf!(31, 0x7f, 3),
                bf!(29, 0x20, 5),
                bf!(32, 0x1, 11),
            ],
            vec![bf!(32, 0x7c, -2), bf!(29, 0x10, 1), bf!(33, 0x1f, 6)],
            vec![
                bf!(29, 0x8, -3),
                bf!(34, 0x7f, 1),
                bf!(29, 0x4, 6),
                bf!(35, 0x3, 9),
            ],
            vec![bf!(35, 0x70, -4), bf!(29, 0x2, 2), bf!(36, 0x0f, 4)],
            vec![bf!(36, 0x70, -4), bf!(29, 0x1, 3), bf!(38, 0x0f, 4)],
            vec![bf!(38, 0x70, -4), bf!(37, 0x40, -3), bf!(39, 0x0f, 4)],
            vec![bf!(39, 0x70, -4), bf!(37, 0x20, -2), bf!(40, 0x0f, 4)],
            vec![
                bf!(40, 0x70, -4),
                bf!(37, 0x10, -1),
                bf!(41, 0x1f, 4),
                bf!(43, 0x40, NZ),
            ],
            vec![bf!(44, 0x7f, 0), bf!(37, 0x1, 7)],
        ],
        // eff2
        [
            vec![bf!(47, 1, 0)],
            vec![
                bf!(45, 0x20, 25),
                bf!(47, 0x7e, 16),
                bf!(48, 0x07, 8),
                bf!(50, 0x1f, 0),
            ],
            vec![
                bf!(50, 0x60, -5),
                bf!(45, 0x4, 0),
                bf!(51, 0x7f, 3),
                bf!(45, 0x2, 9),
                bf!(52, 0x1, 11),
            ],
            vec![bf!(52, 0x7c, -2), bf!(45, 0x1, 5), bf!(54, 0x1f, 6)],
            vec![
                bf!(53, 0x40, -6),
                bf!(55, 0x7f, 1),
                bf!(53, 0x20, 3),
                bf!(56, 0x3, 9),
            ],
            vec![bf!(56, 0x70, -4), bf!(53, 0x10, -1), bf!(57, 0x0f, 4)],
            vec![bf!(57, 0x70, -4), bf!(53, 0x8, 0), bf!(58, 0x0f, 4)],
            vec![bf!(58, 0x70, -4), bf!(53, 0x4, 1), bf!(59, 0x0f, 4)],
            vec![bf!(59, 0x70, -4), bf!(53, 0x2, 2), bf!(60, 0x0f, 4)],
            vec![
                bf!(60, 0x70, -4),
                bf!(53, 0x1, 3),
                bf!(62, 0x1f, 4),
                bf!(64, 0x40, NZ),
            ],
            vec![bf!(65, 0x7f, 0), bf!(61, 0x8, 4)],
        ],
        // eff3
        [
            vec![bf!(67, 1, 0)],
            vec![
                bf!(61, 0x2, 29),
                bf!(67, 0x7e, 16),
                bf!(68, 0x07, 8),
                bf!(71, 0x1f, 0),
            ],
            vec![
                bf!(71, 0x60, -5),
                bf!(69, 0x20, -3),
                bf!(72, 0x7f, 3),
                bf!(69, 0x10, 6),
                bf!(73, 0x1, 11),
            ],
            vec![bf!(73, 0x7c, -2), bf!(69, 0x8, 2), bf!(74, 0x1f, 6)],
            vec![
                bf!(69, 0x4, -2),
                bf!(75, 0x7f, 1),
                bf!(69, 0x2, 7),
                bf!(76, 0x3, 9),
            ],
            vec![bf!(76, 0x70, -4), bf!(69, 0x1, 3), bf!(78, 0x0f, 4)],
            vec![bf!(78, 0x70, -4), bf!(77, 0x40, -3), bf!(79, 0x0f, 4)],
            vec![bf!(79, 0x70, -4), bf!(77, 0x20, -2), bf!(80, 0x0f, 4)],
            vec![bf!(80, 0x70, -4), bf!(77, 0x10, -1), bf!(81, 0x0f, 4)],
            vec![
                bf!(81, 0x70, -4),
                bf!(77, 0x8, 0),
                bf!(82, 0x1f, 4),
                bf!(84, 0x40, NZ),
            ],
            vec![bf!(86, 0x7f, 0), bf!(85, 0x40, 1)],
        ],
        // eff4
        [
            vec![bf!(88, 1, 0)],
            vec![
                bf!(85, 0x10, 26),
                bf!(88, 0x7e, 16),
                bf!(89, 0x07, 8),
                bf!(91, 0x1f, 0),
            ],
            vec![
                bf!(91, 0x60, -5),
                bf!(85, 0x2, 1),
                bf!(92, 0x7f, 3),
                bf!(85, 0x1, 10),
                bf!(94, 0x1, 11),
            ],
            vec![bf!(94, 0x7c, -2), bf!(93, 0x40, -1), bf!(95, 0x1f, 6)],
            vec![
                bf!(93, 0x20, -5),
                bf!(96, 0x7f, 1),
                bf!(93, 0x10, 4),
                bf!(97, 0x3, 9),
            ],
            vec![bf!(97, 0x70, -4), bf!(93, 0x8, 0), bf!(98, 0x0f, 4)],
            vec![bf!(98, 0x70, -4), bf!(93, 0x4, 1), bf!(99, 0x0f, 4)],
            vec![bf!(99, 0x70, -4), bf!(93, 0x2, 2), bf!(100, 0x0f, 4)],
            vec![bf!(100, 0x70, -4), bf!(93, 0x1, 3), bf!(102, 0x0f, 4)],
            vec![
                bf!(102, 0x70, -4),
                bf!(101, 0x40, -3),
                bf!(103, 0x1f, 4),
                bf!(105, 0x40, NZ),
            ],
            vec![bf!(106, 0x7f, 0), bf!(106, 0x4, 5)],
        ],
        // eff5
        [
            vec![bf!(108, 1, 0)],
            vec![
                bf!(101, 0x1, 30),
                bf!(108, 0x7e, 16),
                bf!(110, 0x07, 8),
                bf!(112, 0x1f, 0),
            ],
            vec![
                bf!(112, 0x60, -5),
                bf!(109, 0x10, -2),
                bf!(113, 0x7f, 3),
                bf!(109, 0x8, 7),
                bf!(114, 0x1, 11),
            ],
            vec![bf!(114, 0x7c, -2), bf!(109, 0x4, 3), bf!(115, 0x1f, 6)],
            vec![
                bf!(109, 0x2, -1),
                bf!(116, 0x7f, 1),
                bf!(109, 0x1, 8),
                bf!(118, 0x3, 9),
            ],
            vec![bf!(118, 0x70, -4), bf!(117, 0x40, -3), bf!(119, 0x0f, 4)],
            vec![bf!(119, 0x70, -4), bf!(117, 0x20, -2), bf!(120, 0x0f, 4)],
            vec![bf!(120, 0x70, -4), bf!(117, 0x10, -1), bf!(121, 0x0f, 4)],
            vec![bf!(121, 0x70, -4), bf!(117, 0x8, 0), bf!(122, 0x0f, 4)],
            vec![
                bf!(122, 0x70, -4),
                bf!(117, 0x4, 1),
                bf!(123, 0x1f, 4),
                bf!(126, 0x40, NZ),
            ],
            vec![bf!(127, 0x7f, 0), bf!(125, 0x20, 2)],
        ],
    ]
}

const NAMIDX: [[usize; 10]; 2] = [
    [91, 92, 94, 95, 96, 97, 98, 99, 100, 102],
    [132, 134, 135, 136, 137, 138, 139, 140, 142, 143],
];

const CABBYTE: [usize; 6] = [23, 43, 64, 84, 105, 126];
const V2BYTE: [usize; 4] = [8, 28, 49, 70];
const MAXFXIDX: [usize; 2] = [89, 130];

#[rustfmt::skip]
const EMPTY146: [u8; 146] = [
    0xf0,0x52,0x00,0x58,0x28,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x40,0x05,0x0f,0x45,0x00,0x6d,0x70,0x74,0x79,0x20,0x20,
    0x20,0x00,0x20,0x20,0x00,0xf7,
];

#[rustfmt::skip]
const EMPTY105: [u8; 105] = [
    0xf0,0x52,0x00,0x5f,0x28,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x10,0x00,0x00,0x40,0x04,0x0f,0x45,0x6d,0x00,0x70,0x74,0x79,0x20,0x20,0x20,
    0x20,0x00,0x20,0x00,0xf7,
];

#[derive(Clone, Debug)]
pub struct Apatch {
    pub name: String,
    /// 6 effect slots x 11 parameters, matching the JS `fx` array.
    pub fx: [[i32; 11]; 6],
    pub maxfx: i32,
    pub curfx: i32,
    pub dspstate: i32,
}

impl Default for Apatch {
    fn default() -> Self {
        Self::new()
    }
}

impl Apatch {
    pub fn new() -> Self {
        Apatch {
            name: String::new(),
            fx: [[0; 11]; 6],
            maxfx: 1,
            curfx: 0,
            dspstate: 0,
        }
    }

    pub fn copy_from(&mut self, other: &Apatch) {
        self.name = other.name.clone();
        self.fx = other.fx;
        self.maxfx = other.maxfx;
        self.curfx = other.curfx;
        self.dspstate = 0;
    }

    pub fn get_param_val(&self, n: usize, p: usize) -> i32 {
        self.fx[n][p]
    }

    pub fn get_effect_id(&self, n: usize) -> i32 {
        self.fx[n][1]
    }

    pub fn get_effect_state(&self, n: usize) -> i32 {
        self.fx[n][0]
    }

    pub fn get_dsp_state(&self, n: u32) -> i32 {
        (self.dspstate >> n) & 1
    }

    pub fn get_cur_fx_bit(dat: &[u8]) -> i32 {
        if dat.len() < 146 {
            3 - (((dat[88] as i32 & 0x40) >> 6) + ((dat[85] as i32 & 0x10) >> 3))
        } else {
            6 - (((dat[130] as i32 & 1) << 2)
                + ((dat[125] as i32 & 8) >> 2)
                + ((dat[129] as i32 & 0x40) >> 6))
        }
    }

    pub fn set_cur_fx_bit(dat: &mut [u8], n: i32) {
        if dat.len() < 146 {
            let n = 3 - n;
            dat[88] = ((dat[88] as i32 & !0x40) + ((n & 1) << 6)) as u8;
            dat[85] = ((dat[85] as i32 & !0x10) + ((n & 2) << 3)) as u8;
        } else {
            let n = 5 - n;
            dat[129] = ((dat[129] as i32 & !0x40) + ((n & 1) << 6)) as u8;
            dat[125] = ((dat[125] as i32 & !0x8) + ((n & 2) << 2)) as u8;
            dat[130] = ((dat[130] as i32 & !1) + ((n & 4) >> 2)) as u8;
        }
    }

    pub fn set_max_fx_bit(dat: &mut [u8], n: i32) {
        let len = dat.len();
        let o = MAXFXIDX[if len >= 146 { 1 } else { 0 }];
        let mut n = n;
        if n == 0 {
            n = 1;
        }
        if n > 6 {
            n = 6;
        }
        if len < 146 && n > 4 {
            n = 4;
        }
        dat[o] = ((dat[o] as i32 & !0x1c) + (n << 2)) as u8;
    }

    pub fn get_max_fx_bit(dat: &[u8]) -> i32 {
        let idx = MAXFXIDX[if dat.len() >= 146 { 1 } else { 0 }];
        (dat[idx] as i32 & 0x1c) >> 2
    }

    pub fn set_bits(dat: &mut [u8], bits: &[BitField], val: i32) {
        let len = dat.len();
        for b in bits {
            match *b {
                BitField::NonZero { byte, mask } => {
                    if byte < len {
                        dat[byte] =
                            ((dat[byte] as i32 & !mask) + if val != 0 { mask } else { 0 }) as u8;
                    }
                }
                BitField::Field { byte, mask, shift } => {
                    if byte < len {
                        let v = if shift >= 0 {
                            val >> shift
                        } else {
                            val << -shift
                        };
                        dat[byte] = ((dat[byte] as i32 & !mask) + (v & mask)) as u8;
                    }
                }
            }
        }
    }

    pub fn get_bits(dat: &[u8], bits: &[BitField]) -> i32 {
        let len = dat.len();
        let mut val = 0i32;
        for b in bits {
            if let BitField::Field { byte, mask, shift } = *b {
                if byte < len {
                    let mut v = dat[byte] as i32 & mask;
                    if shift >= 0 {
                        v <<= shift;
                    } else {
                        v >>= -shift;
                    }
                    val |= v;
                }
            }
        }
        val
    }

    pub fn read_bin(&mut self, dat: &[u8]) {
        let len = dat.len();
        let base = if len >= 146 { 132 } else { 91 };

        let mut name = String::new();
        for j in 0..13 {
            let c = dat[base + j];
            if c != 0 {
                name.push(c as char);
            }
        }
        self.name = name.trim_end_matches(' ').to_string();

        let flen = if len >= 146 { 6 } else { 4 };
        let table = bits_table();
        for f in 0..6 {
            for p in 0..11 {
                if f >= flen {
                    self.fx[f][p] = 0;
                } else {
                    self.fx[f][p] = Self::get_bits(dat, &table[f][p]);
                }
            }
        }

        self.maxfx = Self::get_max_fx_bit(dat);
        self.curfx = Self::get_cur_fx_bit(dat);
        self.dspstate = dat[if len >= 146 { 129 } else { 88 }] as i32 & 0x3f;
    }

    pub fn make_bin(&self, id: u8, effect_list: &[EffectDef]) -> Vec<u8> {
        let (mut r, flen): (Vec<u8>, usize) = if id == 0x5f {
            (EMPTY105.to_vec(), 4)
        } else {
            (EMPTY146.to_vec(), 6)
        };
        r[3] = id;
        let len = r.len();

        let mut name = self.name.clone();
        if name.len() > 10 {
            name.truncate(10);
        }
        let name_idx = NAMIDX[if id == 0x5f { 0 } else { 1 }];
        let name_bytes = name.as_bytes();
        for i in 0..10 {
            let c = name_bytes.get(i).copied().unwrap_or(b' ');
            r[name_idx[i]] = if i < name_bytes.len() { c } else { 0x20 };
        }

        let table = bits_table();
        for f in 0..flen {
            for p in 0..11 {
                Self::set_bits(&mut r, &table[f][p], self.fx[f][p]);
            }

            let eff_id = self.fx[f][1] as u32;
            if let Some(ef) = effect_list.iter().find(|e| e.id == eff_id) {
                if ef.group == "AMP" {
                    if (ef.ver & 0xf0) == 0x20 {
                        if f < V2BYTE.len() {
                            r[V2BYTE[f]] = 0x20;
                        }
                    }
                    if ef.ver & 0xf != 0 {
                        r[CABBYTE[f]] = if self.fx[f][9] != 0 { 0x40 } else { 0 };
                    } else {
                        r[CABBYTE[f]] = match self.fx[f][9] {
                            0 => 0,
                            16 | 32 | 48 | 96 | 112 | 192 => 0x50,
                            _ => 0x51,
                        };
                    }
                }
            }
        }

        let mut i: i32 = if len >= 146 { 5 } else { 3 };
        while i > 0 {
            if self.fx[i as usize][1] != 0 {
                break;
            }
            i -= 1;
        }
        Self::set_max_fx_bit(&mut r, i + 1);
        Self::set_cur_fx_bit(&mut r, self.curfx);

        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_name() {
        let mut p = Apatch::new();
        p.name = "Test".to_string();
        let effects: Vec<EffectDef> = vec![];
        let bin = p.make_bin(0x58, &effects);
        let mut p2 = Apatch::new();
        p2.read_bin(&bin);
        assert_eq!(p2.name, "Test");
    }
}
