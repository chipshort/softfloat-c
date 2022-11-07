use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast8_t = i8;
pub type int_fast16_t = i64;
pub type int_fast64_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui64_f64 {
    pub ui: uint64_t,
    pub f: float64_t,
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
unsafe fn softfloat_countLeadingZeros64(mut a: uint64_t) -> uint_fast8_t {
    return (if a != 0 {
        (a as u64).leading_zeros() as i32
    } else {
        64 as i32
    }) as uint_fast8_t;
}
#[inline]
unsafe fn softfloat_shiftRightJam64(
    mut a: uint64_t,
    mut dist: uint_fast32_t,
) -> uint64_t {
    return if dist < 63 as i32 as u64 {
        a >> dist
            | (a << (dist.wrapping_neg() & 63 as i32 as u64)
                != 0 as i32 as u64) as i32 as u64
    } else {
        (a != 0 as i32 as u64) as i32 as u64
    };
}
pub unsafe fn softfloat_subMagsF64(
    mut uiA: uint_fast64_t,
    mut uiB: uint_fast64_t,
    mut signZ: bool,
) -> float64_t {
    let mut current_block: u64;
    let mut expA: int_fast16_t = 0;
    let mut sigA: uint_fast64_t = 0;
    let mut expB: int_fast16_t = 0;
    let mut sigB: uint_fast64_t = 0;
    let mut expDiff: int_fast16_t = 0;
    let mut uiZ: uint_fast64_t = 0;
    let mut sigDiff: int_fast64_t = 0;
    let mut shiftDist: int_fast8_t = 0;
    let mut expZ: int_fast16_t = 0;
    let mut sigZ: uint_fast64_t = 0;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    expA = (uiA >> 52 as i32) as int_fast16_t
        & 0x7ff as i32 as i64;
    sigA = uiA & 0xfffffffffffff as u64;
    expB = (uiB >> 52 as i32) as int_fast16_t
        & 0x7ff as i32 as i64;
    sigB = uiB & 0xfffffffffffff as u64;
    expDiff = expA - expB;
    if expDiff == 0 {
        if expA == 0x7ff as i32 as i64 {
            if sigA | sigB != 0 {
                current_block = 10395241578169512789;
            } else {
                softfloat_raiseFlags(
                    softfloat_flag_invalid as i32 as uint_fast8_t,
                );
                uiZ = 0xfff8000000000000 as u64;
                current_block = 13099461109245191705;
            }
        } else {
            sigDiff = sigA.wrapping_sub(sigB) as int_fast64_t;
            if sigDiff == 0 {
                uiZ = (((softfloat_roundingMode as i32
                    == softfloat_round_min as i32) as i32
                    as uint_fast64_t) << 63 as i32)
                    .wrapping_add(
                        (0 as i32 as uint_fast64_t) << 52 as i32,
                    )
                    .wrapping_add(0 as i32 as u64);
            } else {
                if expA != 0 {
                    expA -= 1;
                }
                if sigDiff < 0 as i32 as i64 {
                    signZ = !signZ;
                    sigDiff = -sigDiff;
                }
                shiftDist = (softfloat_countLeadingZeros64(sigDiff as uint64_t)
                    as i32 - 11 as i32) as int_fast8_t;
                expZ = expA - shiftDist as i64;
                if expZ < 0 as i32 as i64 {
                    shiftDist = expA as int_fast8_t;
                    expZ = 0 as i32 as int_fast16_t;
                }
                uiZ = ((signZ as uint_fast64_t) << 63 as i32)
                    .wrapping_add((expZ as uint_fast64_t) << 52 as i32)
                    .wrapping_add(
                        (sigDiff << shiftDist as i32) as u64,
                    );
            }
            current_block = 13099461109245191705;
        }
    } else {
        sigA <<= 10 as i32;
        sigB <<= 10 as i32;
        if expDiff < 0 as i32 as i64 {
            signZ = !signZ;
            if expB == 0x7ff as i32 as i64 {
                if sigB != 0 {
                    current_block = 10395241578169512789;
                } else {
                    uiZ = ((signZ as uint_fast64_t) << 63 as i32)
                        .wrapping_add(
                            (0x7ff as i32 as uint_fast64_t) << 52 as i32,
                        )
                        .wrapping_add(0 as i32 as u64);
                    current_block = 13099461109245191705;
                }
            } else {
                sigA = (sigA as u64)
                    .wrapping_add(
                        if expA != 0 {
                            0x4000000000000000 as u64
                        } else {
                            sigA
                        },
                    ) as uint_fast64_t as uint_fast64_t;
                sigA = softfloat_shiftRightJam64(sigA, -expDiff as uint_fast32_t);
                sigB |= 0x4000000000000000 as u64;
                expZ = expB;
                sigZ = sigB.wrapping_sub(sigA);
                current_block = 6450597802325118133;
            }
        } else if expA == 0x7ff as i32 as i64 {
            if sigA != 0 {
                current_block = 10395241578169512789;
            } else {
                uiZ = uiA;
                current_block = 13099461109245191705;
            }
        } else {
            sigB = (sigB as u64)
                .wrapping_add(
                    if expB != 0 { 0x4000000000000000 as u64 } else { sigB },
                ) as uint_fast64_t as uint_fast64_t;
            sigB = softfloat_shiftRightJam64(sigB, expDiff as uint_fast32_t);
            sigA |= 0x4000000000000000 as u64;
            expZ = expA;
            sigZ = sigA.wrapping_sub(sigB);
            current_block = 6450597802325118133;
        }
        match current_block {
            13099461109245191705 => {}
            10395241578169512789 => {}
            _ => {
                return softfloat_normRoundPackToF64(
                    signZ,
                    expZ - 1 as i32 as i64,
                    sigZ,
                );
            }
        }
    }
    match current_block {
        10395241578169512789 => {
            uiZ = softfloat_propagateNaNF64UI(uiA, uiB);
        }
        _ => {}
    }
    uZ.ui = uiZ;
    return uZ.f;
}
