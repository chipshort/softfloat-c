use crate::*;

pub type __uint16_t = u16;
pub type uint16_t = __uint16_t;
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
pub const softfloat_flag_invalid: C2RustUnnamed = 16;
pub const softfloat_flag_infinite: C2RustUnnamed = 8;
pub const softfloat_flag_overflow: C2RustUnnamed = 4;
pub const softfloat_flag_underflow: C2RustUnnamed = 2;
pub const softfloat_flag_inexact: C2RustUnnamed = 1;
pub unsafe fn f16_sqrt(mut a: float16_t) -> float16_t {
    let mut current_block: u64;
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiA: uint_fast16_t = 0;
    let mut signA: bool = false;
    let mut expA: int_fast8_t = 0;
    let mut sigA: uint_fast16_t = 0;
    let mut uiZ: uint_fast16_t = 0;
    let mut normExpSig: exp8_sig16 = exp8_sig16 { exp: 0, sig: 0 };
    let mut expZ: int_fast8_t = 0;
    let mut index: i32 = 0;
    let mut r0: uint_fast16_t = 0;
    let mut ESqrR0: uint_fast32_t = 0;
    let mut sigma0: uint16_t = 0;
    let mut recipSqrt16: uint_fast16_t = 0;
    let mut sigZ: uint_fast16_t = 0;
    let mut shiftedSigZ: uint_fast16_t = 0;
    let mut negRem: uint16_t = 0;
    let mut uZ: ui16_f16 = ui16_f16 { ui: 0 };
    uA.f = a;
    uiA = uA.ui as uint_fast16_t;
    signA = uiA as uint16_t as i32 >> 15 as i32 != 0;
    expA = ((uiA >> 10 as i32) as int_fast8_t as i32
        & 0x1f as i32) as int_fast8_t;
    sigA = uiA & 0x3ff as i32 as u64;
    if expA as i32 == 0x1f as i32 {
        if sigA != 0 {
            uiZ = softfloat_propagateNaNF16UI(uiA, 0 as i32 as uint_fast16_t);
            current_block = 17168366175729252995;
        } else {
            if !signA {
                return a;
            }
            current_block = 1890627689740379930;
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
                normExpSig = softfloat_normSubnormalF16Sig(sigA);
                expA = normExpSig.exp;
                sigA = normExpSig.sig;
            }
            expZ = ((expA as i32 - 0xf as i32 >> 1 as i32)
                + 0xe as i32) as int_fast8_t;
            expA = (expA as i32 & 1 as i32) as int_fast8_t;
            sigA |= 0x400 as i32 as u64;
            index = (sigA >> 6 as i32 & 0xe as i32 as u64)
                .wrapping_add(expA as u64) as i32;
            r0 = (softfloat_approxRecipSqrt_1k0s[index as usize] as u64)
                .wrapping_sub(
                    (softfloat_approxRecipSqrt_1k1s[index as usize] as uint_fast32_t)
                        .wrapping_mul(sigA & 0x7f as i32 as u64)
                        >> 11 as i32,
                );
            ESqrR0 = r0.wrapping_mul(r0) >> 1 as i32;
            if expA != 0 {
                ESqrR0 >>= 1 as i32;
            }
            sigma0 = !(ESqrR0.wrapping_mul(sigA) >> 16 as i32) as uint16_t;
            recipSqrt16 = r0
                .wrapping_add(
                    r0.wrapping_mul(sigma0 as u64) >> 25 as i32,
                );
            if recipSqrt16 & 0x8000 as i32 as u64 == 0 {
                recipSqrt16 = 0x8000 as i32 as uint_fast16_t;
            }
            sigZ = (sigA << 5 as i32).wrapping_mul(recipSqrt16)
                >> 16 as i32;
            if expA != 0 {
                sigZ >>= 1 as i32;
            }
            sigZ = sigZ.wrapping_add(1);
            if sigZ & 7 as i32 as u64 == 0 {
                shiftedSigZ = sigZ >> 1 as i32;
                negRem = shiftedSigZ.wrapping_mul(shiftedSigZ) as uint16_t;
                sigZ &= !(1 as i32) as u64;
                if negRem as i32 & 0x8000 as i32 != 0 {
                    sigZ |= 1 as i32 as u64;
                } else if negRem != 0 {
                    sigZ = sigZ.wrapping_sub(1);
                }
            }
            return softfloat_roundPackToF16(
                0 as i32 != 0,
                expZ as int_fast16_t,
                sigZ,
            );
        }
        current_block = 1890627689740379930;
    }
    match current_block {
        1890627689740379930 => {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
            uiZ = 0xfe00 as i32 as uint_fast16_t;
        }
        _ => {}
    }
    uZ.ui = uiZ as uint16_t;
    return uZ.f;
}
