use crate::*;

pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type int_fast8_t = i8;
pub type int_fast16_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast32_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui16_f16 {
    pub ui: uint16_t,
    pub f: float16_t,
}
pub const softfloat_flag_inexact: C2RustUnnamed_0 = 1;
pub const softfloat_round_odd: C2RustUnnamed = 6;
pub const softfloat_round_min: C2RustUnnamed = 2;
pub const softfloat_round_max: C2RustUnnamed = 3;
pub const softfloat_round_minMag: C2RustUnnamed = 1;
pub const softfloat_round_near_even: C2RustUnnamed = 0;
pub const softfloat_flag_invalid: C2RustUnnamed_0 = 16;
pub type C2RustUnnamed = u32;
pub const softfloat_round_near_maxMag: C2RustUnnamed = 4;
pub type C2RustUnnamed_0 = u32;
pub const softfloat_flag_infinite: C2RustUnnamed_0 = 8;
pub const softfloat_flag_overflow: C2RustUnnamed_0 = 4;
pub const softfloat_flag_underflow: C2RustUnnamed_0 = 2;
#[inline]
unsafe fn softfloat_countLeadingZeros16(mut a: uint16_t) -> uint_fast8_t {
    return (if a as i32 != 0 {
        (a as u32).leading_zeros() as i32 - 16 as i32
    } else {
        16 as i32
    }) as uint_fast8_t;
}
#[inline]
unsafe fn softfloat_countLeadingZeros32(mut a: uint32_t) -> uint_fast8_t {
    return (if a != 0 { a.leading_zeros() as i32 } else { 32 as i32 })
        as uint_fast8_t;
}
pub unsafe fn softfloat_subMagsF16(
    mut uiA: uint_fast16_t,
    mut uiB: uint_fast16_t,
) -> float16_t {
    let mut current_block: u64;
    let mut expA: int_fast8_t = 0;
    let mut sigA: uint_fast16_t = 0;
    let mut expB: int_fast8_t = 0;
    let mut sigB: uint_fast16_t = 0;
    let mut expDiff: int_fast8_t = 0;
    let mut uiZ: uint_fast16_t = 0;
    let mut sigDiff: int_fast16_t = 0;
    let mut signZ: bool = false;
    let mut shiftDist: int_fast8_t = 0;
    let mut expZ: int_fast8_t = 0;
    let mut sigZ: uint_fast16_t = 0;
    let mut sigX: uint_fast16_t = 0;
    let mut sigY: uint_fast16_t = 0;
    let mut sig32Z: uint_fast32_t = 0;
    let mut roundingMode: int_fast8_t = 0;
    let mut uZ: ui16_f16 = ui16_f16 { ui: 0 };
    expA = ((uiA >> 10 as i32) as int_fast8_t as i32
        & 0x1f as i32) as int_fast8_t;
    sigA = uiA & 0x3ff as i32 as u64;
    expB = ((uiB >> 10 as i32) as int_fast8_t as i32
        & 0x1f as i32) as int_fast8_t;
    sigB = uiB & 0x3ff as i32 as u64;
    expDiff = (expA as i32 - expB as i32) as int_fast8_t;
    if expDiff == 0 {
        if expA as i32 == 0x1f as i32 {
            if sigA | sigB != 0 {
                current_block = 7659099340699071685;
            } else {
                softfloat_raiseFlags(
                    softfloat_flag_invalid as i32 as uint_fast8_t,
                );
                uiZ = 0xfe00 as i32 as uint_fast16_t;
                current_block = 4392969755685924400;
            }
        } else {
            sigDiff = sigA.wrapping_sub(sigB) as int_fast16_t;
            if sigDiff == 0 {
                uiZ = ((((softfloat_roundingMode as i32
                    == softfloat_round_min as i32) as i32 as uint16_t
                    as i32) << 15 as i32)
                    + ((0 as i32 as uint16_t as i32)
                        << 10 as i32) + 0 as i32) as uint_fast16_t;
                current_block = 4392969755685924400;
            } else {
                if expA != 0 {
                    expA -= 1;
                }
                signZ = uiA as uint16_t as i32 >> 15 as i32 != 0;
                if sigDiff < 0 as i32 as i64 {
                    signZ = !signZ;
                    sigDiff = -sigDiff;
                }
                shiftDist = (softfloat_countLeadingZeros16(sigDiff as uint16_t)
                    as i32 - 5 as i32) as int_fast8_t;
                expZ = (expA as i32 - shiftDist as i32) as int_fast8_t;
                if (expZ as i32) < 0 as i32 {
                    shiftDist = expA;
                    expZ = 0 as i32 as int_fast8_t;
                }
                sigZ = (sigDiff << shiftDist as i32) as uint_fast16_t;
                current_block = 13832762799486176136;
            }
        }
    } else {
        signZ = uiA as uint16_t as i32 >> 15 as i32 != 0;
        if (expDiff as i32) < 0 as i32 {
            signZ = !signZ;
            if expB as i32 == 0x1f as i32 {
                if sigB != 0 {
                    current_block = 7659099340699071685;
                } else {
                    uiZ = (((signZ as uint16_t as i32) << 15 as i32)
                        + ((0x1f as i32 as uint16_t as i32)
                            << 10 as i32) + 0 as i32) as uint_fast16_t;
                    current_block = 4392969755685924400;
                }
            } else if expDiff as i32 <= -(13 as i32) {
                uiZ = ((((signZ as uint16_t as i32) << 15 as i32)
                    + ((expB as uint16_t as i32) << 10 as i32))
                    as u64)
                    .wrapping_add(sigB);
                if expA as u64 | sigA != 0 {
                    current_block = 14653906710034680068;
                } else {
                    current_block = 4392969755685924400;
                }
            } else {
                expZ = (expA as i32 + 19 as i32) as int_fast8_t;
                sigX = sigB | 0x400 as i32 as u64;
                sigY = sigA
                    .wrapping_add(
                        (if expA as i32 != 0 {
                            0x400 as i32 as u64
                        } else {
                            sigA
                        }),
                    );
                expDiff = -(expDiff as i32) as int_fast8_t;
                current_block = 1622411330066726685;
            }
        } else {
            uiZ = uiA;
            if expA as i32 == 0x1f as i32 {
                if sigA != 0 {
                    current_block = 7659099340699071685;
                } else {
                    current_block = 4392969755685924400;
                }
            } else if 13 as i32 <= expDiff as i32 {
                if expB as u64 | sigB != 0 {
                    current_block = 14653906710034680068;
                } else {
                    current_block = 4392969755685924400;
                }
            } else {
                expZ = (expB as i32 + 19 as i32) as int_fast8_t;
                sigX = sigA | 0x400 as i32 as u64;
                sigY = sigB
                    .wrapping_add(
                        (if expB as i32 != 0 {
                            0x400 as i32 as u64
                        } else {
                            sigB
                        }),
                    );
                current_block = 1622411330066726685;
            }
        }
        match current_block {
            4392969755685924400 => {}
            7659099340699071685 => {}
            _ => {
                match current_block {
                    14653906710034680068 => {
                        roundingMode = softfloat_roundingMode as int_fast8_t;
                        if roundingMode as i32
                            != softfloat_round_near_even as i32
                        {
                            if roundingMode as i32
                                == softfloat_round_minMag as i32
                                || roundingMode as i32
                                    == (if uiZ as uint16_t as i32 >> 15 as i32
                                        != 0
                                    {
                                        softfloat_round_max as i32
                                    } else {
                                        softfloat_round_min as i32
                                    })
                            {
                                uiZ = uiZ.wrapping_sub(1);
                            } else if roundingMode as i32
                                    == softfloat_round_odd as i32
                                {
                                uiZ = uiZ.wrapping_sub(1 as i32 as u64)
                                    | 1 as i32 as u64;
                            }
                        }
                        softfloat_exceptionFlags = (softfloat_exceptionFlags
                            as i32 | softfloat_flag_inexact as i32)
                            as uint_fast8_t;
                        current_block = 4392969755685924400;
                    }
                    _ => {
                        sig32Z = (sigX << expDiff as i32).wrapping_sub(sigY);
                        shiftDist = (softfloat_countLeadingZeros32(sig32Z as uint32_t)
                            as i32 - 1 as i32) as int_fast8_t;
                        sig32Z <<= shiftDist as i32;
                        expZ = (expZ as i32 - shiftDist as i32)
                            as int_fast8_t;
                        sigZ = sig32Z >> 16 as i32;
                        if sig32Z & 0xffff as i32 as u64 != 0 {
                            sigZ |= 1 as i32 as u64;
                            current_block = 5892776923941496671;
                        } else if sigZ & 0xf as i32 as u64 == 0
                                && (expZ as u32)
                                    < 0x1e as i32 as u32
                            {
                            sigZ >>= 4 as i32;
                            current_block = 13832762799486176136;
                        } else {
                            current_block = 5892776923941496671;
                        }
                        match current_block {
                            13832762799486176136 => {}
                            _ => {
                                return softfloat_roundPackToF16(
                                    signZ,
                                    expZ as int_fast16_t,
                                    sigZ,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        13832762799486176136 => {
            uiZ = ((((signZ as uint16_t as i32) << 15 as i32)
                + ((expZ as uint16_t as i32) << 10 as i32))
                as u64)
                .wrapping_add(sigZ);
        }
        7659099340699071685 => {
            uiZ = softfloat_propagateNaNF16UI(uiA, uiB);
        }
        _ => {}
    }
    uZ.ui = uiZ as uint16_t;
    return uZ.f;
}
