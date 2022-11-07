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
pub unsafe fn f32_div(mut a: float32_t, mut b: float32_t) -> float32_t {
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
    let mut normExpSig: exp16_sig32 = exp16_sig32 { exp: 0, sig: 0 };
    let mut expZ: int_fast16_t = 0;
    let mut sig64A: uint_fast64_t = 0;
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
        if sigA != 0 {
            current_block = 5846552480344433139;
        } else if expB == 0xff as i32 as i64 {
            if sigB != 0 {
                current_block = 5846552480344433139;
            } else {
                current_block = 7615071539302648478;
            }
        } else {
            current_block = 16587501557276980999;
        }
    } else {
        if expB == 0xff as i32 as i64 {
            if sigB != 0 {
                current_block = 5846552480344433139;
            } else {
                current_block = 15650985371655606026;
            }
        } else {
            if expB == 0 {
                if sigB == 0 {
                    if expA as u64 | sigA == 0 {
                        current_block = 7615071539302648478;
                    } else {
                        softfloat_raiseFlags(
                            softfloat_flag_infinite as i32 as uint_fast8_t,
                        );
                        current_block = 16587501557276980999;
                    }
                } else {
                    normExpSig = softfloat_normSubnormalF32Sig(sigB);
                    expB = normExpSig.exp;
                    sigB = normExpSig.sig;
                    current_block = 7056779235015430508;
                }
            } else {
                current_block = 7056779235015430508;
            }
            match current_block {
                16587501557276980999 => {}
                7615071539302648478 => {}
                _ => {
                    if expA == 0 {
                        if sigA == 0 {
                            current_block = 15650985371655606026;
                        } else {
                            normExpSig = softfloat_normSubnormalF32Sig(sigA);
                            expA = normExpSig.exp;
                            sigA = normExpSig.sig;
                            current_block = 11459959175219260272;
                        }
                    } else {
                        current_block = 11459959175219260272;
                    }
                    match current_block {
                        15650985371655606026 => {}
                        _ => {
                            expZ = expA - expB + 0x7e as i32 as i64;
                            sigA |= 0x800000 as i32 as u64;
                            sigB |= 0x800000 as i32 as u64;
                            if sigA < sigB {
                                expZ -= 1;
                                sig64A = sigA << 31 as i32;
                            } else {
                                sig64A = sigA << 30 as i32;
                            }
                            sigZ = sig64A.wrapping_div(sigB);
                            if sigZ & 0x3f as i32 as u64 == 0 {
                                sigZ
                                    |= (sigB.wrapping_mul(sigZ) != sig64A) as i32
                                        as u64;
                            }
                            return softfloat_roundPackToF32(signZ, expZ, sigZ);
                        }
                    }
                }
            }
        }
        match current_block {
            5846552480344433139 => {}
            16587501557276980999 => {}
            7615071539302648478 => {}
            _ => {
                uiZ = ((signZ as uint32_t) << 31 as i32)
                    .wrapping_add((0 as i32 as uint32_t) << 23 as i32)
                    .wrapping_add(0 as i32 as u32) as uint_fast32_t;
                current_block = 13040394263518793624;
            }
        }
    }
    match current_block {
        7615071539302648478 => {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
            uiZ = 0xffc00000 as u32 as uint_fast32_t;
        }
        5846552480344433139 => {
            uiZ = softfloat_propagateNaNF32UI(uiA, uiB);
        }
        16587501557276980999 => {
            uiZ = ((signZ as uint32_t) << 31 as i32)
                .wrapping_add((0xff as i32 as uint32_t) << 23 as i32)
                .wrapping_add(0 as i32 as u32) as uint_fast32_t;
        }
        _ => {}
    }
    uZ.ui = uiZ as uint32_t;
    return uZ.f;
}
