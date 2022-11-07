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
unsafe fn softfloat_countLeadingZeros32(mut a: uint32_t) -> uint_fast8_t {
    return (if a != 0 { a.leading_zeros() as i32 } else { 32 as i32 })
        as uint_fast8_t;
}
#[inline]
unsafe fn softfloat_shiftRightJam32(
    mut a: uint32_t,
    mut dist: uint_fast16_t,
) -> uint32_t {
    return if dist < 31 as i32 as u64 {
        a >> dist
            | (a << (dist.wrapping_neg() & 31 as i32 as u64)
                != 0 as i32 as u32) as i32 as u32
    } else {
        (a != 0 as i32 as u32) as i32 as u32
    };
}
pub unsafe fn softfloat_mulAddF16(
    mut uiA: uint_fast16_t,
    mut uiB: uint_fast16_t,
    mut uiC: uint_fast16_t,
    mut op: uint_fast8_t,
) -> float16_t {
    let mut current_block: u64;
    let mut signA: bool = false;
    let mut expA: int_fast8_t = 0;
    let mut sigA: uint_fast16_t = 0;
    let mut signB: bool = false;
    let mut expB: int_fast8_t = 0;
    let mut sigB: uint_fast16_t = 0;
    let mut signC: bool = false;
    let mut expC: int_fast8_t = 0;
    let mut sigC: uint_fast16_t = 0;
    let mut signProd: bool = false;
    let mut magBits: uint_fast16_t = 0;
    let mut uiZ: uint_fast16_t = 0;
    let mut normExpSig: exp8_sig16 = exp8_sig16 { exp: 0, sig: 0 };
    let mut expProd: int_fast8_t = 0;
    let mut sigProd: uint_fast32_t = 0;
    let mut signZ: bool = false;
    let mut expZ: int_fast8_t = 0;
    let mut sigZ: uint_fast16_t = 0;
    let mut expDiff: int_fast8_t = 0;
    let mut sig32Z: uint_fast32_t = 0;
    let mut sig32C: uint_fast32_t = 0;
    let mut shiftDist: int_fast8_t = 0;
    let mut uZ: ui16_f16 = ui16_f16 { ui: 0 };
    signA = uiA as uint16_t as i32 >> 15 as i32 != 0;
    expA = ((uiA >> 10 as i32) as int_fast8_t as i32
        & 0x1f as i32) as int_fast8_t;
    sigA = uiA & 0x3ff as i32 as u64;
    signB = uiB as uint16_t as i32 >> 15 as i32 != 0;
    expB = ((uiB >> 10 as i32) as int_fast8_t as i32
        & 0x1f as i32) as int_fast8_t;
    sigB = uiB & 0x3ff as i32 as u64;
    signC = (uiC as uint16_t as i32 >> 15 as i32 != 0) as i32
        ^ (op as i32 == softfloat_mulAdd_subC as i32) as i32
        != 0;
    expC = ((uiC >> 10 as i32) as int_fast8_t as i32
        & 0x1f as i32) as int_fast8_t;
    sigC = uiC & 0x3ff as i32 as u64;
    signProd = signA as i32 ^ signB as i32
        ^ (op as i32 == softfloat_mulAdd_subProd as i32) as i32
        != 0;
    if expA as i32 == 0x1f as i32 {
        if sigA != 0 || expB as i32 == 0x1f as i32 && sigB != 0 {
            current_block = 5673178990457747448;
        } else {
            magBits = expB as u64 | sigB;
            current_block = 16765921993821062129;
        }
    } else if expB as i32 == 0x1f as i32 {
        if sigB != 0 {
            current_block = 5673178990457747448;
        } else {
            magBits = expA as u64 | sigA;
            current_block = 16765921993821062129;
        }
    } else if expC as i32 == 0x1f as i32 {
        if sigC != 0 {
            uiZ = 0 as i32 as uint_fast16_t;
            current_block = 9753847900409538;
        } else {
            uiZ = uiC;
            current_block = 2372134190545492357;
        }
    } else {
        if expA == 0 {
            if sigA == 0 {
                current_block = 13307676748897018853;
            } else {
                normExpSig = softfloat_normSubnormalF16Sig(sigA);
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
                        current_block = 13307676748897018853;
                    } else {
                        normExpSig = softfloat_normSubnormalF16Sig(sigB);
                        expB = normExpSig.exp;
                        sigB = normExpSig.sig;
                        current_block = 18377268871191777778;
                    }
                } else {
                    current_block = 18377268871191777778;
                }
                match current_block {
                    13307676748897018853 => {}
                    _ => {
                        expProd = (expA as i32 + expB as i32
                            - 0xe as i32) as int_fast8_t;
                        sigA = (sigA | 0x400 as i32 as u64)
                            << 4 as i32;
                        sigB = (sigB | 0x400 as i32 as u64)
                            << 4 as i32;
                        sigProd = sigA.wrapping_mul(sigB);
                        if sigProd < 0x20000000 as i32 as u64 {
                            expProd -= 1;
                            sigProd <<= 1 as i32;
                        }
                        signZ = signProd;
                        if expC == 0 {
                            if sigC == 0 {
                                expZ = (expProd as i32 - 1 as i32)
                                    as int_fast8_t;
                                sigZ = sigProd >> 15 as i32
                                    | (sigProd & 0x7fff as i32 as u64
                                        != 0 as i32 as u64) as i32
                                        as u64;
                                current_block = 17456456976253534065;
                            } else {
                                normExpSig = softfloat_normSubnormalF16Sig(sigC);
                                expC = normExpSig.exp;
                                sigC = normExpSig.sig;
                                current_block = 1356832168064818221;
                            }
                        } else {
                            current_block = 1356832168064818221;
                        }
                        match current_block {
                            1356832168064818221 => {
                                sigC = (sigC | 0x400 as i32 as u64)
                                    << 3 as i32;
                                expDiff = (expProd as i32 - expC as i32)
                                    as int_fast8_t;
                                if signProd as i32 == signC as i32 {
                                    if expDiff as i32 <= 0 as i32 {
                                        expZ = expC;
                                        sigZ = sigC
                                            .wrapping_add(
                                                softfloat_shiftRightJam32(
                                                    sigProd as uint32_t,
                                                    (16 as i32 - expDiff as i32)
                                                        as uint_fast16_t,
                                                ) as u64,
                                            );
                                    } else {
                                        expZ = expProd;
                                        sig32Z = sigProd
                                            .wrapping_add(
                                                softfloat_shiftRightJam32(
                                                    (sigC << 16 as i32) as uint32_t,
                                                    expDiff as uint_fast16_t,
                                                ) as u64,
                                            );
                                        sigZ = sig32Z >> 16 as i32
                                            | (sig32Z & 0xffff as i32 as u64
                                                != 0 as i32 as u64) as i32
                                                as u64;
                                    }
                                    if sigZ < 0x4000 as i32 as u64 {
                                        expZ -= 1;
                                        sigZ <<= 1 as i32;
                                    }
                                    current_block = 17456456976253534065;
                                } else {
                                    sig32C = sigC << 16 as i32;
                                    if (expDiff as i32) < 0 as i32 {
                                        signZ = signC;
                                        expZ = expC;
                                        sig32Z = sig32C
                                            .wrapping_sub(
                                                softfloat_shiftRightJam32(
                                                    sigProd as uint32_t,
                                                    -(expDiff as i32) as uint_fast16_t,
                                                ) as u64,
                                            );
                                        current_block = 10853015579903106591;
                                    } else if expDiff == 0 {
                                        expZ = expProd;
                                        sig32Z = sigProd.wrapping_sub(sig32C);
                                        if sig32Z == 0 {
                                            current_block = 8113078540257294733;
                                        } else {
                                            if sig32Z & 0x80000000 as u32 as u64 != 0
                                            {
                                                signZ = !signZ;
                                                sig32Z = sig32Z.wrapping_neg();
                                            }
                                            current_block = 10853015579903106591;
                                        }
                                    } else {
                                        expZ = expProd;
                                        sig32Z = sigProd
                                            .wrapping_sub(
                                                softfloat_shiftRightJam32(
                                                    sig32C as uint32_t,
                                                    expDiff as uint_fast16_t,
                                                ) as u64,
                                            );
                                        current_block = 10853015579903106591;
                                    }
                                    match current_block {
                                        8113078540257294733 => {}
                                        _ => {
                                            shiftDist = (softfloat_countLeadingZeros32(
                                                sig32Z as uint32_t,
                                            ) as i32 - 1 as i32) as int_fast8_t;
                                            expZ = (expZ as i32 - shiftDist as i32)
                                                as int_fast8_t;
                                            shiftDist = (shiftDist as i32 - 16 as i32)
                                                as int_fast8_t;
                                            if (shiftDist as i32) < 0 as i32 {
                                                sigZ = sig32Z >> -(shiftDist as i32)
                                                    | ((sig32Z
                                                        << (shiftDist as i32 & 31 as i32))
                                                        as uint32_t != 0 as i32 as u32)
                                                        as i32 as u64;
                                            } else {
                                                sigZ = sig32Z << shiftDist as i32;
                                            }
                                            current_block = 17456456976253534065;
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            8113078540257294733 => {}
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
            _ => {}
        }
        match current_block {
            13307676748897018853 => {
                uiZ = uiC;
                if expC as u64 | sigC == 0
                    && signProd as i32 != signC as i32
                {
                    current_block = 8113078540257294733;
                } else {
                    current_block = 2372134190545492357;
                }
            }
            _ => {}
        }
        match current_block {
            2372134190545492357 => {}
            _ => {
                uiZ = ((((softfloat_roundingMode as i32
                    == softfloat_round_min as i32) as i32 as uint16_t
                    as i32) << 15 as i32)
                    + ((0 as i32 as uint16_t as i32)
                        << 10 as i32) + 0 as i32) as uint_fast16_t;
                current_block = 2372134190545492357;
            }
        }
    }
    match current_block {
        16765921993821062129 => {
            if magBits != 0 {
                uiZ = (((signProd as uint16_t as i32) << 15 as i32)
                    + ((0x1f as i32 as uint16_t as i32)
                        << 10 as i32) + 0 as i32) as uint_fast16_t;
                if expC as i32 != 0x1f as i32 {
                    current_block = 2372134190545492357;
                } else if sigC != 0 {
                    current_block = 9753847900409538;
                } else if signProd as i32 == signC as i32 {
                    current_block = 2372134190545492357;
                } else {
                    current_block = 14648606000749551097;
                }
            } else {
                current_block = 14648606000749551097;
            }
            match current_block {
                2372134190545492357 => {}
                9753847900409538 => {}
                _ => {
                    softfloat_raiseFlags(
                        softfloat_flag_invalid as i32 as uint_fast8_t,
                    );
                    uiZ = 0xfe00 as i32 as uint_fast16_t;
                    current_block = 9753847900409538;
                }
            }
        }
        5673178990457747448 => {
            uiZ = softfloat_propagateNaNF16UI(uiA, uiB);
            current_block = 9753847900409538;
        }
        _ => {}
    }
    match current_block {
        9753847900409538 => {
            uiZ = softfloat_propagateNaNF16UI(uiZ, uiC);
        }
        _ => {}
    }
    uZ.ui = uiZ as uint16_t;
    return uZ.f;
}
