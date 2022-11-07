use crate::*;

pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type int_fast32_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ui: u128,
    pub s: uint128,
}

pub type extFloat80_t = extFloat80M;

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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub s: extFloat80M,
    pub f: extFloat80_t,
}
#[inline]
unsafe fn softfloat_mul64ByShifted32To128(
    mut a: uint64_t,
    mut b: uint32_t,
) -> uint128 {
    let mut uZ: C2RustUnnamed = C2RustUnnamed { ui: 0 };
    uZ
        .ui = (a as u128)
        .wrapping_mul(((b as uint_fast64_t) << 32 as i32) as u128);
    return uZ.s;
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
pub unsafe fn extF80_rem(
    mut a: extFloat80_t,
    mut b: extFloat80_t,
) -> extFloat80_t {
    let mut current_block: u64;
    let mut uA: C2RustUnnamed_3 = C2RustUnnamed_3 {
        s: extFloat80_t {
            signif: 0,
            signExp: 0,
        },
    };
    let mut uiA64: uint_fast16_t = 0;
    let mut uiA0: uint_fast64_t = 0;
    let mut signA: bool = false;
    let mut expA: int_fast32_t = 0;
    let mut sigA: uint_fast64_t = 0;
    let mut uB: C2RustUnnamed_2 = C2RustUnnamed_2 {
        s: extFloat80_t {
            signif: 0,
            signExp: 0,
        },
    };
    let mut uiB64: uint_fast16_t = 0;
    let mut uiB0: uint_fast64_t = 0;
    let mut expB: int_fast32_t = 0;
    let mut sigB: uint_fast64_t = 0;
    let mut normExpSig: exp32_sig64 = exp32_sig64 { exp: 0, sig: 0 };
    let mut expDiff: int_fast32_t = 0;
    let mut rem: uint128 = uint128 { v0: 0, v64: 0 };
    let mut shiftedSigB: uint128 = uint128 { v0: 0, v64: 0 };
    let mut q: uint_fast32_t = 0;
    let mut recip32: uint_fast32_t = 0;
    let mut q64: uint_fast64_t = 0;
    let mut term: uint128 = uint128 { v0: 0, v64: 0 };
    let mut altRem: uint128 = uint128 { v0: 0, v64: 0 };
    let mut meanRem: uint128 = uint128 { v0: 0, v64: 0 };
    let mut signRem: bool = false;
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uiZ64: uint_fast16_t = 0;
    let mut uiZ0: uint_fast64_t = 0;
    let mut uZ: C2RustUnnamed_1 = C2RustUnnamed_1 {
        s: extFloat80_t {
            signif: 0,
            signExp: 0,
        },
    };
    uA.f = a;
    uiA64 = uA.s.signExp as uint_fast16_t;
    uiA0 = uA.s.signif;
    signA = uiA64 as uint16_t as i32 >> 15 as i32 != 0;
    expA = (uiA64 & 0x7fff as i32 as u64) as int_fast32_t;
    sigA = uiA0;
    uB.f = b;
    uiB64 = uB.s.signExp as uint_fast16_t;
    uiB0 = uB.s.signif;
    expB = (uiB64 & 0x7fff as i32 as u64) as int_fast32_t;
    sigB = uiB0;
    if expA == 0x7fff as i32 as i64 {
        if sigA & 0x7fffffffffffffff as u64 != 0
            || expB == 0x7fff as i32 as i64
                && sigB & 0x7fffffffffffffff as u64 != 0
        {
            current_block = 18049726823399313809;
        } else {
            current_block = 14055877428889622250;
        }
    } else {
        if expB == 0x7fff as i32 as i64 {
            if sigB & 0x7fffffffffffffff as u64 != 0 {
                current_block = 18049726823399313809;
            } else {
                expB += expB;
                current_block = 224731115979188411;
            }
        } else {
            current_block = 224731115979188411;
        }
        match current_block {
            18049726823399313809 => {}
            _ => {
                if expB == 0 {
                    expB = 1 as i32 as int_fast32_t;
                }
                if sigB & 0x8000000000000000 as u64 == 0 {
                    if sigB == 0 {
                        current_block = 14055877428889622250;
                    } else {
                        normExpSig = softfloat_normSubnormalExtF80Sig(sigB);
                        expB += normExpSig.exp;
                        sigB = normExpSig.sig;
                        current_block = 11057878835866523405;
                    }
                } else {
                    current_block = 11057878835866523405;
                }
                match current_block {
                    14055877428889622250 => {}
                    _ => {
                        if expA == 0 {
                            expA = 1 as i32 as int_fast32_t;
                        }
                        if sigA & 0x8000000000000000 as u64 == 0 {
                            if sigA == 0 {
                                expA = 0 as i32 as int_fast32_t;
                                current_block = 11646430818263159649;
                            } else {
                                normExpSig = softfloat_normSubnormalExtF80Sig(sigA);
                                expA += normExpSig.exp;
                                sigA = normExpSig.sig;
                                current_block = 14136749492126903395;
                            }
                        } else {
                            current_block = 14136749492126903395;
                        }
                        match current_block {
                            14136749492126903395 => {
                                expDiff = expA - expB;
                                if !(expDiff < -(1 as i32) as i64) {
                                    rem = softfloat_shortShiftLeft128(
                                        0 as i32 as uint64_t,
                                        sigA,
                                        32 as i32 as uint_fast8_t,
                                    );
                                    shiftedSigB = softfloat_shortShiftLeft128(
                                        0 as i32 as uint64_t,
                                        sigB,
                                        32 as i32 as uint_fast8_t,
                                    );
                                    if expDiff < 1 as i32 as i64 {
                                        if expDiff != 0 {
                                            expB -= 1;
                                            shiftedSigB = softfloat_shortShiftLeft128(
                                                0 as i32 as uint64_t,
                                                sigB,
                                                33 as i32 as uint_fast8_t,
                                            );
                                            q = 0 as i32 as uint_fast32_t;
                                        } else {
                                            q = (sigB <= sigA) as i32 as uint_fast32_t;
                                            if q != 0 {
                                                rem = softfloat_sub128(
                                                    rem.v64,
                                                    rem.v0,
                                                    shiftedSigB.v64,
                                                    shiftedSigB.v0,
                                                );
                                            }
                                        }
                                        current_block = 3546145585875536353;
                                    } else {
                                        recip32 = (0x7fffffffffffffff as u64)
                                            .wrapping_div(
                                                (sigB >> 32 as i32) as uint32_t as u64,
                                            ) as uint32_t as uint_fast32_t;
                                        expDiff -= 30 as i32 as i64;
                                        loop {
                                            q64 = ((rem.v64 >> 2 as i32) as uint32_t
                                                as uint_fast64_t)
                                                .wrapping_mul(recip32);
                                            if expDiff < 0 as i32 as i64 {
                                                break;
                                            }
                                            q = q64
                                                .wrapping_add(0x80000000 as u32 as u64)
                                                >> 32 as i32;
                                            rem = softfloat_shortShiftLeft128(
                                                rem.v64,
                                                rem.v0,
                                                29 as i32 as uint_fast8_t,
                                            );
                                            term = softfloat_mul64ByShifted32To128(sigB, q as uint32_t);
                                            rem = softfloat_sub128(rem.v64, rem.v0, term.v64, term.v0);
                                            if rem.v64 & 0x8000000000000000 as u64 != 0 {
                                                rem = softfloat_add128(
                                                    rem.v64,
                                                    rem.v0,
                                                    shiftedSigB.v64,
                                                    shiftedSigB.v0,
                                                );
                                            }
                                            expDiff -= 29 as i32 as i64;
                                        }
                                        q = ((q64 >> 32 as i32) as uint32_t
                                            >> (!expDiff & 31 as i32 as i64))
                                            as uint_fast32_t;
                                        rem = softfloat_shortShiftLeft128(
                                            rem.v64,
                                            rem.v0,
                                            (expDiff + 30 as i32 as i64)
                                                as uint_fast8_t,
                                        );
                                        term = softfloat_mul64ByShifted32To128(sigB, q as uint32_t);
                                        rem = softfloat_sub128(rem.v64, rem.v0, term.v64, term.v0);
                                        if rem.v64 & 0x8000000000000000 as u64 != 0 {
                                            altRem = softfloat_add128(
                                                rem.v64,
                                                rem.v0,
                                                shiftedSigB.v64,
                                                shiftedSigB.v0,
                                            );
                                            current_block = 9411811017107857188;
                                        } else {
                                            current_block = 3546145585875536353;
                                        }
                                    }
                                    match current_block {
                                        3546145585875536353 => {
                                            loop {
                                                altRem = rem;
                                                q = q.wrapping_add(1);
                                                rem = softfloat_sub128(
                                                    rem.v64,
                                                    rem.v0,
                                                    shiftedSigB.v64,
                                                    shiftedSigB.v0,
                                                );
                                                if !(rem.v64 & 0x8000000000000000 as u64 == 0) {
                                                    break;
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                    meanRem = softfloat_add128(
                                        rem.v64,
                                        rem.v0,
                                        altRem.v64,
                                        altRem.v0,
                                    );
                                    if meanRem.v64 & 0x8000000000000000 as u64 != 0
                                        || meanRem.v64 | meanRem.v0 == 0
                                            && q & 1 as i32 as u64 != 0
                                    {
                                        rem = altRem;
                                    }
                                    signRem = signA;
                                    if rem.v64 & 0x8000000000000000 as u64 != 0 {
                                        signRem = !signRem;
                                        rem = softfloat_sub128(
                                            0 as i32 as uint64_t,
                                            0 as i32 as uint64_t,
                                            rem.v64,
                                            rem.v0,
                                        );
                                    }
                                    return softfloat_normRoundPackToExtF80(
                                        signRem,
                                        if rem.v64 | rem.v0 != 0 {
                                            expB + 32 as i32 as i64
                                        } else {
                                            0 as i32 as i64
                                        },
                                        rem.v64,
                                        rem.v0,
                                        80 as i32 as uint_fast8_t,
                                    );
                                }
                            }
                            _ => {}
                        }
                        if expA < 1 as i32 as i64 {
                            sigA >>= 1 as i32 as i64 - expA;
                            expA = 0 as i32 as int_fast32_t;
                        }
                        uiZ64 = (signA as uint_fast16_t) << 15 as i32
                            | expA as u64;
                        uiZ0 = sigA;
                        current_block = 7875628066601213285;
                    }
                }
            }
        }
    }
    match current_block {
        18049726823399313809 => {
            uiZ = softfloat_propagateNaNExtF80UI(uiA64, uiA0, uiB64, uiB0);
            uiZ64 = uiZ.v64;
            uiZ0 = uiZ.v0;
        }
        14055877428889622250 => {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
            uiZ64 = 0xffff as i32 as uint_fast16_t;
            uiZ0 = 0xc000000000000000 as u64;
        }
        _ => {}
    }
    uZ.s.signExp = uiZ64 as uint16_t;
    uZ.s.signif = uiZ0;
    return uZ.f;
}
