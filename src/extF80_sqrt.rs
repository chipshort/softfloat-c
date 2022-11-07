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
pub unsafe fn extF80_sqrt(mut a: extFloat80_t) -> extFloat80_t {
    let mut current_block: u64;
    let mut uA: C2RustUnnamed_2 = C2RustUnnamed_2 {
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
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uiZ64: uint_fast16_t = 0;
    let mut uiZ0: uint_fast64_t = 0;
    let mut normExpSig: exp32_sig64 = exp32_sig64 { exp: 0, sig: 0 };
    let mut expZ: int_fast32_t = 0;
    let mut sig32A: uint_fast32_t = 0;
    let mut recipSqrt32: uint_fast32_t = 0;
    let mut sig32Z: uint_fast32_t = 0;
    let mut rem: uint128 = uint128 { v0: 0, v64: 0 };
    let mut q: uint_fast64_t = 0;
    let mut x64: uint_fast64_t = 0;
    let mut sigZ: uint_fast64_t = 0;
    let mut y: uint128 = uint128 { v0: 0, v64: 0 };
    let mut term: uint128 = uint128 { v0: 0, v64: 0 };
    let mut sigZExtra: uint_fast64_t = 0;
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
    if expA == 0x7fff as i32 as i64 {
        if sigA & 0x7fffffffffffffff as u64 != 0 {
            uiZ = softfloat_propagateNaNExtF80UI(
                uiA64,
                uiA0,
                0 as i32 as uint_fast16_t,
                0 as i32 as uint_fast64_t,
            );
            uiZ64 = uiZ.v64;
            uiZ0 = uiZ.v0;
            current_block = 10534534392319843906;
        } else {
            if !signA {
                return a;
            }
            current_block = 2303551950970484471;
        }
    } else {
        if signA {
            if sigA == 0 {
                current_block = 4378840088929072219;
            } else {
                current_block = 2303551950970484471;
            }
        } else {
            if expA == 0 {
                expA = 1 as i32 as int_fast32_t;
            }
            if sigA & 0x8000000000000000 as u64 == 0 {
                if sigA == 0 {
                    current_block = 4378840088929072219;
                } else {
                    normExpSig = softfloat_normSubnormalExtF80Sig(sigA);
                    expA += normExpSig.exp;
                    sigA = normExpSig.sig;
                    current_block = 13472856163611868459;
                }
            } else {
                current_block = 13472856163611868459;
            }
            match current_block {
                4378840088929072219 => {}
                _ => {
                    expZ = (expA - 0x3fff as i32 as i64
                        >> 1 as i32) + 0x3fff as i32 as i64;
                    expA &= 1 as i32 as i64;
                    sig32A = sigA >> 32 as i32;
                    recipSqrt32 = softfloat_approxRecipSqrt32_1(
                        expA as u32,
                        sig32A as uint32_t,
                    ) as uint_fast32_t;
                    sig32Z = sig32A.wrapping_mul(recipSqrt32) >> 32 as i32;
                    if expA != 0 {
                        sig32Z >>= 1 as i32;
                        rem = softfloat_shortShiftLeft128(
                            0 as i32 as uint64_t,
                            sigA,
                            61 as i32 as uint_fast8_t,
                        );
                    } else {
                        rem = softfloat_shortShiftLeft128(
                            0 as i32 as uint64_t,
                            sigA,
                            62 as i32 as uint_fast8_t,
                        );
                    }
                    rem
                        .v64 = (rem.v64 as u64)
                        .wrapping_sub(sig32Z.wrapping_mul(sig32Z)) as uint64_t
                        as uint64_t;
                    q = ((rem.v64 >> 2 as i32) as uint32_t as u64)
                        .wrapping_mul(recipSqrt32) >> 32 as i32;
                    x64 = sig32Z << 32 as i32;
                    sigZ = x64.wrapping_add(q << 3 as i32);
                    y = softfloat_shortShiftLeft128(
                        rem.v64,
                        rem.v0,
                        29 as i32 as uint_fast8_t,
                    );
                    loop {
                        term = softfloat_mul64ByShifted32To128(
                            x64.wrapping_add(sigZ),
                            q as uint32_t,
                        );
                        rem = softfloat_sub128(y.v64, y.v0, term.v64, term.v0);
                        if rem.v64 & 0x8000000000000000 as u64 == 0 {
                            break;
                        }
                        q = q.wrapping_sub(1);
                        sigZ = (sigZ as u64)
                            .wrapping_sub(
                                ((1 as i32) << 3 as i32) as u64,
                            ) as uint_fast64_t as uint_fast64_t;
                    }
                    q = ((rem.v64 >> 2 as i32).wrapping_mul(recipSqrt32)
                        >> 32 as i32)
                        .wrapping_add(2 as i32 as u64);
                    x64 = sigZ;
                    sigZ = (sigZ << 1 as i32)
                        .wrapping_add(q >> 25 as i32);
                    sigZExtra = q << 39 as i32;
                    if q & 0xffffff as i32 as u64
                        <= 2 as i32 as u64
                    {
                        q &= !(0xffff as i32 as uint_fast64_t);
                        sigZExtra = q << 39 as i32;
                        term = softfloat_mul64ByShifted32To128(
                            x64.wrapping_add(q >> 27 as i32),
                            q as uint32_t,
                        );
                        x64 = ((q << 5 as i32) as uint32_t as u64)
                            .wrapping_mul(q as uint32_t as uint_fast64_t);
                        term = softfloat_add128(
                            term.v64,
                            term.v0,
                            0 as i32 as uint64_t,
                            x64,
                        );
                        rem = softfloat_shortShiftLeft128(
                            rem.v64,
                            rem.v0,
                            28 as i32 as uint_fast8_t,
                        );
                        rem = softfloat_sub128(rem.v64, rem.v0, term.v64, term.v0);
                        if rem.v64 & 0x8000000000000000 as u64 != 0 {
                            if sigZExtra == 0 {
                                sigZ = sigZ.wrapping_sub(1);
                            }
                            sigZExtra = sigZExtra.wrapping_sub(1);
                        } else if rem.v64 | rem.v0 != 0 {
                            sigZExtra |= 1 as i32 as u64;
                        }
                    }
                    return softfloat_roundPackToExtF80(
                        0 as i32 != 0,
                        expZ,
                        sigZ,
                        sigZExtra,
                        extF80_roundingPrecision,
                    );
                }
            }
        }
        match current_block {
            2303551950970484471 => {}
            _ => {
                uiZ64 = (signA as uint_fast16_t) << 15 as i32
                    | 0 as i32 as u64;
                uiZ0 = 0 as i32 as uint_fast64_t;
                current_block = 10534534392319843906;
            }
        }
    }
    match current_block {
        2303551950970484471 => {
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
