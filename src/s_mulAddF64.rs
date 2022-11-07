use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast8_t = i8;
pub type int_fast16_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ui: u128,
    pub s: uint128,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui64_f64 {
    pub ui: uint64_t,
    pub f: float64_t,
}
pub type C2RustUnnamed_0 = u32;
pub const softfloat_mulAdd_subProd: C2RustUnnamed_0 = 2;
pub const softfloat_mulAdd_subC: C2RustUnnamed_0 = 1;

pub const softfloat_round_min: C2RustUnnamed_1 = 2;
pub const softfloat_flag_invalid: C2RustUnnamed_2 = 16;
pub type C2RustUnnamed_1 = u32;
pub const softfloat_round_odd: C2RustUnnamed_1 = 6;
pub const softfloat_round_near_maxMag: C2RustUnnamed_1 = 4;
pub const softfloat_round_max: C2RustUnnamed_1 = 3;
pub const softfloat_round_minMag: C2RustUnnamed_1 = 1;
pub const softfloat_round_near_even: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = u32;
pub const softfloat_flag_infinite: C2RustUnnamed_2 = 8;
pub const softfloat_flag_overflow: C2RustUnnamed_2 = 4;
pub const softfloat_flag_underflow: C2RustUnnamed_2 = 2;
pub const softfloat_flag_inexact: C2RustUnnamed_2 = 1;
#[inline]
unsafe fn softfloat_countLeadingZeros64(mut a: uint64_t) -> uint_fast8_t {
    return (if a != 0 {
        (a as u64).leading_zeros() as i32
    } else {
        64 as i32
    }) as uint_fast8_t;
}
#[inline]
unsafe fn softfloat_mul64To128(mut a: uint64_t, mut b: uint64_t) -> uint128 {
    let mut uZ: C2RustUnnamed = C2RustUnnamed { ui: 0 };
    uZ.ui = (a as u128).wrapping_mul(b as u128);
    return uZ.s;
}
#[inline]
unsafe fn softfloat_shortShiftRightJam64(
    mut a: uint64_t,
    mut dist: uint_fast8_t,
) -> uint64_t {
    return a >> dist as i32
        | (a
            & ((1 as i32 as uint_fast64_t) << dist as i32)
                .wrapping_sub(1 as i32 as u64)
            != 0 as i32 as u64) as i32 as u64;
}
#[inline]
unsafe fn softfloat_shortShiftRightJam128(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut dist: uint_fast8_t,
) -> uint128 {
    let mut negDist: uint_fast8_t = -(dist as i32) as uint_fast8_t;
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    z.v64 = a64 >> dist as i32;
    z
        .v0 = a64 << (negDist as i32 & 63 as i32)
        | a0 >> dist as i32
        | (a0 << (negDist as i32 & 63 as i32)
            != 0 as i32 as u64) as i32 as u64;
    return z;
}
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
pub unsafe fn softfloat_mulAddF64(
    mut uiA: uint_fast64_t,
    mut uiB: uint_fast64_t,
    mut uiC: uint_fast64_t,
    mut op: uint_fast8_t,
) -> float64_t {
    let mut current_block: u64;
    let mut signA: bool = false;
    let mut expA: int_fast16_t = 0;
    let mut sigA: uint_fast64_t = 0;
    let mut signB: bool = false;
    let mut expB: int_fast16_t = 0;
    let mut sigB: uint_fast64_t = 0;
    let mut signC: bool = false;
    let mut expC: int_fast16_t = 0;
    let mut sigC: uint_fast64_t = 0;
    let mut signZ: bool = false;
    let mut magBits: uint_fast64_t = 0;
    let mut uiZ: uint_fast64_t = 0;
    let mut normExpSig: exp16_sig64 = exp16_sig64 { exp: 0, sig: 0 };
    let mut expZ: int_fast16_t = 0;
    let mut sig128Z: uint128 = uint128 { v0: 0, v64: 0 };
    let mut sigZ: uint_fast64_t = 0;
    let mut expDiff: int_fast16_t = 0;
    let mut sig128C: uint128 = uint128 { v0: 0, v64: 0 };
    let mut shiftDist: int_fast8_t = 0;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    signA = uiA >> 63 as i32 != 0;
    expA = (uiA >> 52 as i32) as int_fast16_t
        & 0x7ff as i32 as i64;
    sigA = uiA & 0xfffffffffffff as u64;
    signB = uiB >> 63 as i32 != 0;
    expB = (uiB >> 52 as i32) as int_fast16_t
        & 0x7ff as i32 as i64;
    sigB = uiB & 0xfffffffffffff as u64;
    signC = (uiC >> 63 as i32 != 0) as i32
        ^ (op as i32 == softfloat_mulAdd_subC as i32) as i32
        != 0;
    expC = (uiC >> 52 as i32) as int_fast16_t
        & 0x7ff as i32 as i64;
    sigC = uiC & 0xfffffffffffff as u64;
    signZ = signA as i32 ^ signB as i32
        ^ (op as i32 == softfloat_mulAdd_subProd as i32) as i32
        != 0;
    if expA == 0x7ff as i32 as i64 {
        if sigA != 0 || expB == 0x7ff as i32 as i64 && sigB != 0 {
            current_block = 1591315760915442468;
        } else {
            magBits = expB as u64 | sigB;
            current_block = 15987375900167682042;
        }
    } else if expB == 0x7ff as i32 as i64 {
        if sigB != 0 {
            current_block = 1591315760915442468;
        } else {
            magBits = expA as u64 | sigA;
            current_block = 15987375900167682042;
        }
    } else if expC == 0x7ff as i32 as i64 {
        if sigC != 0 {
            uiZ = 0 as i32 as uint_fast64_t;
            current_block = 6537911109183367910;
        } else {
            uiZ = uiC;
            current_block = 16819912267750420713;
        }
    } else {
        if expA == 0 {
            if sigA == 0 {
                current_block = 1319834421151647412;
            } else {
                normExpSig = softfloat_normSubnormalF64Sig(sigA);
                expA = normExpSig.exp;
                sigA = normExpSig.sig;
                current_block = 15925075030174552612;
            }
        } else {
            current_block = 15925075030174552612;
        }
        match current_block {
            15925075030174552612 => {
                if expB == 0 {
                    if sigB == 0 {
                        current_block = 1319834421151647412;
                    } else {
                        normExpSig = softfloat_normSubnormalF64Sig(sigB);
                        expB = normExpSig.exp;
                        sigB = normExpSig.sig;
                        current_block = 5494826135382683477;
                    }
                } else {
                    current_block = 5494826135382683477;
                }
                match current_block {
                    1319834421151647412 => {}
                    _ => {
                        expZ = expA + expB - 0x3fe as i32 as i64;
                        sigA = (sigA | 0x10000000000000 as u64)
                            << 10 as i32;
                        sigB = (sigB | 0x10000000000000 as u64)
                            << 10 as i32;
                        sig128Z = softfloat_mul64To128(sigA, sigB);
                        if sig128Z.v64 < 0x2000000000000000 as u64 {
                            expZ -= 1;
                            sig128Z = softfloat_add128(
                                sig128Z.v64,
                                sig128Z.v0,
                                sig128Z.v64,
                                sig128Z.v0,
                            );
                        }
                        if expC == 0 {
                            if sigC == 0 {
                                expZ -= 1;
                                sigZ = sig128Z.v64 << 1 as i32
                                    | (sig128Z.v0 != 0 as i32 as u64)
                                        as i32 as u64;
                                current_block = 4044624156395465809;
                            } else {
                                normExpSig = softfloat_normSubnormalF64Sig(sigC);
                                expC = normExpSig.exp;
                                sigC = normExpSig.sig;
                                current_block = 2480299350034459858;
                            }
                        } else {
                            current_block = 2480299350034459858;
                        }
                        match current_block {
                            2480299350034459858 => {
                                sigC = (sigC | 0x10000000000000 as u64)
                                    << 9 as i32;
                                expDiff = expZ - expC;
                                if expDiff < 0 as i32 as i64 {
                                    expZ = expC;
                                    if signZ as i32 == signC as i32
                                        || expDiff < -(1 as i32) as i64
                                    {
                                        sig128Z
                                            .v64 = softfloat_shiftRightJam64(
                                            sig128Z.v64,
                                            -expDiff as uint_fast32_t,
                                        );
                                    } else {
                                        sig128Z = softfloat_shortShiftRightJam128(
                                            sig128Z.v64,
                                            sig128Z.v0,
                                            1 as i32 as uint_fast8_t,
                                        );
                                    }
                                } else if expDiff != 0 {
                                    sig128C = softfloat_shiftRightJam128(
                                        sigC,
                                        0 as i32 as uint64_t,
                                        expDiff as uint_fast32_t,
                                    );
                                }
                                if signZ as i32 == signC as i32 {
                                    if expDiff <= 0 as i32 as i64 {
                                        sigZ = sigC.wrapping_add(sig128Z.v64)
                                            | (sig128Z.v0 != 0 as i32 as u64)
                                                as i32 as u64;
                                    } else {
                                        sig128Z = softfloat_add128(
                                            sig128Z.v64,
                                            sig128Z.v0,
                                            sig128C.v64,
                                            sig128C.v0,
                                        );
                                        sigZ = sig128Z.v64
                                            | (sig128Z.v0 != 0 as i32 as u64)
                                                as i32 as u64;
                                    }
                                    if sigZ < 0x4000000000000000 as u64 {
                                        expZ -= 1;
                                        sigZ <<= 1 as i32;
                                    }
                                    current_block = 4044624156395465809;
                                } else {
                                    if expDiff < 0 as i32 as i64 {
                                        signZ = signC;
                                        sig128Z = softfloat_sub128(
                                            sigC,
                                            0 as i32 as uint64_t,
                                            sig128Z.v64,
                                            sig128Z.v0,
                                        );
                                        current_block = 7158658067966855297;
                                    } else if expDiff == 0 {
                                        sig128Z.v64 = (sig128Z.v64).wrapping_sub(sigC);
                                        if sig128Z.v64 | sig128Z.v0 == 0 {
                                            current_block = 6443314554198398570;
                                        } else {
                                            if sig128Z.v64 & 0x8000000000000000 as u64 != 0 {
                                                signZ = !signZ;
                                                sig128Z = softfloat_sub128(
                                                    0 as i32 as uint64_t,
                                                    0 as i32 as uint64_t,
                                                    sig128Z.v64,
                                                    sig128Z.v0,
                                                );
                                            }
                                            current_block = 7158658067966855297;
                                        }
                                    } else {
                                        sig128Z = softfloat_sub128(
                                            sig128Z.v64,
                                            sig128Z.v0,
                                            sig128C.v64,
                                            sig128C.v0,
                                        );
                                        current_block = 7158658067966855297;
                                    }
                                    match current_block {
                                        6443314554198398570 => {}
                                        _ => {
                                            if sig128Z.v64 == 0 {
                                                expZ -= 64 as i32 as i64;
                                                sig128Z.v64 = sig128Z.v0;
                                                sig128Z.v0 = 0 as i32 as uint64_t;
                                            }
                                            shiftDist = (softfloat_countLeadingZeros64(sig128Z.v64)
                                                as i32 - 1 as i32) as int_fast8_t;
                                            expZ -= shiftDist as i64;
                                            if (shiftDist as i32) < 0 as i32 {
                                                sigZ = softfloat_shortShiftRightJam64(
                                                    sig128Z.v64,
                                                    -(shiftDist as i32) as uint_fast8_t,
                                                );
                                            } else {
                                                sig128Z = softfloat_shortShiftLeft128(
                                                    sig128Z.v64,
                                                    sig128Z.v0,
                                                    shiftDist as uint_fast8_t,
                                                );
                                                sigZ = sig128Z.v64;
                                            }
                                            sigZ
                                                |= (sig128Z.v0 != 0 as i32 as u64)
                                                    as i32 as u64;
                                            current_block = 4044624156395465809;
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            6443314554198398570 => {}
                            _ => return softfloat_roundPackToF64(signZ, expZ, sigZ),
                        }
                    }
                }
            }
            _ => {}
        }
        match current_block {
            1319834421151647412 => {
                uiZ = uiC;
                if expC as u64 | sigC == 0
                    && signZ as i32 != signC as i32
                {
                    current_block = 6443314554198398570;
                } else {
                    current_block = 16819912267750420713;
                }
            }
            _ => {}
        }
        match current_block {
            16819912267750420713 => {}
            _ => {
                uiZ = (((softfloat_roundingMode as i32
                    == softfloat_round_min as i32) as i32
                    as uint_fast64_t) << 63 as i32)
                    .wrapping_add(
                        (0 as i32 as uint_fast64_t) << 52 as i32,
                    )
                    .wrapping_add(0 as i32 as u64);
                current_block = 16819912267750420713;
            }
        }
    }
    match current_block {
        15987375900167682042 => {
            if magBits != 0 {
                uiZ = ((signZ as uint_fast64_t) << 63 as i32)
                    .wrapping_add(
                        (0x7ff as i32 as uint_fast64_t) << 52 as i32,
                    )
                    .wrapping_add(0 as i32 as u64);
                if expC != 0x7ff as i32 as i64 {
                    current_block = 16819912267750420713;
                } else if sigC != 0 {
                    current_block = 6537911109183367910;
                } else if signZ as i32 == signC as i32 {
                    current_block = 16819912267750420713;
                } else {
                    current_block = 10887629115603254199;
                }
            } else {
                current_block = 10887629115603254199;
            }
            match current_block {
                16819912267750420713 => {}
                6537911109183367910 => {}
                _ => {
                    softfloat_raiseFlags(
                        softfloat_flag_invalid as i32 as uint_fast8_t,
                    );
                    uiZ = 0xfff8000000000000 as u64;
                    current_block = 6537911109183367910;
                }
            }
        }
        1591315760915442468 => {
            uiZ = softfloat_propagateNaNF64UI(uiA, uiB);
            current_block = 6537911109183367910;
        }
        _ => {}
    }
    match current_block {
        6537911109183367910 => {
            uiZ = softfloat_propagateNaNF64UI(uiZ, uiC);
        }
        _ => {}
    }
    uZ.ui = uiZ;
    return uZ.f;
}
