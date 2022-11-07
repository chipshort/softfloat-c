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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct uint64_extra {
    pub extra: uint64_t,
    pub v: uint64_t,
}

pub type extFloat80_t = extFloat80M;

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s: extFloat80M,
    pub f: extFloat80_t,
}
#[inline]
unsafe fn softfloat_shortShiftRightJam64Extra(
    mut a: uint64_t,
    mut extra: uint64_t,
    mut dist: uint_fast8_t,
) -> uint64_extra {
    let mut z: uint64_extra = uint64_extra { extra: 0, v: 0 };
    z.v = a >> dist as i32;
    z
        .extra = a << (-(dist as i32) & 63 as i32)
        | (extra != 0 as i32 as u64) as i32 as u64;
    return z;
}
#[inline]
unsafe fn softfloat_shiftRightJam64Extra(
    mut a: uint64_t,
    mut extra: uint64_t,
    mut dist: uint_fast32_t,
) -> uint64_extra {
    let mut z: uint64_extra = uint64_extra { extra: 0, v: 0 };
    if dist < 64 as i32 as u64 {
        z.v = a >> dist;
        z.extra = a << (dist.wrapping_neg() & 63 as i32 as u64);
    } else {
        z.v = 0 as i32 as uint64_t;
        z
            .extra = if dist == 64 as i32 as u64 {
            a
        } else {
            (a != 0 as i32 as u64) as i32 as u64
        };
    }
    z.extra
        |= (extra != 0 as i32 as u64) as i32 as u64;
    return z;
}
pub unsafe fn softfloat_addMagsExtF80(
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
    let mut sigZ: uint_fast64_t = 0;
    let mut sigZExtra: uint_fast64_t = 0;
    let mut normExpSig: exp32_sig64 = exp32_sig64 { exp: 0, sig: 0 };
    let mut expZ: int_fast32_t = 0;
    let mut sig64Extra: uint64_extra = uint64_extra { extra: 0, v: 0 };
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
    if expDiff == 0 {
        if expA == 0x7fff as i32 as i64 {
            if (sigA | sigB) & 0x7fffffffffffffff as u64 != 0 {
                current_block = 14679127422746781061;
            } else {
                uiZ64 = uiA64;
                uiZ0 = uiA0;
                current_block = 7194293357906229479;
            }
        } else {
            sigZ = sigA.wrapping_add(sigB);
            sigZExtra = 0 as i32 as uint_fast64_t;
            if expA == 0 {
                normExpSig = softfloat_normSubnormalExtF80Sig(sigZ);
                expZ = normExpSig.exp + 1 as i32 as i64;
                sigZ = normExpSig.sig;
                current_block = 3358143479494337190;
            } else {
                expZ = expA;
                current_block = 15982178851085216570;
            }
        }
    } else {
        if expDiff < 0 as i32 as i64 {
            if expB == 0x7fff as i32 as i64 {
                if sigB & 0x7fffffffffffffff as u64 != 0 {
                    current_block = 14679127422746781061;
                } else {
                    uiZ64 = (signZ as uint_fast16_t) << 15 as i32
                        | 0x7fff as i32 as u64;
                    uiZ0 = uiB0;
                    current_block = 7194293357906229479;
                }
            } else {
                expZ = expB;
                if expA == 0 {
                    expDiff += 1;
                    sigZExtra = 0 as i32 as uint_fast64_t;
                    if expDiff == 0 {
                        current_block = 16825437407839615934;
                    } else {
                        current_block = 7056779235015430508;
                    }
                } else {
                    current_block = 7056779235015430508;
                }
                match current_block {
                    16825437407839615934 => {}
                    _ => {
                        sig64Extra = softfloat_shiftRightJam64Extra(
                            sigA,
                            0 as i32 as uint64_t,
                            -expDiff as uint_fast32_t,
                        );
                        sigA = sig64Extra.v;
                        sigZExtra = sig64Extra.extra;
                        current_block = 16825437407839615934;
                    }
                }
            }
        } else if expA == 0x7fff as i32 as i64 {
            if sigA & 0x7fffffffffffffff as u64 != 0 {
                current_block = 14679127422746781061;
            } else {
                uiZ64 = uiA64;
                uiZ0 = uiA0;
                current_block = 7194293357906229479;
            }
        } else {
            expZ = expA;
            if expB == 0 {
                expDiff -= 1;
                sigZExtra = 0 as i32 as uint_fast64_t;
                if expDiff == 0 {
                    current_block = 16825437407839615934;
                } else {
                    current_block = 11048769245176032998;
                }
            } else {
                current_block = 11048769245176032998;
            }
            match current_block {
                16825437407839615934 => {}
                _ => {
                    sig64Extra = softfloat_shiftRightJam64Extra(
                        sigB,
                        0 as i32 as uint64_t,
                        expDiff as uint_fast32_t,
                    );
                    sigB = sig64Extra.v;
                    sigZExtra = sig64Extra.extra;
                    current_block = 16825437407839615934;
                }
            }
        }
        match current_block {
            14679127422746781061 => {}
            7194293357906229479 => {}
            _ => {
                sigZ = sigA.wrapping_add(sigB);
                if sigZ & 0x8000000000000000 as u64 != 0 {
                    current_block = 3358143479494337190;
                } else {
                    current_block = 15982178851085216570;
                }
            }
        }
    }
    match current_block {
        15982178851085216570 => {
            sig64Extra = softfloat_shortShiftRightJam64Extra(
                sigZ,
                sigZExtra,
                1 as i32 as uint_fast8_t,
            );
            sigZ = sig64Extra.v | 0x8000000000000000 as u64;
            sigZExtra = sig64Extra.extra;
            expZ += 1;
            current_block = 3358143479494337190;
        }
        14679127422746781061 => {
            uiZ = softfloat_propagateNaNExtF80UI(uiA64, uiA0, uiB64, uiB0);
            uiZ64 = uiZ.v64;
            uiZ0 = uiZ.v0;
            current_block = 7194293357906229479;
        }
        _ => {}
    }
    match current_block {
        3358143479494337190 => {
            return softfloat_roundPackToExtF80(
                signZ,
                expZ,
                sigZ,
                sigZExtra,
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
