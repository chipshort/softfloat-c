use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast16_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ui: u128,
    pub s: uint128,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui64_f64 {
    pub ui: uint64_t,
    pub f: float64_t,
}

pub type C2RustUnnamed_0 = u32;
pub const softfloat_flag_invalid: C2RustUnnamed_0 = 16;
pub const softfloat_flag_infinite: C2RustUnnamed_0 = 8;
pub const softfloat_flag_overflow: C2RustUnnamed_0 = 4;
pub const softfloat_flag_underflow: C2RustUnnamed_0 = 2;
pub const softfloat_flag_inexact: C2RustUnnamed_0 = 1;
#[inline]
unsafe fn softfloat_mul64To128(mut a: uint64_t, mut b: uint64_t) -> uint128 {
    let mut uZ: C2RustUnnamed = C2RustUnnamed { ui: 0 };
    uZ.ui = (a as u128).wrapping_mul(b as u128);
    return uZ.s;
}
pub unsafe fn f64_mul(mut a: float64_t, mut b: float64_t) -> float64_t {
    let mut current_block: u64;
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: uint_fast64_t = 0;
    let mut signA: bool = false;
    let mut expA: int_fast16_t = 0;
    let mut sigA: uint_fast64_t = 0;
    let mut uB: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiB: uint_fast64_t = 0;
    let mut signB: bool = false;
    let mut expB: int_fast16_t = 0;
    let mut sigB: uint_fast64_t = 0;
    let mut signZ: bool = false;
    let mut magBits: uint_fast64_t = 0;
    let mut normExpSig: exp16_sig64 = exp16_sig64 { exp: 0, sig: 0 };
    let mut expZ: int_fast16_t = 0;
    let mut sig128Z: uint128 = uint128 { v0: 0, v64: 0 };
    let mut sigZ: uint_fast64_t = 0;
    let mut uiZ: uint_fast64_t = 0;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    uA.f = a;
    uiA = uA.ui;
    signA = uiA >> 63 as i32 != 0;
    expA = (uiA >> 52 as i32) as int_fast16_t
        & 0x7ff as i32 as i64;
    sigA = uiA & 0xfffffffffffff as u64;
    uB.f = b;
    uiB = uB.ui;
    signB = uiB >> 63 as i32 != 0;
    expB = (uiB >> 52 as i32) as int_fast16_t
        & 0x7ff as i32 as i64;
    sigB = uiB & 0xfffffffffffff as u64;
    signZ = signA as i32 ^ signB as i32 != 0;
    if expA == 0x7ff as i32 as i64 {
        if sigA != 0 || expB == 0x7ff as i32 as i64 && sigB != 0 {
            current_block = 15056456186161125840;
        } else {
            magBits = expB as u64 | sigB;
            current_block = 3069823138960970540;
        }
    } else if expB == 0x7ff as i32 as i64 {
        if sigB != 0 {
            current_block = 15056456186161125840;
        } else {
            magBits = expA as u64 | sigA;
            current_block = 3069823138960970540;
        }
    } else {
        if expA == 0 {
            if sigA == 0 {
                current_block = 3544437690877523161;
            } else {
                normExpSig = softfloat_normSubnormalF64Sig(sigA);
                expA = normExpSig.exp;
                sigA = normExpSig.sig;
                current_block = 11307063007268554308;
            }
        } else {
            current_block = 11307063007268554308;
        }
        match current_block {
            11307063007268554308 => {
                if expB == 0 {
                    if sigB == 0 {
                        current_block = 3544437690877523161;
                    } else {
                        normExpSig = softfloat_normSubnormalF64Sig(sigB);
                        expB = normExpSig.exp;
                        sigB = normExpSig.sig;
                        current_block = 15925075030174552612;
                    }
                } else {
                    current_block = 15925075030174552612;
                }
                match current_block {
                    3544437690877523161 => {}
                    _ => {
                        expZ = expA + expB - 0x3ff as i32 as i64;
                        sigA = (sigA | 0x10000000000000 as u64)
                            << 10 as i32;
                        sigB = (sigB | 0x10000000000000 as u64)
                            << 11 as i32;
                        sig128Z = softfloat_mul64To128(sigA, sigB);
                        sigZ = sig128Z.v64
                            | (sig128Z.v0 != 0 as i32 as u64)
                                as i32 as u64;
                        if sigZ < 0x4000000000000000 as u64 {
                            expZ -= 1;
                            sigZ <<= 1 as i32;
                        }
                        return softfloat_roundPackToF64(signZ, expZ, sigZ);
                    }
                }
            }
            _ => {}
        }
        uiZ = ((signZ as uint_fast64_t) << 63 as i32)
            .wrapping_add((0 as i32 as uint_fast64_t) << 52 as i32)
            .wrapping_add(0 as i32 as u64);
        current_block = 5675705834591939388;
    }
    match current_block {
        3069823138960970540 => {
            if magBits == 0 {
                softfloat_raiseFlags(
                    softfloat_flag_invalid as i32 as uint_fast8_t,
                );
                uiZ = 0xfff8000000000000 as u64;
            } else {
                uiZ = ((signZ as uint_fast64_t) << 63 as i32)
                    .wrapping_add(
                        (0x7ff as i32 as uint_fast64_t) << 52 as i32,
                    )
                    .wrapping_add(0 as i32 as u64);
            }
        }
        15056456186161125840 => {
            uiZ = softfloat_propagateNaNF64UI(uiA, uiB);
        }
        _ => {}
    }
    uZ.ui = uiZ;
    return uZ.f;
}
