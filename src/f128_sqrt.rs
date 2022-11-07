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
pub union C2RustUnnamed_0 {
    pub ui: u128,
    pub s: uint128,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui128_f128 {
    pub ui: uint128,
    pub f: float128_t,
}

pub type C2RustUnnamed_1 = u32;
pub const softfloat_flag_invalid: C2RustUnnamed_1 = 16;
pub const softfloat_flag_infinite: C2RustUnnamed_1 = 8;
pub const softfloat_flag_overflow: C2RustUnnamed_1 = 4;
pub const softfloat_flag_underflow: C2RustUnnamed_1 = 2;
pub const softfloat_flag_inexact: C2RustUnnamed_1 = 1;
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
unsafe fn softfloat_mul128By32(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut b: uint32_t,
) -> uint128 {
    let mut uZ: C2RustUnnamed_0 = C2RustUnnamed_0 { ui: 0 };
    uZ.ui = ((a64 as u128) << 64 as i32 | a0 as u128).wrapping_mul(b as u128);
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
pub unsafe fn f128_sqrt(mut a: float128_t) -> float128_t {
    let mut current_block: u64;
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiA64: uint_fast64_t = 0;
    let mut uiA0: uint_fast64_t = 0;
    let mut signA: bool = false;
    let mut expA: int_fast32_t = 0;
    let mut sigA: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut normExpSig: exp32_sig128 = exp32_sig128 {
        exp: 0,
        sig: uint128 { v0: 0, v64: 0 },
    };
    let mut expZ: int_fast32_t = 0;
    let mut sig32A: uint_fast32_t = 0;
    let mut recipSqrt32: uint_fast32_t = 0;
    let mut sig32Z: uint_fast32_t = 0;
    let mut rem: uint128 = uint128 { v0: 0, v64: 0 };
    let mut qs: [uint32_t; 3] = [0; 3];
    let mut q: uint_fast32_t = 0;
    let mut x64: uint_fast64_t = 0;
    let mut sig64Z: uint_fast64_t = 0;
    let mut y: uint128 = uint128 { v0: 0, v64: 0 };
    let mut term: uint128 = uint128 { v0: 0, v64: 0 };
    let mut sigZExtra: uint_fast64_t = 0;
    let mut sigZ: uint128 = uint128 { v0: 0, v64: 0 };
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
    if expA == 0x7fff as i32 as i64 {
        if sigA.v64 | sigA.v0 != 0 {
            uiZ = softfloat_propagateNaNF128UI(
                uiA64,
                uiA0,
                0 as i32 as uint_fast64_t,
                0 as i32 as uint_fast64_t,
            );
            current_block = 4946023159180306790;
        } else {
            if !signA {
                return a;
            }
            current_block = 561480683784283636;
        }
    } else {
        if signA {
            if expA as u64 | sigA.v64 | sigA.v0 == 0 {
                return a;
            }
        } else {
            if expA == 0 {
                if sigA.v64 | sigA.v0 == 0 {
                    return a;
                }
                normExpSig = softfloat_normSubnormalF128Sig(sigA.v64, sigA.v0);
                expA = normExpSig.exp;
                sigA = normExpSig.sig;
            }
            expZ = (expA - 0x3fff as i32 as i64 >> 1 as i32)
                + 0x3ffe as i32 as i64;
            expA &= 1 as i32 as i64;
            sigA.v64 |= 0x1000000000000 as u64;
            sig32A = sigA.v64 >> 17 as i32;
            recipSqrt32 = softfloat_approxRecipSqrt32_1(
                expA as u32,
                sig32A as uint32_t,
            ) as uint_fast32_t;
            sig32Z = sig32A.wrapping_mul(recipSqrt32) >> 32 as i32;
            if expA != 0 {
                sig32Z >>= 1 as i32;
                rem = softfloat_shortShiftLeft128(
                    sigA.v64,
                    sigA.v0,
                    12 as i32 as uint_fast8_t,
                );
            } else {
                rem = softfloat_shortShiftLeft128(
                    sigA.v64,
                    sigA.v0,
                    13 as i32 as uint_fast8_t,
                );
            }
            qs[2 as i32 as usize] = sig32Z as uint32_t;
            rem
                .v64 = (rem.v64 as u64)
                .wrapping_sub(sig32Z.wrapping_mul(sig32Z)) as uint64_t as uint64_t;
            q = ((rem.v64 >> 2 as i32) as uint32_t as u64)
                .wrapping_mul(recipSqrt32) >> 32 as i32;
            x64 = sig32Z << 32 as i32;
            sig64Z = x64.wrapping_add(q << 3 as i32);
            y = softfloat_shortShiftLeft128(
                rem.v64,
                rem.v0,
                29 as i32 as uint_fast8_t,
            );
            loop {
                term = softfloat_mul64ByShifted32To128(
                    x64.wrapping_add(sig64Z),
                    q as uint32_t,
                );
                rem = softfloat_sub128(y.v64, y.v0, term.v64, term.v0);
                if rem.v64 & 0x8000000000000000 as u64 == 0 {
                    break;
                }
                q = q.wrapping_sub(1);
                sig64Z = (sig64Z as u64)
                    .wrapping_sub(
                        ((1 as i32) << 3 as i32) as u64,
                    ) as uint_fast64_t as uint_fast64_t;
            }
            qs[1 as i32 as usize] = q as uint32_t;
            q = (rem.v64 >> 2 as i32).wrapping_mul(recipSqrt32)
                >> 32 as i32;
            y = softfloat_shortShiftLeft128(
                rem.v64,
                rem.v0,
                29 as i32 as uint_fast8_t,
            );
            sig64Z <<= 1 as i32;
            loop {
                term = softfloat_shortShiftLeft128(
                    0 as i32 as uint64_t,
                    sig64Z,
                    32 as i32 as uint_fast8_t,
                );
                term = softfloat_add128(
                    term.v64,
                    term.v0,
                    0 as i32 as uint64_t,
                    q << 6 as i32,
                );
                term = softfloat_mul128By32(term.v64, term.v0, q as uint32_t);
                rem = softfloat_sub128(y.v64, y.v0, term.v64, term.v0);
                if rem.v64 & 0x8000000000000000 as u64 == 0 {
                    break;
                }
                q = q.wrapping_sub(1);
            }
            qs[0 as i32 as usize] = q as uint32_t;
            q = ((rem.v64 >> 2 as i32).wrapping_mul(recipSqrt32)
                >> 32 as i32)
                .wrapping_add(2 as i32 as u64);
            sigZExtra = q << 59 as i32;
            term = softfloat_shortShiftLeft128(
                0 as i32 as uint64_t,
                qs[1 as i32 as usize] as uint64_t,
                53 as i32 as uint_fast8_t,
            );
            sigZ = softfloat_add128(
                (qs[2 as i32 as usize] as uint_fast64_t) << 18 as i32,
                ((qs[0 as i32 as usize] as uint_fast64_t) << 24 as i32)
                    .wrapping_add(q >> 5 as i32),
                term.v64,
                term.v0,
            );
            if q & 0xf as i32 as u64
                <= 2 as i32 as u64
            {
                q &= !(3 as i32) as u64;
                sigZExtra = q << 59 as i32;
                y = softfloat_shortShiftLeft128(
                    sigZ.v64,
                    sigZ.v0,
                    6 as i32 as uint_fast8_t,
                );
                y.v0 |= sigZExtra >> 58 as i32;
                term = softfloat_sub128(y.v64, y.v0, 0 as i32 as uint64_t, q);
                y = softfloat_mul64ByShifted32To128(term.v0, q as uint32_t);
                term = softfloat_mul64ByShifted32To128(term.v64, q as uint32_t);
                term = softfloat_add128(
                    term.v64,
                    term.v0,
                    0 as i32 as uint64_t,
                    y.v64,
                );
                rem = softfloat_shortShiftLeft128(
                    rem.v64,
                    rem.v0,
                    20 as i32 as uint_fast8_t,
                );
                term = softfloat_sub128(term.v64, term.v0, rem.v64, rem.v0);
                if term.v64 & 0x8000000000000000 as u64 != 0 {
                    sigZExtra |= 1 as i32 as u64;
                } else if term.v64 | term.v0 | y.v0 != 0 {
                    if sigZExtra != 0 {
                        sigZExtra = sigZExtra.wrapping_sub(1);
                    } else {
                        sigZ = softfloat_sub128(
                            sigZ.v64,
                            sigZ.v0,
                            0 as i32 as uint64_t,
                            1 as i32 as uint64_t,
                        );
                        sigZExtra = !(0 as i32) as uint_fast64_t;
                    }
                }
            }
            return softfloat_roundPackToF128(
                0 as i32 != 0,
                expZ,
                sigZ.v64,
                sigZ.v0,
                sigZExtra,
            );
        }
        current_block = 561480683784283636;
    }
    match current_block {
        561480683784283636 => {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
            uiZ.v64 = 0xffff800000000000 as u64;
            uiZ.v0 = 0 as u64;
        }
        _ => {}
    }
    uZ.ui = uiZ;
    return uZ.f;
}
