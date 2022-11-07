use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast16_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui64_f64 {
    pub ui: uint64_t,
    pub f: float64_t,
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
pub unsafe fn f64_roundToInt(
    mut a: float64_t,
    mut roundingMode: uint_fast8_t,
    mut exact: bool,
) -> float64_t {
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: uint_fast64_t = 0;
    let mut exp: int_fast16_t = 0;
    let mut uiZ: uint_fast64_t = 0;
    let mut lastBitMask: uint_fast64_t = 0;
    let mut roundBitsMask: uint_fast64_t = 0;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    uA.f = a;
    uiA = uA.ui;
    exp = (uiA >> 52 as i32) as int_fast16_t
        & 0x7ff as i32 as i64;
    if exp <= 0x3fe as i32 as i64 {
        if uiA & 0x7fffffffffffffff as u64 == 0 {
            return a;
        }
        if exact {
            softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                | softfloat_flag_inexact as i32) as uint_fast8_t;
        }
        uiZ = uiA
            & ((1 as i32 as uint_fast64_t) << 63 as i32)
                .wrapping_add((0 as i32 as uint_fast64_t) << 52 as i32)
                .wrapping_add(0 as i32 as u64);
        let mut current_block_14: u64;
        match roundingMode as i32 {
            0 => {
                if uiA & 0xfffffffffffff as u64 == 0 {
                    current_block_14 = 15904375183555213903;
                } else {
                    current_block_14 = 1143463644456424704;
                }
            }
            4 => {
                current_block_14 = 1143463644456424704;
            }
            2 => {
                if uiZ != 0 {
                    uiZ = ((1 as i32 as uint_fast64_t) << 63 as i32)
                        .wrapping_add(
                            (0x3ff as i32 as uint_fast64_t) << 52 as i32,
                        )
                        .wrapping_add(0 as i32 as u64);
                }
                current_block_14 = 15904375183555213903;
            }
            3 => {
                if uiZ == 0 {
                    uiZ = ((0 as i32 as uint_fast64_t) << 63 as i32)
                        .wrapping_add(
                            (0x3ff as i32 as uint_fast64_t) << 52 as i32,
                        )
                        .wrapping_add(0 as i32 as u64);
                }
                current_block_14 = 15904375183555213903;
            }
            6 => {
                uiZ
                    |= ((0 as i32 as uint_fast64_t) << 63 as i32)
                        .wrapping_add(
                            (0x3ff as i32 as uint_fast64_t) << 52 as i32,
                        )
                        .wrapping_add(0 as i32 as u64);
                current_block_14 = 15904375183555213903;
            }
            _ => {
                current_block_14 = 15904375183555213903;
            }
        }
        match current_block_14 {
            1143463644456424704 => {
                if exp == 0x3fe as i32 as i64 {
                    uiZ
                        |= ((0 as i32 as uint_fast64_t) << 63 as i32)
                            .wrapping_add(
                                (0x3ff as i32 as uint_fast64_t) << 52 as i32,
                            )
                            .wrapping_add(0 as i32 as u64);
                }
            }
            _ => {}
        }
    } else if 0x433 as i32 as i64 <= exp {
        if exp == 0x7ff as i32 as i64
            && uiA & 0xfffffffffffff as u64 != 0
        {
            uiZ = softfloat_propagateNaNF64UI(uiA, 0 as i32 as uint_fast64_t);
        } else {
            return a
        }
    } else {
        uiZ = uiA;
        lastBitMask = (1 as i32 as uint_fast64_t)
            << 0x433 as i32 as i64 - exp;
        roundBitsMask = lastBitMask.wrapping_sub(1 as i32 as u64);
        if roundingMode as i32 == softfloat_round_near_maxMag as i32 {
            uiZ = (uiZ as u64).wrapping_add(lastBitMask >> 1 as i32)
                as uint_fast64_t as uint_fast64_t;
        } else if roundingMode as i32 == softfloat_round_near_even as i32
            {
            uiZ = (uiZ as u64).wrapping_add(lastBitMask >> 1 as i32)
                as uint_fast64_t as uint_fast64_t;
            if uiZ & roundBitsMask == 0 {
                uiZ &= !lastBitMask;
            }
        } else if roundingMode as i32
                == (if uiZ >> 63 as i32 != 0 {
                    softfloat_round_min as i32
                } else {
                    softfloat_round_max as i32
                })
            {
            uiZ = (uiZ as u64).wrapping_add(roundBitsMask) as uint_fast64_t
                as uint_fast64_t;
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
    uZ.ui = uiZ;
    return uZ.f;
}
