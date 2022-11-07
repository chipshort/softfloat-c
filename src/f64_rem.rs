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
pub unsafe fn f64_rem(mut a: float64_t, mut b: float64_t) -> float64_t {
    let mut current_block: u64;
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: uint_fast64_t = 0;
    let mut signA: bool = false;
    let mut expA: int_fast16_t = 0;
    let mut sigA: uint_fast64_t = 0;
    let mut uB: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiB: uint_fast64_t = 0;
    let mut expB: int_fast16_t = 0;
    let mut sigB: uint_fast64_t = 0;
    let mut normExpSig: exp16_sig64 = exp16_sig64 { exp: 0, sig: 0 };
    let mut rem: uint64_t = 0;
    let mut expDiff: int_fast16_t = 0;
    let mut q: uint32_t = 0;
    let mut recip32: uint32_t = 0;
    let mut q64: uint_fast64_t = 0;
    let mut altRem: uint64_t = 0;
    let mut meanRem: uint64_t = 0;
    let mut signRem: bool = false;
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
    expB = (uiB >> 52 as i32) as int_fast16_t
        & 0x7ff as i32 as i64;
    sigB = uiB & 0xfffffffffffff as u64;
    if expA == 0x7ff as i32 as i64 {
        if sigA != 0 || expB == 0x7ff as i32 as i64 && sigB != 0 {
            current_block = 2917149414562135168;
        } else {
            current_block = 16115628854858113829;
        }
    } else if expB == 0x7ff as i32 as i64 {
        if sigB != 0 {
            current_block = 2917149414562135168;
        } else {
            return a
        }
    } else {
        if expA < expB - 1 as i32 as i64 {
            return a;
        }
        if expB == 0 {
            if sigB == 0 {
                current_block = 16115628854858113829;
            } else {
                normExpSig = softfloat_normSubnormalF64Sig(sigB);
                expB = normExpSig.exp;
                sigB = normExpSig.sig;
                current_block = 14401909646449704462;
            }
        } else {
            current_block = 14401909646449704462;
        }
        match current_block {
            16115628854858113829 => {}
            _ => {
                if expA == 0 {
                    if sigA == 0 {
                        return a;
                    }
                    normExpSig = softfloat_normSubnormalF64Sig(sigA);
                    expA = normExpSig.exp;
                    sigA = normExpSig.sig;
                }
                rem = sigA | 0x10000000000000 as u64;
                sigB |= 0x10000000000000 as u64;
                expDiff = expA - expB;
                if expDiff < 1 as i32 as i64 {
                    if expDiff < -(1 as i32) as i64 {
                        return a;
                    }
                    sigB <<= 9 as i32;
                    if expDiff != 0 {
                        rem <<= 8 as i32;
                        q = 0 as i32 as uint32_t;
                    } else {
                        rem <<= 9 as i32;
                        q = (sigB <= rem) as i32 as uint32_t;
                        if q != 0 {
                            rem = (rem as u64).wrapping_sub(sigB) as uint64_t
                                as uint64_t;
                        }
                    }
                    current_block = 13763002826403452995;
                } else {
                    recip32 = (0x7fffffffffffffff as u64)
                        .wrapping_div(
                            (sigB >> 21 as i32) as uint32_t as u64,
                        ) as uint32_t;
                    rem <<= 9 as i32;
                    expDiff -= 30 as i32 as i64;
                    sigB <<= 9 as i32;
                    loop {
                        q64 = ((rem >> 32 as i32) as uint32_t as u64)
                            .wrapping_mul(recip32 as uint_fast64_t);
                        if expDiff < 0 as i32 as i64 {
                            break;
                        }
                        q = (q64
                            .wrapping_add(0x80000000 as u32 as u64)
                            >> 32 as i32) as uint32_t;
                        rem <<= 29 as i32;
                        rem = (rem as u64)
                            .wrapping_sub((q as u64).wrapping_mul(sigB))
                            as uint64_t as uint64_t;
                        if rem & 0x8000000000000000 as u64 != 0 {
                            rem = (rem as u64).wrapping_add(sigB) as uint64_t
                                as uint64_t;
                        }
                        expDiff -= 29 as i32 as i64;
                    }
                    q = (q64 >> 32 as i32) as uint32_t
                        >> (!expDiff & 31 as i32 as i64);
                    rem = (rem << expDiff + 30 as i32 as i64)
                        .wrapping_sub((q as u64).wrapping_mul(sigB));
                    if rem & 0x8000000000000000 as u64 != 0 {
                        altRem = rem.wrapping_add(sigB);
                        current_block = 2191512986525780294;
                    } else {
                        current_block = 13763002826403452995;
                    }
                }
                match current_block {
                    13763002826403452995 => {
                        loop {
                            altRem = rem;
                            q = q.wrapping_add(1);
                            rem = (rem as u64).wrapping_sub(sigB) as uint64_t
                                as uint64_t;
                            if !(rem & 0x8000000000000000 as u64 == 0) {
                                break;
                            }
                        }
                    }
                    _ => {}
                }
                meanRem = rem.wrapping_add(altRem);
                if meanRem & 0x8000000000000000 as u64 != 0
                    || meanRem == 0 && q & 1 as i32 as u32 != 0
                {
                    rem = altRem;
                }
                signRem = signA;
                if rem & 0x8000000000000000 as u64 != 0 {
                    signRem = !signRem;
                    rem = rem.wrapping_neg();
                }
                return softfloat_normRoundPackToF64(signRem, expB, rem);
            }
        }
    }
    match current_block {
        2917149414562135168 => {
            uiZ = softfloat_propagateNaNF64UI(uiA, uiB);
        }
        _ => {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
            uiZ = 0xfff8000000000000 as u64;
        }
    }
    uZ.ui = uiZ;
    return uZ.f;
}
