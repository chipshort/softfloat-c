use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast32_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;


#[derive(Copy, Clone)]
#[repr(C)]
pub union ui128_f128 {
    pub ui: uint128,
    pub f: float128_t,
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
unsafe fn softfloat_shortShiftLeft128(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut dist: uint_fast8_t,
) -> uint128 {
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    z
        .v64 = a64 << dist as i32
        | a0 >> (-(dist as i32) & 63 as i32);
    z.v0 = a0 << dist as i32;
    return z;
}
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
pub unsafe fn softfloat_subMagsF128(
    mut uiA64: uint_fast64_t,
    mut uiA0: uint_fast64_t,
    mut uiB64: uint_fast64_t,
    mut uiB0: uint_fast64_t,
    mut signZ: bool,
) -> float128_t {
    let mut current_block: u64;
    let mut expA: int_fast32_t = 0;
    let mut sigA: uint128 = uint128 { v0: 0, v64: 0 };
    let mut expB: int_fast32_t = 0;
    let mut sigB: uint128 = uint128 { v0: 0, v64: 0 };
    let mut sigZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut expDiff: int_fast32_t = 0;
    let mut expZ: int_fast32_t = 0;
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    expA = (uiA64 >> 48 as i32) as int_fast32_t
        & 0x7fff as i32 as i64;
    sigA.v64 = uiA64 & 0xffffffffffff as u64;
    sigA.v0 = uiA0;
    expB = (uiB64 >> 48 as i32) as int_fast32_t
        & 0x7fff as i32 as i64;
    sigB.v64 = uiB64 & 0xffffffffffff as u64;
    sigB.v0 = uiB0;
    sigA = softfloat_shortShiftLeft128(
        sigA.v64,
        sigA.v0,
        4 as i32 as uint_fast8_t,
    );
    sigB = softfloat_shortShiftLeft128(
        sigB.v64,
        sigB.v0,
        4 as i32 as uint_fast8_t,
    );
    expDiff = expA - expB;
    if (0 as i32 as i64) < expDiff {
        if expA == 0x7fff as i32 as i64 {
            if sigA.v64 | sigA.v0 != 0 {
                current_block = 10005822445609561829;
            } else {
                uiZ.v64 = uiA64;
                uiZ.v0 = uiA0;
                current_block = 5385895327590831436;
            }
        } else {
            if expB != 0 {
                sigB.v64 |= 0x10000000000000 as u64;
                current_block = 12997042908615822766;
            } else {
                expDiff -= 1;
                if expDiff == 0 {
                    current_block = 9684676501875640691;
                } else {
                    current_block = 12997042908615822766;
                }
            }
            match current_block {
                12997042908615822766 => {
                    sigB = softfloat_shiftRightJam128(
                        sigB.v64,
                        sigB.v0,
                        expDiff as uint_fast32_t,
                    );
                }
                _ => {}
            }
            expZ = expA;
            sigA.v64 |= 0x10000000000000 as u64;
            current_block = 3389508712923644711;
        }
    } else {
        if expDiff < 0 as i32 as i64 {
            if expB == 0x7fff as i32 as i64 {
                if sigB.v64 | sigB.v0 != 0 {
                    current_block = 10005822445609561829;
                } else {
                    uiZ
                        .v64 = (((signZ as i32 ^ 1 as i32)
                        as uint_fast64_t) << 63 as i32)
                        .wrapping_add(
                            (0x7fff as i32 as uint_fast64_t) << 48 as i32,
                        )
                        .wrapping_add(0 as i32 as u64);
                    uiZ.v0 = 0 as i32 as uint64_t;
                    current_block = 5385895327590831436;
                }
            } else {
                if expA != 0 {
                    sigA.v64 |= 0x10000000000000 as u64;
                    current_block = 8693738493027456495;
                } else {
                    expDiff += 1;
                    if expDiff == 0 {
                        current_block = 17494425638149595497;
                    } else {
                        current_block = 8693738493027456495;
                    }
                }
                match current_block {
                    8693738493027456495 => {
                        sigA = softfloat_shiftRightJam128(
                            sigA.v64,
                            sigA.v0,
                            -expDiff as uint_fast32_t,
                        );
                    }
                    _ => {}
                }
                expZ = expB;
                sigB.v64 |= 0x10000000000000 as u64;
                current_block = 4675857737588259019;
            }
        } else if expA == 0x7fff as i32 as i64 {
            if sigA.v64 | sigA.v0 | sigB.v64 | sigB.v0 != 0 {
                current_block = 10005822445609561829;
            } else {
                softfloat_raiseFlags(
                    softfloat_flag_invalid as i32 as uint_fast8_t,
                );
                uiZ.v64 = 0xffff800000000000 as u64;
                uiZ.v0 = 0 as u64;
                current_block = 5385895327590831436;
            }
        } else {
            expZ = expA;
            if expZ == 0 {
                expZ = 1 as i32 as int_fast32_t;
            }
            if sigB.v64 < sigA.v64 {
                current_block = 3389508712923644711;
            } else if sigA.v64 < sigB.v64 {
                current_block = 4675857737588259019;
            } else if sigB.v0 < sigA.v0 {
                current_block = 3389508712923644711;
            } else if sigA.v0 < sigB.v0 {
                current_block = 4675857737588259019;
            } else {
                uiZ
                    .v64 = (((softfloat_roundingMode as i32
                    == softfloat_round_min as i32) as i32
                    as uint_fast64_t) << 63 as i32)
                    .wrapping_add(
                        (0 as i32 as uint_fast64_t) << 48 as i32,
                    )
                    .wrapping_add(0 as i32 as u64);
                uiZ.v0 = 0 as i32 as uint64_t;
                current_block = 5385895327590831436;
            }
        }
        match current_block {
            10005822445609561829 => {}
            5385895327590831436 => {}
            3389508712923644711 => {}
            _ => {
                signZ = !signZ;
                sigZ = softfloat_sub128(sigB.v64, sigB.v0, sigA.v64, sigA.v0);
                current_block = 6335226375177376187;
            }
        }
    }
    match current_block {
        10005822445609561829 => {
            uiZ = softfloat_propagateNaNF128UI(uiA64, uiA0, uiB64, uiB0);
            current_block = 5385895327590831436;
        }
        3389508712923644711 => {
            sigZ = softfloat_sub128(sigA.v64, sigA.v0, sigB.v64, sigB.v0);
            current_block = 6335226375177376187;
        }
        _ => {}
    }
    match current_block {
        6335226375177376187 => {
            return softfloat_normRoundPackToF128(
                signZ,
                expZ - 5 as i32 as i64,
                sigZ.v64,
                sigZ.v0,
            );
        }
        _ => {
            uZ.ui = uiZ;
            return uZ.f;
        }
    };
}
