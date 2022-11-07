use crate::*;

pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type int_fast32_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast64_t = u64;


pub type extFloat80_t = extFloat80M;

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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub s: extFloat80M,
    pub f: extFloat80_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub s: extFloat80M,
    pub f: extFloat80_t,
}
pub unsafe fn extF80_roundToInt(
    mut a: extFloat80_t,
    mut roundingMode: uint_fast8_t,
    mut exact: bool,
) -> extFloat80_t {
    let mut current_block: u64;
    let mut uA: C2RustUnnamed_2 = C2RustUnnamed_2 {
        s: extFloat80_t {
            signif: 0,
            signExp: 0,
        },
    };
    let mut uiA64: uint_fast16_t = 0;
    let mut signUI64: uint_fast16_t = 0;
    let mut exp: int_fast32_t = 0;
    let mut sigA: uint_fast64_t = 0;
    let mut uiZ64: uint_fast16_t = 0;
    let mut sigZ: uint_fast64_t = 0;
    let mut normExpSig: exp32_sig64 = exp32_sig64 { exp: 0, sig: 0 };
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut lastBitMask: uint_fast64_t = 0;
    let mut roundBitsMask: uint_fast64_t = 0;
    let mut uZ: C2RustUnnamed_1 = C2RustUnnamed_1 {
        s: extFloat80_t {
            signif: 0,
            signExp: 0,
        },
    };
    uA.f = a;
    uiA64 = uA.s.signExp as uint_fast16_t;
    signUI64 = uiA64
        & ((1 as i32 as uint_fast16_t) << 15 as i32
            | 0 as i32 as u64);
    exp = (uiA64 & 0x7fff as i32 as u64) as int_fast32_t;
    sigA = uA.s.signif;
    if sigA & 0x8000000000000000 as u64 == 0
        && exp != 0x7fff as i32 as i64
    {
        if sigA == 0 {
            uiZ64 = signUI64;
            sigZ = 0 as i32 as uint_fast64_t;
            current_block = 2538641245582488177;
        } else {
            normExpSig = softfloat_normSubnormalExtF80Sig(sigA);
            exp += normExpSig.exp;
            sigA = normExpSig.sig;
            current_block = 1856101646708284338;
        }
    } else {
        current_block = 1856101646708284338;
    }
    match current_block {
        1856101646708284338 => {
            if 0x403e as i32 as i64 <= exp {
                if exp == 0x7fff as i32 as i64 {
                    if sigA & 0x7fffffffffffffff as u64 != 0 {
                        uiZ = softfloat_propagateNaNExtF80UI(
                            uiA64,
                            sigA,
                            0 as i32 as uint_fast16_t,
                            0 as i32 as uint_fast64_t,
                        );
                        uiZ64 = uiZ.v64;
                        sigZ = uiZ.v0;
                        current_block = 2538641245582488177;
                    } else {
                        sigZ = 0x8000000000000000 as u64;
                        current_block = 13797916685926291137;
                    }
                } else {
                    sigZ = sigA;
                    current_block = 13797916685926291137;
                }
                match current_block {
                    2538641245582488177 => {}
                    _ => {
                        uiZ64 = signUI64 | exp as u64;
                    }
                }
            } else if exp <= 0x3ffe as i32 as i64 {
                if exact {
                    softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                        | softfloat_flag_inexact as i32) as uint_fast8_t;
                }
                match roundingMode as i32 {
                    0 => {
                        if sigA & 0x7fffffffffffffff as u64 == 0 {
                            current_block = 2891135413264362348;
                        } else {
                            current_block = 14323931891669749092;
                        }
                    }
                    4 => {
                        current_block = 14323931891669749092;
                    }
                    2 => {
                        if signUI64 != 0 {
                            current_block = 576658720679080422;
                        } else {
                            current_block = 2891135413264362348;
                        }
                    }
                    3 => {
                        if signUI64 == 0 {
                            current_block = 576658720679080422;
                        } else {
                            current_block = 2891135413264362348;
                        }
                    }
                    6 => {
                        current_block = 576658720679080422;
                    }
                    _ => {
                        current_block = 2891135413264362348;
                    }
                }
                match current_block {
                    14323931891669749092 => {
                        if exp == 0x3ffe as i32 as i64 {
                            current_block = 576658720679080422;
                        } else {
                            current_block = 2891135413264362348;
                        }
                    }
                    _ => {}
                }
                match current_block {
                    576658720679080422 => {
                        uiZ64 = signUI64 | 0x3fff as i32 as u64;
                        sigZ = 0x8000000000000000 as u64;
                    }
                    _ => {
                        uiZ64 = signUI64;
                        sigZ = 0 as i32 as uint_fast64_t;
                    }
                }
            } else {
                uiZ64 = signUI64 | exp as u64;
                lastBitMask = (1 as i32 as uint_fast64_t)
                    << 0x403e as i32 as i64 - exp;
                roundBitsMask = lastBitMask
                    .wrapping_sub(1 as i32 as u64);
                sigZ = sigA;
                if roundingMode as i32
                    == softfloat_round_near_maxMag as i32
                {
                    sigZ = (sigZ as u64)
                        .wrapping_add(lastBitMask >> 1 as i32) as uint_fast64_t
                        as uint_fast64_t;
                } else if roundingMode as i32
                        == softfloat_round_near_even as i32
                    {
                    sigZ = (sigZ as u64)
                        .wrapping_add(lastBitMask >> 1 as i32) as uint_fast64_t
                        as uint_fast64_t;
                    if sigZ & roundBitsMask == 0 {
                        sigZ &= !lastBitMask;
                    }
                } else if roundingMode as i32
                        == (if signUI64 != 0 {
                            softfloat_round_min as i32
                        } else {
                            softfloat_round_max as i32
                        })
                    {
                    sigZ = (sigZ as u64).wrapping_add(roundBitsMask)
                        as uint_fast64_t as uint_fast64_t;
                }
                sigZ &= !roundBitsMask;
                if sigZ == 0 {
                    uiZ64 = uiZ64.wrapping_add(1);
                    sigZ = 0x8000000000000000 as u64;
                }
                if sigZ != sigA {
                    if roundingMode as i32 == softfloat_round_odd as i32
                    {
                        sigZ |= lastBitMask;
                    }
                    if exact {
                        softfloat_exceptionFlags = (softfloat_exceptionFlags
                            as i32 | softfloat_flag_inexact as i32)
                            as uint_fast8_t;
                    }
                }
            }
        }
        _ => {}
    }
    uZ.s.signExp = uiZ64 as uint16_t;
    uZ.s.signif = sigZ;
    return uZ.f;
}
