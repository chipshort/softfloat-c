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
pub unsafe fn f64_div(mut a: float64_t, mut b: float64_t) -> float64_t {
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
    let mut normExpSig: exp16_sig64 = exp16_sig64 { exp: 0, sig: 0 };
    let mut expZ: int_fast16_t = 0;
    let mut recip32: uint32_t = 0;
    let mut sig32Z: uint32_t = 0;
    let mut doubleTerm: uint32_t = 0;
    let mut rem: uint_fast64_t = 0;
    let mut q: uint32_t = 0;
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
        if sigA != 0 {
            current_block = 10236918244844113390;
        } else if expB == 0x7ff as i32 as i64 {
            if sigB != 0 {
                current_block = 10236918244844113390;
            } else {
                current_block = 9334783689646240567;
            }
        } else {
            current_block = 3178586857448608340;
        }
    } else {
        if expB == 0x7ff as i32 as i64 {
            if sigB != 0 {
                current_block = 10236918244844113390;
            } else {
                current_block = 16052719353329760691;
            }
        } else {
            if expB == 0 {
                if sigB == 0 {
                    if expA as u64 | sigA == 0 {
                        current_block = 9334783689646240567;
                    } else {
                        softfloat_raiseFlags(
                            softfloat_flag_infinite as i32 as uint_fast8_t,
                        );
                        current_block = 3178586857448608340;
                    }
                } else {
                    normExpSig = softfloat_normSubnormalF64Sig(sigB);
                    expB = normExpSig.exp;
                    sigB = normExpSig.sig;
                    current_block = 14763689060501151050;
                }
            } else {
                current_block = 14763689060501151050;
            }
            match current_block {
                3178586857448608340 => {}
                9334783689646240567 => {}
                _ => {
                    if expA == 0 {
                        if sigA == 0 {
                            current_block = 16052719353329760691;
                        } else {
                            normExpSig = softfloat_normSubnormalF64Sig(sigA);
                            expA = normExpSig.exp;
                            sigA = normExpSig.sig;
                            current_block = 15897653523371991391;
                        }
                    } else {
                        current_block = 15897653523371991391;
                    }
                    match current_block {
                        16052719353329760691 => {}
                        _ => {
                            expZ = expA - expB + 0x3fe as i32 as i64;
                            sigA |= 0x10000000000000 as u64;
                            sigB |= 0x10000000000000 as u64;
                            if sigA < sigB {
                                expZ -= 1;
                                sigA <<= 11 as i32;
                            } else {
                                sigA <<= 10 as i32;
                            }
                            sigB <<= 11 as i32;
                            recip32 = ((0x7fffffffffffffff as u64)
                                .wrapping_div(
                                    (sigB >> 32 as i32) as uint32_t as u64,
                                ) as uint32_t)
                                .wrapping_sub(2 as i32 as u32);
                            sig32Z = (((sigA >> 32 as i32) as uint32_t
                                as u64)
                                .wrapping_mul(recip32 as uint_fast64_t)
                                >> 32 as i32) as uint32_t;
                            doubleTerm = sig32Z << 1 as i32;
                            rem = (sigA
                                .wrapping_sub(
                                    (doubleTerm as uint_fast64_t)
                                        .wrapping_mul(
                                            (sigB >> 32 as i32) as uint32_t as u64,
                                        ),
                                ) << 28 as i32)
                                .wrapping_sub(
                                    (doubleTerm as uint_fast64_t)
                                        .wrapping_mul(
                                            (sigB as uint32_t >> 4 as i32) as u64,
                                        ),
                                );
                            q = (((rem >> 32 as i32) as uint32_t
                                as u64)
                                .wrapping_mul(recip32 as uint_fast64_t)
                                >> 32 as i32)
                                .wrapping_add(4 as i32 as u64)
                                as uint32_t;
                            sigZ = ((sig32Z as uint_fast64_t) << 32 as i32)
                                .wrapping_add((q as uint_fast64_t) << 4 as i32);
                            if (sigZ & 0x1ff as i32 as u64)
                                < ((4 as i32) << 4 as i32) as u64
                            {
                                q &= !(7 as i32) as u32;
                                sigZ &= !(0x7f as i32 as uint_fast64_t);
                                doubleTerm = q << 1 as i32;
                                rem = (rem
                                    .wrapping_sub(
                                        (doubleTerm as uint_fast64_t)
                                            .wrapping_mul(
                                                (sigB >> 32 as i32) as uint32_t as u64,
                                            ),
                                    ) << 28 as i32)
                                    .wrapping_sub(
                                        (doubleTerm as uint_fast64_t)
                                            .wrapping_mul(
                                                (sigB as uint32_t >> 4 as i32) as u64,
                                            ),
                                    );
                                if rem & 0x8000000000000000 as u64 != 0 {
                                    sigZ = (sigZ as u64)
                                        .wrapping_sub(
                                            ((1 as i32) << 7 as i32) as u64,
                                        ) as uint_fast64_t as uint_fast64_t;
                                } else if rem != 0 {
                                    sigZ |= 1 as i32 as u64;
                                }
                            }
                            return softfloat_roundPackToF64(signZ, expZ, sigZ);
                        }
                    }
                }
            }
        }
        match current_block {
            10236918244844113390 => {}
            3178586857448608340 => {}
            9334783689646240567 => {}
            _ => {
                uiZ = ((signZ as uint_fast64_t) << 63 as i32)
                    .wrapping_add(
                        (0 as i32 as uint_fast64_t) << 52 as i32,
                    )
                    .wrapping_add(0 as i32 as u64);
                current_block = 11152584042954099915;
            }
        }
    }
    match current_block {
        9334783689646240567 => {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
            uiZ = 0xfff8000000000000 as u64;
        }
        3178586857448608340 => {
            uiZ = ((signZ as uint_fast64_t) << 63 as i32)
                .wrapping_add(
                    (0x7ff as i32 as uint_fast64_t) << 52 as i32,
                )
                .wrapping_add(0 as i32 as u64);
        }
        10236918244844113390 => {
            uiZ = softfloat_propagateNaNF64UI(uiA, uiB);
        }
        _ => {}
    }
    uZ.ui = uiZ;
    return uZ.f;
}
