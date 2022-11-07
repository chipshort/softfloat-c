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
#[inline]
unsafe fn softfloat_shortShiftRightJam128Extra(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut extra: uint64_t,
    mut dist: uint_fast8_t,
) -> uint128_extra {
    let mut negDist: uint_fast8_t = -(dist as i32) as uint_fast8_t;
    let mut z: uint128_extra = uint128_extra {
        extra: 0,
        v: uint128 { v0: 0, v64: 0 },
    };
    z.v.v64 = a64 >> dist as i32;
    z
        .v
        .v0 = a64 << (negDist as i32 & 63 as i32)
        | a0 >> dist as i32;
    z
        .extra = a0 << (negDist as i32 & 63 as i32)
        | (extra != 0 as i32 as u64) as i32 as u64;
    return z;
}
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
pub unsafe fn softfloat_addMagsF128(
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
    let mut expDiff: int_fast32_t = 0;
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut sigZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut expZ: int_fast32_t = 0;
    let mut sigZExtra: uint_fast64_t = 0;
    let mut sig128Extra: uint128_extra = uint128_extra {
        extra: 0,
        v: uint128 { v0: 0, v64: 0 },
    };
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
    expDiff = expA - expB;
    if expDiff == 0 {
        if expA == 0x7fff as i32 as i64 {
            if sigA.v64 | sigA.v0 | sigB.v64 | sigB.v0 != 0 {
                current_block = 2137407365062195086;
            } else {
                uiZ.v64 = uiA64;
                uiZ.v0 = uiA0;
                current_block = 5256967698815673647;
            }
        } else {
            sigZ = softfloat_add128(sigA.v64, sigA.v0, sigB.v64, sigB.v0);
            if expA == 0 {
                uiZ
                    .v64 = ((signZ as uint_fast64_t) << 63 as i32)
                    .wrapping_add(
                        (0 as i32 as uint_fast64_t) << 48 as i32,
                    )
                    .wrapping_add(sigZ.v64);
                uiZ.v0 = sigZ.v0;
                current_block = 5256967698815673647;
            } else {
                expZ = expA;
                sigZ.v64 |= 0x2000000000000 as u64;
                sigZExtra = 0 as i32 as uint_fast64_t;
                current_block = 13514675851677366800;
            }
        }
    } else {
        if expDiff < 0 as i32 as i64 {
            if expB == 0x7fff as i32 as i64 {
                if sigB.v64 | sigB.v0 != 0 {
                    current_block = 2137407365062195086;
                } else {
                    uiZ
                        .v64 = ((signZ as uint_fast64_t) << 63 as i32)
                        .wrapping_add(
                            (0x7fff as i32 as uint_fast64_t) << 48 as i32,
                        )
                        .wrapping_add(0 as i32 as u64);
                    uiZ.v0 = 0 as i32 as uint64_t;
                    current_block = 5256967698815673647;
                }
            } else {
                expZ = expB;
                if expA != 0 {
                    sigA.v64 |= 0x1000000000000 as u64;
                    current_block = 14648156034262866959;
                } else {
                    expDiff += 1;
                    sigZExtra = 0 as i32 as uint_fast64_t;
                    if expDiff == 0 {
                        current_block = 13620120285487554387;
                    } else {
                        current_block = 14648156034262866959;
                    }
                }
                match current_block {
                    13620120285487554387 => {}
                    _ => {
                        sig128Extra = softfloat_shiftRightJam128Extra(
                            sigA.v64,
                            sigA.v0,
                            0 as i32 as uint64_t,
                            -expDiff as uint_fast32_t,
                        );
                        sigA = sig128Extra.v;
                        sigZExtra = sig128Extra.extra;
                        current_block = 13620120285487554387;
                    }
                }
            }
        } else if expA == 0x7fff as i32 as i64 {
            if sigA.v64 | sigA.v0 != 0 {
                current_block = 2137407365062195086;
            } else {
                uiZ.v64 = uiA64;
                uiZ.v0 = uiA0;
                current_block = 5256967698815673647;
            }
        } else {
            expZ = expA;
            if expB != 0 {
                sigB.v64 |= 0x1000000000000 as u64;
                current_block = 15090052786889560393;
            } else {
                expDiff -= 1;
                sigZExtra = 0 as i32 as uint_fast64_t;
                if expDiff == 0 {
                    current_block = 13620120285487554387;
                } else {
                    current_block = 15090052786889560393;
                }
            }
            match current_block {
                13620120285487554387 => {}
                _ => {
                    sig128Extra = softfloat_shiftRightJam128Extra(
                        sigB.v64,
                        sigB.v0,
                        0 as i32 as uint64_t,
                        expDiff as uint_fast32_t,
                    );
                    sigB = sig128Extra.v;
                    sigZExtra = sig128Extra.extra;
                    current_block = 13620120285487554387;
                }
            }
        }
        match current_block {
            2137407365062195086 => {}
            5256967698815673647 => {}
            _ => {
                sigZ = softfloat_add128(
                    sigA.v64 | 0x1000000000000 as u64,
                    sigA.v0,
                    sigB.v64,
                    sigB.v0,
                );
                expZ -= 1;
                if sigZ.v64 < 0x2000000000000 as u64 {
                    current_block = 12437542929250855829;
                } else {
                    expZ += 1;
                    current_block = 13514675851677366800;
                }
            }
        }
    }
    match current_block {
        13514675851677366800 => {
            sig128Extra = softfloat_shortShiftRightJam128Extra(
                sigZ.v64,
                sigZ.v0,
                sigZExtra,
                1 as i32 as uint_fast8_t,
            );
            sigZ = sig128Extra.v;
            sigZExtra = sig128Extra.extra;
            current_block = 12437542929250855829;
        }
        2137407365062195086 => {
            uiZ = softfloat_propagateNaNF128UI(uiA64, uiA0, uiB64, uiB0);
            current_block = 5256967698815673647;
        }
        _ => {}
    }
    match current_block {
        12437542929250855829 => {
            return softfloat_roundPackToF128(signZ, expZ, sigZ.v64, sigZ.v0, sigZExtra);
        }
        _ => {
            uZ.ui = uiZ;
            return uZ.f;
        }
    };
}
