use crate::*;

pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub unsafe fn f32_mul(mut a: float32_t, mut b: float32_t) -> float32_t {
    let mut current_block: u64;
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiA: uint_fast32_t = 0;
    let mut signA: bool = false;
    let mut expA: int_fast16_t = 0;
    let mut sigA: uint_fast32_t = 0;
    let mut uB: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiB: uint_fast32_t = 0;
    let mut signB: bool = false;
    let mut expB: int_fast16_t = 0;
    let mut sigB: uint_fast32_t = 0;
    let mut signZ: bool = false;
    let mut magBits: uint_fast32_t = 0;
    let mut normExpSig: exp16_sig32 = exp16_sig32 { exp: 0, sig: 0 };
    let mut expZ: int_fast16_t = 0;
    let mut sigZ: uint_fast32_t = 0;
    let mut uiZ: uint_fast32_t = 0;
    let mut uZ: ui32_f32 = ui32_f32 { ui: 0 };
    uA.f = a;
    uiA = uA.ui as uint_fast32_t;
    signA = uiA as uint32_t >> 31 as i32 != 0;
    expA = (uiA >> 23 as i32) as int_fast16_t
        & 0xff as i32 as i64;
    sigA = uiA & 0x7fffff as i32 as u64;
    uB.f = b;
    uiB = uB.ui as uint_fast32_t;
    signB = uiB as uint32_t >> 31 as i32 != 0;
    expB = (uiB >> 23 as i32) as int_fast16_t
        & 0xff as i32 as i64;
    sigB = uiB & 0x7fffff as i32 as u64;
    signZ = signA as i32 ^ signB as i32 != 0;
    if expA == 0xff as i32 as i64 {
        if sigA != 0 || expB == 0xff as i32 as i64 && sigB != 0 {
            current_block = 10947527268379419398;
        } else {
            magBits = expB as u64 | sigB;
            current_block = 12286261574051090798;
        }
    } else if expB == 0xff as i32 as i64 {
        if sigB != 0 {
            current_block = 10947527268379419398;
        } else {
            magBits = expA as u64 | sigA;
            current_block = 12286261574051090798;
        }
    } else {
        if expA == 0 {
            if sigA == 0 {
                current_block = 12877898249637869080;
            } else {
                normExpSig = softfloat_normSubnormalF32Sig(sigA);
                expA = normExpSig.exp;
                sigA = normExpSig.sig;
                current_block = 14401909646449704462;
            }
        } else {
            current_block = 14401909646449704462;
        }
        match current_block {
            14401909646449704462 => {
                if expB == 0 {
                    if sigB == 0 {
                        current_block = 12877898249637869080;
                    } else {
                        normExpSig = softfloat_normSubnormalF32Sig(sigB);
                        expB = normExpSig.exp;
                        sigB = normExpSig.sig;
                        current_block = 14818589718467733107;
                    }
                } else {
                    current_block = 14818589718467733107;
                }
                match current_block {
                    12877898249637869080 => {}
                    _ => {
                        expZ = expA + expB - 0x7f as i32 as i64;
                        sigA = (sigA | 0x800000 as i32 as u64)
                            << 7 as i32;
                        sigB = (sigB | 0x800000 as i32 as u64)
                            << 8 as i32;
                        sigZ = softfloat_shortShiftRightJam64(
                            sigA.wrapping_mul(sigB),
                            32 as i32 as uint_fast8_t,
                        );
                        if sigZ < 0x40000000 as i32 as u64 {
                            expZ -= 1;
                            sigZ <<= 1 as i32;
                        }
                        return softfloat_roundPackToF32(signZ, expZ, sigZ);
                    }
                }
            }
            _ => {}
        }
        uiZ = ((signZ as uint32_t) << 31 as i32)
            .wrapping_add((0 as i32 as uint32_t) << 23 as i32)
            .wrapping_add(0 as i32 as u32) as uint_fast32_t;
        current_block = 13809288849401763248;
    }
    match current_block {
        12286261574051090798 => {
            if magBits == 0 {
                softfloat_raiseFlags(
                    softfloat_flag_invalid as i32 as uint_fast8_t,
                );
                uiZ = 0xffc00000 as u32 as uint_fast32_t;
            } else {
                uiZ = ((signZ as uint32_t) << 31 as i32)
                    .wrapping_add((0xff as i32 as uint32_t) << 23 as i32)
                    .wrapping_add(0 as i32 as u32) as uint_fast32_t;
            }
        }
        10947527268379419398 => {
            uiZ = softfloat_propagateNaNF32UI(uiA, uiB);
        }
        _ => {}
    }
    uZ.ui = uiZ as uint32_t;
    return uZ.f;
}
