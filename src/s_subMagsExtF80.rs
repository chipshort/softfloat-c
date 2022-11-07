use crate::*;

pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type int_fast32_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;


pub type extFloat80_t = extFloat80M;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s: extFloat80M,
    pub f: extFloat80_t,
}
pub const softfloat_round_min: C2RustUnnamed_0 = 2;
pub const softfloat_flag_invalid: C2RustUnnamed_1 = 16;
pub type C2RustUnnamed_0 = u32;
pub const softfloat_round_odd: C2RustUnnamed_0 = 6;
pub const softfloat_round_near_maxMag: C2RustUnnamed_0 = 4;
pub const softfloat_round_max: C2RustUnnamed_0 = 3;
pub const softfloat_round_minMag: C2RustUnnamed_0 = 1;
pub const softfloat_round_near_even: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = u32;
pub const softfloat_flag_infinite: C2RustUnnamed_1 = 8;
pub const softfloat_flag_overflow: C2RustUnnamed_1 = 4;
pub const softfloat_flag_underflow: C2RustUnnamed_1 = 2;
pub const softfloat_flag_inexact: C2RustUnnamed_1 = 1;
#[inline]
unsafe fn softfloat_sub128(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut b64: uint64_t,
    mut b0: uint64_t,
) -> uint128 {
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    z.v0 = a0.wrapping_sub(b0);
    z.v64 = a64.wrapping_sub(b64);
    z
        .v64 = (z.v64 as u64)
        .wrapping_sub((a0 < b0) as i32 as u64) as uint64_t as uint64_t;
    return z;
}
pub unsafe fn softfloat_subMagsExtF80(
    mut uiA64: uint_fast16_t,
    mut uiA0: uint_fast64_t,
    mut uiB64: uint_fast16_t,
    mut uiB0: uint_fast64_t,
    mut signZ: bool,
) -> extFloat80_t {
    let mut current_block: u64;
    let mut expA: int_fast32_t = 0;
    let mut sigA: uint_fast64_t = 0;
    let mut expB: int_fast32_t = 0;
    let mut sigB: uint_fast64_t = 0;
    let mut expDiff: int_fast32_t = 0;
    let mut uiZ64: uint_fast16_t = 0;
    let mut uiZ0: uint_fast64_t = 0;
    let mut expZ: int_fast32_t = 0;
    let mut sigExtra: uint_fast64_t = 0;
    let mut sig128: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uZ: C2RustUnnamed = C2RustUnnamed {
        s: extFloat80M {
            signif: 0,
            signExp: 0,
        },
    };
    expA = (uiA64 & 0x7fff as i32 as u64) as int_fast32_t;
    sigA = uiA0;
    expB = (uiB64 & 0x7fff as i32 as u64) as int_fast32_t;
    sigB = uiB0;
    expDiff = expA - expB;
    if (0 as i32 as i64) < expDiff {
        if expA == 0x7fff as i32 as i64 {
            if sigA & 0x7fffffffffffffff as u64 != 0 {
                current_block = 16253057913240697181;
            } else {
                uiZ64 = uiA64;
                uiZ0 = uiA0;
                current_block = 17711383428705500230;
            }
        } else {
            if expB == 0 {
                expDiff -= 1;
                sigExtra = 0 as i32 as uint_fast64_t;
                if expDiff == 0 {
                    current_block = 1407272662424433031;
                } else {
                    current_block = 6450597802325118133;
                }
            } else {
                current_block = 6450597802325118133;
            }
            match current_block {
                6450597802325118133 => {
                    sig128 = softfloat_shiftRightJam128(
                        sigB,
                        0 as i32 as uint64_t,
                        expDiff as uint_fast32_t,
                    );
                    sigB = sig128.v64;
                    sigExtra = sig128.v0;
                }
                _ => {}
            }
            expZ = expA;
            current_block = 1383913927575805312;
        }
    } else {
        if expDiff < 0 as i32 as i64 {
            if expB == 0x7fff as i32 as i64 {
                if sigB & 0x7fffffffffffffff as u64 != 0 {
                    current_block = 16253057913240697181;
                } else {
                    uiZ64 = ((signZ as i32 ^ 1 as i32) as uint_fast16_t)
                        << 15 as i32 | 0x7fff as i32 as u64;
                    uiZ0 = 0x8000000000000000 as u64;
                    current_block = 17711383428705500230;
                }
            } else {
                if expA == 0 {
                    expDiff += 1;
                    sigExtra = 0 as i32 as uint_fast64_t;
                    if expDiff == 0 {
                        current_block = 5439833222536683847;
                    } else {
                        current_block = 13550086250199790493;
                    }
                } else {
                    current_block = 13550086250199790493;
                }
                match current_block {
                    13550086250199790493 => {
                        sig128 = softfloat_shiftRightJam128(
                            sigA,
                            0 as i32 as uint64_t,
                            -expDiff as uint_fast32_t,
                        );
                        sigA = sig128.v64;
                        sigExtra = sig128.v0;
                    }
                    _ => {}
                }
                expZ = expB;
                current_block = 8341321261953370208;
            }
        } else if expA == 0x7fff as i32 as i64 {
            if (sigA | sigB) & 0x7fffffffffffffff as u64 != 0 {
                current_block = 16253057913240697181;
            } else {
                softfloat_raiseFlags(
                    softfloat_flag_invalid as i32 as uint_fast8_t,
                );
                uiZ64 = 0xffff as i32 as uint_fast16_t;
                uiZ0 = 0xc000000000000000 as u64;
                current_block = 17711383428705500230;
            }
        } else {
            expZ = expA;
            if expZ == 0 {
                expZ = 1 as i32 as int_fast32_t;
            }
            sigExtra = 0 as i32 as uint_fast64_t;
            if sigB < sigA {
                current_block = 1383913927575805312;
            } else if sigA < sigB {
                current_block = 8341321261953370208;
            } else {
                uiZ64 = ((softfloat_roundingMode as i32
                    == softfloat_round_min as i32) as i32
                    as uint_fast16_t) << 15 as i32
                    | 0 as i32 as u64;
                uiZ0 = 0 as i32 as uint_fast64_t;
                current_block = 17711383428705500230;
            }
        }
        match current_block {
            16253057913240697181 => {}
            17711383428705500230 => {}
            1383913927575805312 => {}
            _ => {
                signZ = !signZ;
                sig128 = softfloat_sub128(
                    sigB,
                    0 as i32 as uint64_t,
                    sigA,
                    sigExtra,
                );
                current_block = 13723621432742172895;
            }
        }
    }
    match current_block {
        16253057913240697181 => {
            uiZ = softfloat_propagateNaNExtF80UI(uiA64, uiA0, uiB64, uiB0);
            uiZ64 = uiZ.v64;
            uiZ0 = uiZ.v0;
            current_block = 17711383428705500230;
        }
        1383913927575805312 => {
            sig128 = softfloat_sub128(
                sigA,
                0 as i32 as uint64_t,
                sigB,
                sigExtra,
            );
            current_block = 13723621432742172895;
        }
        _ => {}
    }
    match current_block {
        13723621432742172895 => {
            return softfloat_normRoundPackToExtF80(
                signZ,
                expZ,
                sig128.v64,
                sig128.v0,
                extF80_roundingPrecision,
            );
        }
        _ => {
            uZ.s.signExp = uiZ64 as uint16_t;
            uZ.s.signif = uiZ0;
            return uZ.f;
        }
    };
}
