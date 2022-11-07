use crate::*;

pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type int_fast16_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui64_f64 {
    pub ui: uint64_t,
    pub f: float64_t,
}

pub type C2RustUnnamed = u32;
pub const softfloat_flag_invalid: C2RustUnnamed = 16;
pub const softfloat_flag_infinite: C2RustUnnamed = 8;
pub const softfloat_flag_overflow: C2RustUnnamed = 4;
pub const softfloat_flag_underflow: C2RustUnnamed = 2;
pub const softfloat_flag_inexact: C2RustUnnamed = 1;
pub unsafe fn f64_sqrt(mut a: float64_t) -> float64_t {
    let mut current_block: u64;
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: uint_fast64_t = 0;
    let mut signA: bool = false;
    let mut expA: int_fast16_t = 0;
    let mut sigA: uint_fast64_t = 0;
    let mut uiZ: uint_fast64_t = 0;
    let mut normExpSig: exp16_sig64 = exp16_sig64 { exp: 0, sig: 0 };
    let mut expZ: int_fast16_t = 0;
    let mut sig32A: uint32_t = 0;
    let mut recipSqrt32: uint32_t = 0;
    let mut sig32Z: uint32_t = 0;
    let mut rem: uint_fast64_t = 0;
    let mut q: uint32_t = 0;
    let mut sigZ: uint_fast64_t = 0;
    let mut shiftedSigZ: uint_fast64_t = 0;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    uA.f = a;
    uiA = uA.ui;
    signA = uiA >> 63 as i32 != 0;
    expA = (uiA >> 52 as i32) as int_fast16_t
        & 0x7ff as i32 as i64;
    sigA = uiA & 0xfffffffffffff as u64;
    if expA == 0x7ff as i32 as i64 {
        if sigA != 0 {
            uiZ = softfloat_propagateNaNF64UI(uiA, 0 as i32 as uint_fast64_t);
            current_block = 7224668257601483890;
        } else {
            if !signA {
                return a;
            }
            current_block = 4177575401560248200;
        }
    } else {
        if signA {
            if expA as u64 | sigA == 0 {
                return a;
            }
        } else {
            if expA == 0 {
                if sigA == 0 {
                    return a;
                }
                normExpSig = softfloat_normSubnormalF64Sig(sigA);
                expA = normExpSig.exp;
                sigA = normExpSig.sig;
            }
            expZ = (expA - 0x3ff as i32 as i64 >> 1 as i32)
                + 0x3fe as i32 as i64;
            expA &= 1 as i32 as i64;
            sigA |= 0x10000000000000 as u64;
            sig32A = (sigA >> 21 as i32) as uint32_t;
            recipSqrt32 = softfloat_approxRecipSqrt32_1(expA as u32, sig32A);
            sig32Z = ((sig32A as uint_fast64_t)
                .wrapping_mul(recipSqrt32 as u64) >> 32 as i32)
                as uint32_t;
            if expA != 0 {
                sigA <<= 8 as i32;
                sig32Z >>= 1 as i32;
            } else {
                sigA <<= 9 as i32;
            }
            rem = sigA
                .wrapping_sub(
                    (sig32Z as uint_fast64_t).wrapping_mul(sig32Z as u64),
                );
            q = (((rem >> 2 as i32) as uint32_t as u64)
                .wrapping_mul(recipSqrt32 as uint_fast64_t) >> 32 as i32)
                as uint32_t;
            sigZ = ((sig32Z as uint_fast64_t) << 32 as i32
                | ((1 as i32) << 5 as i32) as u64)
                .wrapping_add((q as uint_fast64_t) << 3 as i32);
            if (sigZ & 0x1ff as i32 as u64)
                < 0x22 as i32 as u64
            {
                sigZ &= !(0x3f as i32 as uint_fast64_t);
                shiftedSigZ = sigZ >> 6 as i32;
                rem = (sigA << 52 as i32)
                    .wrapping_sub(shiftedSigZ.wrapping_mul(shiftedSigZ));
                if rem & 0x8000000000000000 as u64 != 0 {
                    sigZ = sigZ.wrapping_sub(1);
                } else if rem != 0 {
                    sigZ |= 1 as i32 as u64;
                }
            }
            return softfloat_roundPackToF64(0 as i32 != 0, expZ, sigZ);
        }
        current_block = 4177575401560248200;
    }
    match current_block {
        4177575401560248200 => {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
            uiZ = 0xfff8000000000000 as u64;
        }
        _ => {}
    }
    uZ.ui = uiZ;
    return uZ.f;
}
