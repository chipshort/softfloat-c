use crate::*;

pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type int_fast32_t = i64;
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
pub union ui128_f128 {
    pub ui: uint128,
    pub f: float128_t,
}

pub type C2RustUnnamed_0 = u32;
pub const softfloat_flag_invalid: C2RustUnnamed_0 = 16;
pub const softfloat_flag_infinite: C2RustUnnamed_0 = 8;
pub const softfloat_flag_overflow: C2RustUnnamed_0 = 4;
pub const softfloat_flag_underflow: C2RustUnnamed_0 = 2;
pub const softfloat_flag_inexact: C2RustUnnamed_0 = 1;
#[inline]
unsafe fn softfloat_mul128By32(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut b: uint32_t,
) -> uint128 {
    let mut uZ: C2RustUnnamed = C2RustUnnamed { ui: 0 };
    uZ.ui = ((a64 as u128) << 64 as i32 | a0 as u128).wrapping_mul(b as u128);
    return uZ.s;
}
#[inline]
unsafe fn softfloat_le128(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut b64: uint64_t,
    mut b0: uint64_t,
) -> bool {
    return a64 < b64 || a64 == b64 && a0 <= b0;
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
pub unsafe fn f128_rem(mut a: float128_t, mut b: float128_t) -> float128_t {
    let mut current_block: u64;
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiA64: uint_fast64_t = 0;
    let mut uiA0: uint_fast64_t = 0;
    let mut signA: bool = false;
    let mut expA: int_fast32_t = 0;
    let mut sigA: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uB: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiB64: uint_fast64_t = 0;
    let mut uiB0: uint_fast64_t = 0;
    let mut expB: int_fast32_t = 0;
    let mut sigB: uint128 = uint128 { v0: 0, v64: 0 };
    let mut normExpSig: exp32_sig128 = exp32_sig128 {
        exp: 0,
        sig: uint128 { v0: 0, v64: 0 },
    };
    let mut rem: uint128 = uint128 { v0: 0, v64: 0 };
    let mut expDiff: int_fast32_t = 0;
    let mut q: uint_fast32_t = 0;
    let mut recip32: uint_fast32_t = 0;
    let mut q64: uint_fast64_t = 0;
    let mut term: uint128 = uint128 { v0: 0, v64: 0 };
    let mut altRem: uint128 = uint128 { v0: 0, v64: 0 };
    let mut meanRem: uint128 = uint128 { v0: 0, v64: 0 };
    let mut signRem: bool = false;
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    signA = uiA64 >> 63 as i32 != 0;
    expA = (uiA64 >> 48 as i32) as int_fast32_t
        & 0x7fff as i32 as i64;
    sigA.v64 = uiA64 & 0xffffffffffff as u64;
    sigA.v0 = uiA0;
    uB.f = b;
    uiB64 = uB.ui.v64;
    uiB0 = uB.ui.v0;
    expB = (uiB64 >> 48 as i32) as int_fast32_t
        & 0x7fff as i32 as i64;
    sigB.v64 = uiB64 & 0xffffffffffff as u64;
    sigB.v0 = uiB0;
    if expA == 0x7fff as i32 as i64 {
        if sigA.v64 | sigA.v0 != 0
            || expB == 0x7fff as i32 as i64 && sigB.v64 | sigB.v0 != 0
        {
            current_block = 14125127626113485617;
        } else {
            current_block = 14121197095020257330;
        }
    } else if expB == 0x7fff as i32 as i64 {
        if sigB.v64 | sigB.v0 != 0 {
            current_block = 14125127626113485617;
        } else {
            return a
        }
    } else {
        if expB == 0 {
            if sigB.v64 | sigB.v0 == 0 {
                current_block = 14121197095020257330;
            } else {
                normExpSig = softfloat_normSubnormalF128Sig(sigB.v64, sigB.v0);
                expB = normExpSig.exp;
                sigB = normExpSig.sig;
                current_block = 13472856163611868459;
            }
        } else {
            current_block = 13472856163611868459;
        }
        match current_block {
            14121197095020257330 => {}
            _ => {
                if expA == 0 {
                    if sigA.v64 | sigA.v0 == 0 {
                        return a;
                    }
                    normExpSig = softfloat_normSubnormalF128Sig(sigA.v64, sigA.v0);
                    expA = normExpSig.exp;
                    sigA = normExpSig.sig;
                }
                sigA.v64 |= 0x1000000000000 as u64;
                sigB.v64 |= 0x1000000000000 as u64;
                rem = sigA;
                expDiff = expA - expB;
                if expDiff < 1 as i32 as i64 {
                    if expDiff < -(1 as i32) as i64 {
                        return a;
                    }
                    if expDiff != 0 {
                        expB -= 1;
                        sigB = softfloat_add128(sigB.v64, sigB.v0, sigB.v64, sigB.v0);
                        q = 0 as i32 as uint_fast32_t;
                    } else {
                        q = softfloat_le128(sigB.v64, sigB.v0, rem.v64, rem.v0)
                            as uint_fast32_t;
                        if q != 0 {
                            rem = softfloat_sub128(rem.v64, rem.v0, sigB.v64, sigB.v0);
                        }
                    }
                    current_block = 5159818223158340697;
                } else {
                    recip32 = (0x7fffffffffffffff as u64)
                        .wrapping_div(
                            (sigB.v64 >> 17 as i32) as uint32_t as u64,
                        ) as uint32_t as uint_fast32_t;
                    expDiff -= 30 as i32 as i64;
                    loop {
                        q64 = ((rem.v64 >> 19 as i32) as uint32_t
                            as uint_fast64_t)
                            .wrapping_mul(recip32);
                        if expDiff < 0 as i32 as i64 {
                            break;
                        }
                        q = q64.wrapping_add(0x80000000 as u32 as u64)
                            >> 32 as i32;
                        rem = softfloat_shortShiftLeft128(
                            rem.v64,
                            rem.v0,
                            29 as i32 as uint_fast8_t,
                        );
                        term = softfloat_mul128By32(sigB.v64, sigB.v0, q as uint32_t);
                        rem = softfloat_sub128(rem.v64, rem.v0, term.v64, term.v0);
                        if rem.v64 & 0x8000000000000000 as u64 != 0 {
                            rem = softfloat_add128(rem.v64, rem.v0, sigB.v64, sigB.v0);
                        }
                        expDiff -= 29 as i32 as i64;
                    }
                    q = ((q64 >> 32 as i32) as uint32_t
                        >> (!expDiff & 31 as i32 as i64))
                        as uint_fast32_t;
                    rem = softfloat_shortShiftLeft128(
                        rem.v64,
                        rem.v0,
                        (expDiff + 30 as i32 as i64) as uint_fast8_t,
                    );
                    term = softfloat_mul128By32(sigB.v64, sigB.v0, q as uint32_t);
                    rem = softfloat_sub128(rem.v64, rem.v0, term.v64, term.v0);
                    if rem.v64 & 0x8000000000000000 as u64 != 0 {
                        altRem = softfloat_add128(rem.v64, rem.v0, sigB.v64, sigB.v0);
                        current_block = 1018914860367004410;
                    } else {
                        current_block = 5159818223158340697;
                    }
                }
                match current_block {
                    5159818223158340697 => {
                        loop {
                            altRem = rem;
                            q = q.wrapping_add(1);
                            rem = softfloat_sub128(rem.v64, rem.v0, sigB.v64, sigB.v0);
                            if !(rem.v64 & 0x8000000000000000 as u64 == 0) {
                                break;
                            }
                        }
                    }
                    _ => {}
                }
                meanRem = softfloat_add128(rem.v64, rem.v0, altRem.v64, altRem.v0);
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
                return softfloat_normRoundPackToF128(
                    signRem,
                    expB - 1 as i32 as i64,
                    rem.v64,
                    rem.v0,
                );
            }
        }
    }
    match current_block {
        14125127626113485617 => {
            uiZ = softfloat_propagateNaNF128UI(uiA64, uiA0, uiB64, uiB0);
        }
        _ => {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
            uiZ.v64 = 0xffff800000000000 as u64;
            uiZ.v0 = 0 as u64;
        }
    }
    uZ.ui = uiZ;
    return uZ.f;
}
