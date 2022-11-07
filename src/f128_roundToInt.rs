use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast32_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast64_t = u64;


#[derive(Copy, Clone)]
#[repr(C)]
pub union ui128_f128 {
    pub ui: uint128,
    pub f: float128_t,
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
#[inline]
unsafe fn softfloat_add128(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut b64: uint64_t,
    mut b0: uint64_t,
) -> uint128 {
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    z.v0 = a0.wrapping_add(b0);
    z
        .v64 = a64
        .wrapping_add(b64)
        .wrapping_add((z.v0 < a0) as i32 as u64);
    return z;
}
pub unsafe fn f128_roundToInt(
    mut a: float128_t,
    mut roundingMode: uint_fast8_t,
    mut exact: bool,
) -> float128_t {
    let mut current_block: u64;
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiA64: uint_fast64_t = 0;
    let mut uiA0: uint_fast64_t = 0;
    let mut exp: int_fast32_t = 0;
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut lastBitMask0: uint_fast64_t = 0;
    let mut roundBitsMask: uint_fast64_t = 0;
    let mut roundNearEven: bool = false;
    let mut lastBitMask64: uint_fast64_t = 0;
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    exp = (uiA64 >> 48 as i32) as int_fast32_t
        & 0x7fff as i32 as i64;
    if 0x402f as i32 as i64 <= exp {
        if 0x406f as i32 as i64 <= exp {
            if exp == 0x7fff as i32 as i64
                && uiA64 & 0xffffffffffff as u64 | uiA0 != 0
            {
                uiZ = softfloat_propagateNaNF128UI(
                    uiA64,
                    uiA0,
                    0 as i32 as uint_fast64_t,
                    0 as i32 as uint_fast64_t,
                );
            } else {
                return a
            }
            current_block = 14887321147285320204;
        } else {
            lastBitMask0 = (2 as i32 as uint_fast64_t)
                << 0x406e as i32 as i64 - exp;
            roundBitsMask = lastBitMask0.wrapping_sub(1 as i32 as u64);
            uiZ.v64 = uiA64;
            uiZ.v0 = uiA0;
            roundNearEven = roundingMode as i32
                == softfloat_round_near_even as i32;
            if roundNearEven as i32 != 0
                || roundingMode as i32
                    == softfloat_round_near_maxMag as i32
            {
                if exp == 0x402f as i32 as i64 {
                    if 0x8000000000000000 as u64 <= uiZ.v0 {
                        uiZ.v64 = (uiZ.v64).wrapping_add(1);
                        if roundNearEven as i32 != 0
                            && uiZ.v0 == 0x8000000000000000 as u64
                        {
                            uiZ.v64 &= !(1 as i32) as u64;
                        }
                    }
                } else {
                    uiZ = softfloat_add128(
                        uiZ.v64,
                        uiZ.v0,
                        0 as i32 as uint64_t,
                        lastBitMask0 >> 1 as i32,
                    );
                    if roundNearEven as i32 != 0 && uiZ.v0 & roundBitsMask == 0 {
                        uiZ.v0 &= !lastBitMask0;
                    }
                }
            } else if roundingMode as i32
                    == (if uiZ.v64 >> 63 as i32 != 0 {
                        softfloat_round_min as i32
                    } else {
                        softfloat_round_max as i32
                    })
                {
                uiZ = softfloat_add128(
                    uiZ.v64,
                    uiZ.v0,
                    0 as i32 as uint64_t,
                    roundBitsMask,
                );
            }
            uiZ.v0 &= !roundBitsMask;
            lastBitMask64 = (lastBitMask0 == 0) as i32 as uint_fast64_t;
            current_block = 8835654301469918283;
        }
    } else if exp < 0x3fff as i32 as i64 {
        if uiA64 & 0x7fffffffffffffff as u64 | uiA0 == 0 {
            return a;
        }
        if exact {
            softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                | softfloat_flag_inexact as i32) as uint_fast8_t;
        }
        uiZ
            .v64 = uiA64
            & ((1 as i32 as uint_fast64_t) << 63 as i32)
                .wrapping_add((0 as i32 as uint_fast64_t) << 48 as i32)
                .wrapping_add(0 as i32 as u64);
        uiZ.v0 = 0 as i32 as uint64_t;
        let mut current_block_43: u64;
        match roundingMode as i32 {
            0 => {
                if uiA64 & 0xffffffffffff as u64 | uiA0 == 0 {
                    current_block_43 = 14447253356787937536;
                } else {
                    current_block_43 = 15074722147828535541;
                }
            }
            4 => {
                current_block_43 = 15074722147828535541;
            }
            2 => {
                if uiZ.v64 != 0 {
                    uiZ
                        .v64 = ((1 as i32 as uint_fast64_t) << 63 as i32)
                        .wrapping_add(
                            (0x3fff as i32 as uint_fast64_t) << 48 as i32,
                        )
                        .wrapping_add(0 as i32 as u64);
                }
                current_block_43 = 14447253356787937536;
            }
            3 => {
                if uiZ.v64 == 0 {
                    uiZ
                        .v64 = ((0 as i32 as uint_fast64_t) << 63 as i32)
                        .wrapping_add(
                            (0x3fff as i32 as uint_fast64_t) << 48 as i32,
                        )
                        .wrapping_add(0 as i32 as u64);
                }
                current_block_43 = 14447253356787937536;
            }
            6 => {
                uiZ.v64
                    |= ((0 as i32 as uint_fast64_t) << 63 as i32)
                        .wrapping_add(
                            (0x3fff as i32 as uint_fast64_t) << 48 as i32,
                        )
                        .wrapping_add(0 as i32 as u64);
                current_block_43 = 14447253356787937536;
            }
            _ => {
                current_block_43 = 14447253356787937536;
            }
        }
        match current_block_43 {
            15074722147828535541 => {
                if exp == 0x3ffe as i32 as i64 {
                    uiZ.v64
                        |= ((0 as i32 as uint_fast64_t) << 63 as i32)
                            .wrapping_add(
                                (0x3fff as i32 as uint_fast64_t)
                                    << 48 as i32,
                            )
                            .wrapping_add(0 as i32 as u64);
                }
            }
            _ => {}
        }
        current_block = 14887321147285320204;
    } else {
        uiZ.v64 = uiA64;
        uiZ.v0 = 0 as i32 as uint64_t;
        lastBitMask64 = (1 as i32 as uint_fast64_t)
            << 0x402f as i32 as i64 - exp;
        roundBitsMask = lastBitMask64.wrapping_sub(1 as i32 as u64);
        if roundingMode as i32 == softfloat_round_near_maxMag as i32 {
            uiZ
                .v64 = (uiZ.v64 as u64)
                .wrapping_add(lastBitMask64 >> 1 as i32) as uint64_t as uint64_t;
        } else if roundingMode as i32 == softfloat_round_near_even as i32
            {
            uiZ
                .v64 = (uiZ.v64 as u64)
                .wrapping_add(lastBitMask64 >> 1 as i32) as uint64_t as uint64_t;
            if uiZ.v64 & roundBitsMask | uiA0 == 0 {
                uiZ.v64 &= !lastBitMask64;
            }
        } else if roundingMode as i32
                == (if uiZ.v64 >> 63 as i32 != 0 {
                    softfloat_round_min as i32
                } else {
                    softfloat_round_max as i32
                })
            {
            uiZ
                .v64 = (uiZ.v64
                | (uiA0 != 0 as i32 as u64) as i32
                    as u64)
                .wrapping_add(roundBitsMask);
        }
        uiZ.v64 &= !roundBitsMask;
        lastBitMask0 = 0 as i32 as uint_fast64_t;
        current_block = 8835654301469918283;
    }
    match current_block {
        8835654301469918283 => {
            if uiZ.v64 != uiA64 || uiZ.v0 != uiA0 {
                if roundingMode as i32 == softfloat_round_odd as i32 {
                    uiZ.v64 |= lastBitMask64;
                    uiZ.v0 |= lastBitMask0;
                }
                if exact {
                    softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                        | softfloat_flag_inexact as i32) as uint_fast8_t;
                }
            }
        }
        _ => {}
    }
    uZ.ui = uiZ;
    return uZ.f;
}
