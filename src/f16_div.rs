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
pub unsafe fn f16_div(mut a: float16_t, mut b: float16_t) -> float16_t {
    let mut current_block: u64;
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiA: uint_fast16_t = 0;
    let mut signA: bool = false;
    let mut expA: int_fast8_t = 0;
    let mut sigA: uint_fast16_t = 0;
    let mut uB: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiB: uint_fast16_t = 0;
    let mut signB: bool = false;
    let mut expB: int_fast8_t = 0;
    let mut sigB: uint_fast16_t = 0;
    let mut signZ: bool = false;
    let mut normExpSig: exp8_sig16 = exp8_sig16 { exp: 0, sig: 0 };
    let mut expZ: int_fast8_t = 0;
    let mut sig32A: uint_fast32_t = 0;
    let mut sigZ: uint_fast16_t = 0;
    let mut uiZ: uint_fast16_t = 0;
    let mut uZ: ui16_f16 = ui16_f16 { ui: 0 };
    uA.f = a;
    uiA = uA.ui as uint_fast16_t;
    signA = uiA as uint16_t as i32 >> 15 as i32 != 0;
    expA = ((uiA >> 10 as i32) as int_fast8_t as i32
        & 0x1f as i32) as int_fast8_t;
    sigA = uiA & 0x3ff as i32 as u64;
    uB.f = b;
    uiB = uB.ui as uint_fast16_t;
    signB = uiB as uint16_t as i32 >> 15 as i32 != 0;
    expB = ((uiB >> 10 as i32) as int_fast8_t as i32
        & 0x1f as i32) as int_fast8_t;
    sigB = uiB & 0x3ff as i32 as u64;
    signZ = signA as i32 ^ signB as i32 != 0;
    if expA as i32 == 0x1f as i32 {
        if sigA != 0 {
            current_block = 12543407592600593542;
        } else if expB as i32 == 0x1f as i32 {
            if sigB != 0 {
                current_block = 12543407592600593542;
            } else {
                current_block = 9540299977535002332;
            }
        } else {
            current_block = 8147314017596554528;
        }
    } else {
        if expB as i32 == 0x1f as i32 {
            if sigB != 0 {
                current_block = 12543407592600593542;
            } else {
                current_block = 15400136379169198643;
            }
        } else {
            if expB == 0 {
                if sigB == 0 {
                    if expA as u64 | sigA == 0 {
                        current_block = 9540299977535002332;
                    } else {
                        softfloat_raiseFlags(
                            softfloat_flag_infinite as i32 as uint_fast8_t,
                        );
                        current_block = 8147314017596554528;
                    }
                } else {
                    normExpSig = softfloat_normSubnormalF16Sig(sigB);
                    expB = normExpSig.exp;
                    sigB = normExpSig.sig;
                    current_block = 7056779235015430508;
                }
            } else {
                current_block = 7056779235015430508;
            }
            match current_block {
                8147314017596554528 => {}
                9540299977535002332 => {}
                _ => {
                    if expA == 0 {
                        if sigA == 0 {
                            current_block = 15400136379169198643;
                        } else {
                            normExpSig = softfloat_normSubnormalF16Sig(sigA);
                            expA = normExpSig.exp;
                            sigA = normExpSig.sig;
                            current_block = 11459959175219260272;
                        }
                    } else {
                        current_block = 11459959175219260272;
                    }
                    match current_block {
                        15400136379169198643 => {}
                        _ => {
                            expZ = (expA as i32 - expB as i32
                                + 0xe as i32) as int_fast8_t;
                            sigA |= 0x400 as i32 as u64;
                            sigB |= 0x400 as i32 as u64;
                            if sigA < sigB {
                                expZ -= 1;
                                sig32A = sigA << 15 as i32;
                            } else {
                                sig32A = sigA << 14 as i32;
                            }
                            sigZ = sig32A.wrapping_div(sigB);
                            if sigZ & 7 as i32 as u64 == 0 {
                                sigZ
                                    |= (sigB.wrapping_mul(sigZ) != sig32A) as i32
                                        as u64;
                            }
                            return softfloat_roundPackToF16(
                                signZ,
                                expZ as int_fast16_t,
                                sigZ,
                            );
                        }
                    }
                }
            }
        }
        match current_block {
            12543407592600593542 => {}
            8147314017596554528 => {}
            9540299977535002332 => {}
            _ => {
                uiZ = (((signZ as uint16_t as i32) << 15 as i32)
                    + ((0 as i32 as uint16_t as i32)
                        << 10 as i32) + 0 as i32) as uint_fast16_t;
                current_block = 15379305755115942022;
            }
        }
    }
    match current_block {
        9540299977535002332 => {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
            uiZ = 0xfe00 as i32 as uint_fast16_t;
        }
        12543407592600593542 => {
            uiZ = softfloat_propagateNaNF16UI(uiA, uiB);
        }
        8147314017596554528 => {
            uiZ = (((signZ as uint16_t as i32) << 15 as i32)
                + ((0x1f as i32 as uint16_t as i32) << 10 as i32)
                + 0 as i32) as uint_fast16_t;
        }
        _ => {}
    }
    uZ.ui = uiZ as uint16_t;
    return uZ.f;
}
