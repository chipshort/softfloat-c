use crate::*;

pub type __uint16_t = u16;
pub type uint16_t = __uint16_t;
pub type int_fast8_t = i8;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui16_f16 {
    pub ui: uint16_t,
    pub f: float16_t,
}
pub type C2RustUnnamed = u32;
pub const softfloat_round_odd: C2RustUnnamed = 6;
pub const softfloat_round_near_maxMag: C2RustUnnamed = 4;
pub const softfloat_round_max: C2RustUnnamed = 3;
pub const softfloat_round_min: C2RustUnnamed = 2;
pub const softfloat_round_minMag: C2RustUnnamed = 1;
pub const softfloat_round_near_even: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = u32;
pub const softfloat_flag_invalid: C2RustUnnamed_0 = 16;
pub const softfloat_flag_infinite: C2RustUnnamed_0 = 8;
pub const softfloat_flag_overflow: C2RustUnnamed_0 = 4;
pub const softfloat_flag_underflow: C2RustUnnamed_0 = 2;
pub const softfloat_flag_inexact: C2RustUnnamed_0 = 1;
pub unsafe fn f16_roundToInt(
    mut a: float16_t,
    mut roundingMode: uint_fast8_t,
    mut exact: bool,
) -> float16_t {
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiA: uint_fast16_t = 0;
    let mut exp: int_fast8_t = 0;
    let mut uiZ: uint_fast16_t = 0;
    let mut lastBitMask: uint_fast16_t = 0;
    let mut roundBitsMask: uint_fast16_t = 0;
    let mut uZ: ui16_f16 = ui16_f16 { ui: 0 };
    uA.f = a;
    uiA = uA.ui as uint_fast16_t;
    exp = ((uiA >> 10 as i32) as int_fast8_t as i32
        & 0x1f as i32) as int_fast8_t;
    if exp as i32 <= 0xe as i32 {
        if (uiA << 1 as i32) as uint16_t == 0 {
            return a;
        }
        if exact {
            softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                | softfloat_flag_inexact as i32) as uint_fast8_t;
        }
        uiZ = uiA
            & (((1 as i32 as uint16_t as i32) << 15 as i32)
                + ((0 as i32 as uint16_t as i32) << 10 as i32)
                + 0 as i32) as u64;
        let mut current_block_14: u64;
        match roundingMode as i32 {
            0 => {
                if uiA & 0x3ff as i32 as u64 == 0 {
                    current_block_14 = 15904375183555213903;
                } else {
                    current_block_14 = 6382675588328852115;
                }
            }
            4 => {
                current_block_14 = 6382675588328852115;
            }
            2 => {
                if uiZ != 0 {
                    uiZ = (((1 as i32 as uint16_t as i32)
                        << 15 as i32)
                        + ((0xf as i32 as uint16_t as i32)
                            << 10 as i32) + 0 as i32) as uint_fast16_t;
                }
                current_block_14 = 15904375183555213903;
            }
            3 => {
                if uiZ == 0 {
                    uiZ = (((0 as i32 as uint16_t as i32)
                        << 15 as i32)
                        + ((0xf as i32 as uint16_t as i32)
                            << 10 as i32) + 0 as i32) as uint_fast16_t;
                }
                current_block_14 = 15904375183555213903;
            }
            6 => {
                uiZ
                    |= (((0 as i32 as uint16_t as i32)
                        << 15 as i32)
                        + ((0xf as i32 as uint16_t as i32)
                            << 10 as i32) + 0 as i32) as u64;
                current_block_14 = 15904375183555213903;
            }
            _ => {
                current_block_14 = 15904375183555213903;
            }
        }
        match current_block_14 {
            6382675588328852115 => {
                if exp as i32 == 0xe as i32 {
                    uiZ
                        |= (((0 as i32 as uint16_t as i32)
                            << 15 as i32)
                            + ((0xf as i32 as uint16_t as i32)
                                << 10 as i32) + 0 as i32) as u64;
                }
            }
            _ => {}
        }
    } else if 0x19 as i32 <= exp as i32 {
        if exp as i32 == 0x1f as i32
            && uiA & 0x3ff as i32 as u64 != 0
        {
            uiZ = softfloat_propagateNaNF16UI(uiA, 0 as i32 as uint_fast16_t);
        } else {
            return a
        }
    } else {
        uiZ = uiA;
        lastBitMask = (1 as i32 as uint_fast16_t)
            << 0x19 as i32 - exp as i32;
        roundBitsMask = lastBitMask.wrapping_sub(1 as i32 as u64);
        if roundingMode as i32 == softfloat_round_near_maxMag as i32 {
            uiZ = (uiZ as u64).wrapping_add(lastBitMask >> 1 as i32)
                as uint_fast16_t as uint_fast16_t;
        } else if roundingMode as i32 == softfloat_round_near_even as i32
            {
            uiZ = (uiZ as u64).wrapping_add(lastBitMask >> 1 as i32)
                as uint_fast16_t as uint_fast16_t;
            if uiZ & roundBitsMask == 0 {
                uiZ &= !lastBitMask;
            }
        } else if roundingMode as i32
                == (if uiZ as uint16_t as i32 >> 15 as i32 != 0 {
                    softfloat_round_min as i32
                } else {
                    softfloat_round_max as i32
                })
            {
            uiZ = (uiZ as u64).wrapping_add(roundBitsMask) as uint_fast16_t
                as uint_fast16_t;
        }
        uiZ &= !roundBitsMask;
        if uiZ != uiA {
            if roundingMode as i32 == softfloat_round_odd as i32 {
                uiZ |= lastBitMask;
            }
            if exact {
                softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                    | softfloat_flag_inexact as i32) as uint_fast8_t;
            }
        }
    }
    uZ.ui = uiZ as uint16_t;
    return uZ.f;
}
