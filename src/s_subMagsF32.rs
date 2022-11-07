use crate::*;

pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
pub type int_fast8_t = i8;
pub type int_fast16_t = i64;
pub type int_fast32_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast32_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui32_f32 {
    pub ui: uint32_t,
    pub f: float32_t,
}
pub const softfloat_round_min: C2RustUnnamed = 2;
pub const softfloat_flag_invalid: C2RustUnnamed_0 = 16;
pub type C2RustUnnamed = u32;
pub const softfloat_round_odd: C2RustUnnamed = 6;
pub const softfloat_round_near_maxMag: C2RustUnnamed = 4;
pub const softfloat_round_max: C2RustUnnamed = 3;
pub const softfloat_round_minMag: C2RustUnnamed = 1;
pub const softfloat_round_near_even: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = u32;
pub const softfloat_flag_infinite: C2RustUnnamed_0 = 8;
pub const softfloat_flag_overflow: C2RustUnnamed_0 = 4;
pub const softfloat_flag_underflow: C2RustUnnamed_0 = 2;
pub const softfloat_flag_inexact: C2RustUnnamed_0 = 1;
#[inline]
unsafe fn softfloat_countLeadingZeros32(mut a: uint32_t) -> uint_fast8_t {
    return (if a != 0 { a.leading_zeros() as i32 } else { 32 as i32 })
        as uint_fast8_t;
}
#[inline]
unsafe fn softfloat_shiftRightJam32(
    mut a: uint32_t,
    mut dist: uint_fast16_t,
) -> uint32_t {
    return if dist < 31 as i32 as u64 {
        a >> dist
            | (a << (dist.wrapping_neg() & 31 as i32 as u64)
                != 0 as i32 as u32) as i32 as u32
    } else {
        (a != 0 as i32 as u32) as i32 as u32
    };
}
pub unsafe fn softfloat_subMagsF32(
    mut uiA: uint_fast32_t,
    mut uiB: uint_fast32_t,
) -> float32_t {
    let mut current_block: u64;
    let mut expA: int_fast16_t = 0;
    let mut sigA: uint_fast32_t = 0;
    let mut expB: int_fast16_t = 0;
    let mut sigB: uint_fast32_t = 0;
    let mut expDiff: int_fast16_t = 0;
    let mut uiZ: uint_fast32_t = 0;
    let mut sigDiff: int_fast32_t = 0;
    let mut signZ: bool = false;
    let mut shiftDist: int_fast8_t = 0;
    let mut expZ: int_fast16_t = 0;
    let mut sigX: uint_fast32_t = 0;
    let mut sigY: uint_fast32_t = 0;
    let mut uZ: ui32_f32 = ui32_f32 { ui: 0 };
    expA = (uiA >> 23 as i32) as int_fast16_t
        & 0xff as i32 as i64;
    sigA = uiA & 0x7fffff as i32 as u64;
    expB = (uiB >> 23 as i32) as int_fast16_t
        & 0xff as i32 as i64;
    sigB = uiB & 0x7fffff as i32 as u64;
    expDiff = expA - expB;
    if expDiff == 0 {
        if expA == 0xff as i32 as i64 {
            if sigA | sigB != 0 {
                current_block = 711145919618150066;
            } else {
                softfloat_raiseFlags(
                    softfloat_flag_invalid as i32 as uint_fast8_t,
                );
                uiZ = 0xffc00000 as u32 as uint_fast32_t;
                current_block = 10583070795618549991;
            }
        } else {
            sigDiff = sigA.wrapping_sub(sigB) as int_fast32_t;
            if sigDiff == 0 {
                uiZ = (((softfloat_roundingMode as i32
                    == softfloat_round_min as i32) as i32 as uint32_t)
                    << 31 as i32)
                    .wrapping_add((0 as i32 as uint32_t) << 23 as i32)
                    .wrapping_add(0 as i32 as u32) as uint_fast32_t;
            } else {
                if expA != 0 {
                    expA -= 1;
                }
                signZ = uiA as uint32_t >> 31 as i32 != 0;
                if sigDiff < 0 as i32 as i64 {
                    signZ = !signZ;
                    sigDiff = -sigDiff;
                }
                shiftDist = (softfloat_countLeadingZeros32(sigDiff as uint32_t)
                    as i32 - 8 as i32) as int_fast8_t;
                expZ = expA - shiftDist as i64;
                if expZ < 0 as i32 as i64 {
                    shiftDist = expA as int_fast8_t;
                    expZ = 0 as i32 as int_fast16_t;
                }
                uiZ = (((signZ as uint32_t) << 31 as i32)
                    .wrapping_add((expZ as uint32_t) << 23 as i32)
                    as i64 + (sigDiff << shiftDist as i32))
                    as uint_fast32_t;
            }
            current_block = 10583070795618549991;
        }
    } else {
        signZ = uiA as uint32_t >> 31 as i32 != 0;
        sigA <<= 7 as i32;
        sigB <<= 7 as i32;
        if expDiff < 0 as i32 as i64 {
            signZ = !signZ;
            if expB == 0xff as i32 as i64 {
                if sigB != 0 {
                    current_block = 711145919618150066;
                } else {
                    uiZ = ((signZ as uint32_t) << 31 as i32)
                        .wrapping_add(
                            (0xff as i32 as uint32_t) << 23 as i32,
                        )
                        .wrapping_add(0 as i32 as u32) as uint_fast32_t;
                    current_block = 10583070795618549991;
                }
            } else {
                expZ = expB - 1 as i32 as i64;
                sigX = sigB | 0x40000000 as i32 as u64;
                sigY = sigA
                    .wrapping_add(
                        (if expA != 0 {
                            0x40000000 as i32 as u64
                        } else {
                            sigA
                        }),
                    );
                expDiff = -expDiff;
                current_block = 15090052786889560393;
            }
        } else if expA == 0xff as i32 as i64 {
            if sigA != 0 {
                current_block = 711145919618150066;
            } else {
                uiZ = uiA;
                current_block = 10583070795618549991;
            }
        } else {
            expZ = expA - 1 as i32 as i64;
            sigX = sigA | 0x40000000 as i32 as u64;
            sigY = sigB
                .wrapping_add(
                    (if expB != 0 {
                        0x40000000 as i32 as u64
                    } else {
                        sigB
                    }),
                );
            current_block = 15090052786889560393;
        }
        match current_block {
            10583070795618549991 => {}
            711145919618150066 => {}
            _ => {
                return softfloat_normRoundPackToF32(
                    signZ,
                    expZ,
                    sigX
                        .wrapping_sub(
                            softfloat_shiftRightJam32(
                                sigY as uint32_t,
                                expDiff as uint_fast16_t,
                            ) as u64,
                        ),
                );
            }
        }
    }
    match current_block {
        711145919618150066 => {
            uiZ = softfloat_propagateNaNF32UI(uiA, uiB);
        }
        _ => {}
    }
    uZ.ui = uiZ as uint32_t;
    return uZ.f;
}
