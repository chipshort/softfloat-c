use crate::*;

pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
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
pub const softfloat_flag_invalid: C2RustUnnamed = 16;
pub const softfloat_flag_infinite: C2RustUnnamed = 8;
pub const softfloat_flag_overflow: C2RustUnnamed = 4;
pub const softfloat_flag_underflow: C2RustUnnamed = 2;
pub const softfloat_flag_inexact: C2RustUnnamed = 1;
pub unsafe fn f32_sqrt(mut a: float32_t) -> float32_t {
    let mut current_block: u64;
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiA: uint_fast32_t = 0;
    let mut signA: bool = false;
    let mut expA: int_fast16_t = 0;
    let mut sigA: uint_fast32_t = 0;
    let mut uiZ: uint_fast32_t = 0;
    let mut normExpSig: exp16_sig32 = exp16_sig32 { exp: 0, sig: 0 };
    let mut expZ: int_fast16_t = 0;
    let mut sigZ: uint_fast32_t = 0;
    let mut shiftedSigZ: uint_fast32_t = 0;
    let mut negRem: uint32_t = 0;
    let mut uZ: ui32_f32 = ui32_f32 { ui: 0 };
    uA.f = a;
    uiA = uA.ui as uint_fast32_t;
    signA = uiA as uint32_t >> 31 as i32 != 0;
    expA = (uiA >> 23 as i32) as int_fast16_t
        & 0xff as i32 as i64;
    sigA = uiA & 0x7fffff as i32 as u64;
    if expA == 0xff as i32 as i64 {
        if sigA != 0 {
            uiZ = softfloat_propagateNaNF32UI(uiA, 0 as i32 as uint_fast32_t);
            current_block = 13809288849401763248;
        } else {
            if !signA {
                return a;
            }
            current_block = 11113987368093462495;
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
                normExpSig = softfloat_normSubnormalF32Sig(sigA);
                expA = normExpSig.exp;
                sigA = normExpSig.sig;
            }
            expZ = (expA - 0x7f as i32 as i64 >> 1 as i32)
                + 0x7e as i32 as i64;
            expA &= 1 as i32 as i64;
            sigA = (sigA | 0x800000 as i32 as u64) << 8 as i32;
            sigZ = sigA
                .wrapping_mul(
                    softfloat_approxRecipSqrt32_1(expA as u32, sigA as uint32_t)
                        as u64,
                ) >> 32 as i32;
            if expA != 0 {
                sigZ >>= 1 as i32;
            }
            sigZ = (sigZ as u64)
                .wrapping_add(2 as i32 as u64) as uint_fast32_t
                as uint_fast32_t;
            if (sigZ & 0x3f as i32 as u64)
                < 2 as i32 as u64
            {
                shiftedSigZ = sigZ >> 2 as i32;
                negRem = shiftedSigZ.wrapping_mul(shiftedSigZ) as uint32_t;
                sigZ &= !(3 as i32) as u64;
                if negRem & 0x80000000 as u32 != 0 {
                    sigZ |= 1 as i32 as u64;
                } else if negRem != 0 {
                    sigZ = sigZ.wrapping_sub(1);
                }
            }
            return softfloat_roundPackToF32(0 as i32 != 0, expZ, sigZ);
        }
        current_block = 11113987368093462495;
    }
    match current_block {
        11113987368093462495 => {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
            uiZ = 0xffc00000 as u32 as uint_fast32_t;
        }
        _ => {}
    }
    uZ.ui = uiZ as uint32_t;
    return uZ.f;
}
