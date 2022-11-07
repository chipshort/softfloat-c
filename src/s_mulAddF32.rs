use crate::*;

pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type int_fast8_t = i8;
pub type int_fast16_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui32_f32 {
    pub ui: uint32_t,
    pub f: float32_t,
}
pub type C2RustUnnamed = u32;
pub const softfloat_mulAdd_subProd: C2RustUnnamed = 2;
pub const softfloat_mulAdd_subC: C2RustUnnamed = 1;

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
unsafe fn softfloat_countLeadingZeros64(mut a: uint64_t) -> uint_fast8_t {
    return (if a != 0 {
        (a as u64).leading_zeros() as i32
    } else {
        64 as i32
    }) as uint_fast8_t;
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
pub unsafe fn softfloat_mulAddF32(
    mut uiA: uint_fast32_t,
    mut uiB: uint_fast32_t,
    mut uiC: uint_fast32_t,
    mut op: uint_fast8_t,
) -> float32_t {
    let mut current_block: u64;
    let mut signA: bool = false;
    let mut expA: int_fast16_t = 0;
    let mut sigA: uint_fast32_t = 0;
    let mut signB: bool = false;
    let mut expB: int_fast16_t = 0;
    let mut sigB: uint_fast32_t = 0;
    let mut signC: bool = false;
    let mut expC: int_fast16_t = 0;
    let mut sigC: uint_fast32_t = 0;
    let mut signProd: bool = false;
    let mut magBits: uint_fast32_t = 0;
    let mut uiZ: uint_fast32_t = 0;
    let mut normExpSig: exp16_sig32 = exp16_sig32 { exp: 0, sig: 0 };
    let mut expProd: int_fast16_t = 0;
    let mut sigProd: uint_fast64_t = 0;
    let mut signZ: bool = false;
    let mut expZ: int_fast16_t = 0;
    let mut sigZ: uint_fast32_t = 0;
    let mut expDiff: int_fast16_t = 0;
    let mut sig64Z: uint_fast64_t = 0;
    let mut sig64C: uint_fast64_t = 0;
    let mut shiftDist: int_fast8_t = 0;
    let mut uZ: ui32_f32 = ui32_f32 { ui: 0 };
    signA = uiA as uint32_t >> 31 as i32 != 0;
    expA = (uiA >> 23 as i32) as int_fast16_t
        & 0xff as i32 as i64;
    sigA = uiA & 0x7fffff as i32 as u64;
    signB = uiB as uint32_t >> 31 as i32 != 0;
    expB = (uiB >> 23 as i32) as int_fast16_t
        & 0xff as i32 as i64;
    sigB = uiB & 0x7fffff as i32 as u64;
    signC = (uiC as uint32_t >> 31 as i32 != 0) as i32
        ^ (op as i32 == softfloat_mulAdd_subC as i32) as i32
        != 0;
    expC = (uiC >> 23 as i32) as int_fast16_t
        & 0xff as i32 as i64;
    sigC = uiC & 0x7fffff as i32 as u64;
    signProd = signA as i32 ^ signB as i32
        ^ (op as i32 == softfloat_mulAdd_subProd as i32) as i32
        != 0;
    if expA == 0xff as i32 as i64 {
        if sigA != 0 || expB == 0xff as i32 as i64 && sigB != 0 {
            current_block = 9625126720518017888;
        } else {
            magBits = expB as u64 | sigB;
            current_block = 16650704885333802742;
        }
    } else if expB == 0xff as i32 as i64 {
        if sigB != 0 {
            current_block = 9625126720518017888;
        } else {
            magBits = expA as u64 | sigA;
            current_block = 16650704885333802742;
        }
    } else if expC == 0xff as i32 as i64 {
        if sigC != 0 {
            uiZ = 0 as i32 as uint_fast32_t;
            current_block = 4728923198747788409;
        } else {
            uiZ = uiC;
            current_block = 13041432063757805130;
        }
    } else {
        if expA == 0 {
            if sigA == 0 {
                current_block = 2999579709949419792;
            } else {
                normExpSig = softfloat_normSubnormalF32Sig(sigA);
                expA = normExpSig.exp;
                sigA = normExpSig.sig;
                current_block = 3437258052017859086;
            }
        } else {
            current_block = 3437258052017859086;
        }
        match current_block {
            3437258052017859086 => {
                if expB == 0 {
                    if sigB == 0 {
                        current_block = 2999579709949419792;
                    } else {
                        normExpSig = softfloat_normSubnormalF32Sig(sigB);
                        expB = normExpSig.exp;
                        sigB = normExpSig.sig;
                        current_block = 18377268871191777778;
                    }
                } else {
                    current_block = 18377268871191777778;
                }
                match current_block {
                    2999579709949419792 => {}
                    _ => {
                        expProd = expA + expB - 0x7e as i32 as i64;
                        sigA = (sigA | 0x800000 as i32 as u64)
                            << 7 as i32;
                        sigB = (sigB | 0x800000 as i32 as u64)
                            << 7 as i32;
                        sigProd = sigA.wrapping_mul(sigB);
                        if sigProd < 0x2000000000000000 as u64 {
                            expProd -= 1;
                            sigProd <<= 1 as i32;
                        }
                        signZ = signProd;
                        if expC == 0 {
                            if sigC == 0 {
                                expZ = expProd - 1 as i32 as i64;
                                sigZ = softfloat_shortShiftRightJam64(
                                    sigProd,
                                    31 as i32 as uint_fast8_t,
                                );
                                current_block = 9399272276261068376;
                            } else {
                                normExpSig = softfloat_normSubnormalF32Sig(sigC);
                                expC = normExpSig.exp;
                                sigC = normExpSig.sig;
                                current_block = 1356832168064818221;
                            }
                        } else {
                            current_block = 1356832168064818221;
                        }
                        match current_block {
                            1356832168064818221 => {
                                sigC = (sigC | 0x800000 as i32 as u64)
                                    << 6 as i32;
                                expDiff = expProd - expC;
                                if signProd as i32 == signC as i32 {
                                    if expDiff <= 0 as i32 as i64 {
                                        expZ = expC;
                                        sigZ = sigC
                                            .wrapping_add(
                                                softfloat_shiftRightJam64(
                                                    sigProd,
                                                    (32 as i32 as i64 - expDiff)
                                                        as uint_fast32_t,
                                                ),
                                            );
                                    } else {
                                        expZ = expProd;
                                        sig64Z = sigProd
                                            .wrapping_add(
                                                softfloat_shiftRightJam64(
                                                    sigC << 32 as i32,
                                                    expDiff as uint_fast32_t,
                                                ),
                                            );
                                        sigZ = softfloat_shortShiftRightJam64(
                                            sig64Z,
                                            32 as i32 as uint_fast8_t,
                                        );
                                    }
                                    if sigZ < 0x40000000 as i32 as u64 {
                                        expZ -= 1;
                                        sigZ <<= 1 as i32;
                                    }
                                    current_block = 9399272276261068376;
                                } else {
                                    sig64C = sigC << 32 as i32;
                                    if expDiff < 0 as i32 as i64 {
                                        signZ = signC;
                                        expZ = expC;
                                        sig64Z = sig64C
                                            .wrapping_sub(
                                                softfloat_shiftRightJam64(
                                                    sigProd,
                                                    -expDiff as uint_fast32_t,
                                                ),
                                            );
                                        current_block = 10853015579903106591;
                                    } else if expDiff == 0 {
                                        expZ = expProd;
                                        sig64Z = sigProd.wrapping_sub(sig64C);
                                        if sig64Z == 0 {
                                            current_block = 1115355813607263255;
                                        } else {
                                            if sig64Z & 0x8000000000000000 as u64 != 0 {
                                                signZ = !signZ;
                                                sig64Z = sig64Z.wrapping_neg();
                                            }
                                            current_block = 10853015579903106591;
                                        }
                                    } else {
                                        expZ = expProd;
                                        sig64Z = sigProd
                                            .wrapping_sub(
                                                softfloat_shiftRightJam64(sig64C, expDiff as uint_fast32_t),
                                            );
                                        current_block = 10853015579903106591;
                                    }
                                    match current_block {
                                        1115355813607263255 => {}
                                        _ => {
                                            shiftDist = (softfloat_countLeadingZeros64(sig64Z)
                                                as i32 - 1 as i32) as int_fast8_t;
                                            expZ -= shiftDist as i64;
                                            shiftDist = (shiftDist as i32 - 32 as i32)
                                                as int_fast8_t;
                                            if (shiftDist as i32) < 0 as i32 {
                                                sigZ = softfloat_shortShiftRightJam64(
                                                    sig64Z,
                                                    -(shiftDist as i32) as uint_fast8_t,
                                                );
                                            } else {
                                                sigZ = sig64Z << shiftDist as i32;
                                            }
                                            current_block = 9399272276261068376;
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            1115355813607263255 => {}
                            _ => return softfloat_roundPackToF32(signZ, expZ, sigZ),
                        }
                    }
                }
            }
            _ => {}
        }
        match current_block {
            2999579709949419792 => {
                uiZ = uiC;
                if expC as u64 | sigC == 0
                    && signProd as i32 != signC as i32
                {
                    current_block = 1115355813607263255;
                } else {
                    current_block = 13041432063757805130;
                }
            }
            _ => {}
        }
        match current_block {
            13041432063757805130 => {}
            _ => {
                uiZ = (((softfloat_roundingMode as i32
                    == softfloat_round_min as i32) as i32 as uint32_t)
                    << 31 as i32)
                    .wrapping_add((0 as i32 as uint32_t) << 23 as i32)
                    .wrapping_add(0 as i32 as u32) as uint_fast32_t;
                current_block = 13041432063757805130;
            }
        }
    }
    match current_block {
        16650704885333802742 => {
            if magBits != 0 {
                uiZ = ((signProd as uint32_t) << 31 as i32)
                    .wrapping_add((0xff as i32 as uint32_t) << 23 as i32)
                    .wrapping_add(0 as i32 as u32) as uint_fast32_t;
                if expC != 0xff as i32 as i64 {
                    current_block = 13041432063757805130;
                } else if sigC != 0 {
                    current_block = 4728923198747788409;
                } else if signProd as i32 == signC as i32 {
                    current_block = 13041432063757805130;
                } else {
                    current_block = 14648606000749551097;
                }
            } else {
                current_block = 14648606000749551097;
            }
            match current_block {
                13041432063757805130 => {}
                4728923198747788409 => {}
                _ => {
                    softfloat_raiseFlags(
                        softfloat_flag_invalid as i32 as uint_fast8_t,
                    );
                    uiZ = 0xffc00000 as u32 as uint_fast32_t;
                    current_block = 4728923198747788409;
                }
            }
        }
        9625126720518017888 => {
            uiZ = softfloat_propagateNaNF32UI(uiA, uiB);
            current_block = 4728923198747788409;
        }
        _ => {}
    }
    match current_block {
        4728923198747788409 => {
            uiZ = softfloat_propagateNaNF32UI(uiZ, uiC);
        }
        _ => {}
    }
    uZ.ui = uiZ as uint32_t;
    return uZ.f;
}
