use crate::*;

pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type int_fast8_t = i8;
pub type int_fast16_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;

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
pub unsafe fn f16_rem(mut a: float16_t, mut b: float16_t) -> float16_t {
    let mut current_block: u64;
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiA: uint_fast16_t = 0;
    let mut signA: bool = false;
    let mut expA: int_fast8_t = 0;
    let mut sigA: uint_fast16_t = 0;
    let mut uB: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiB: uint_fast16_t = 0;
    let mut expB: int_fast8_t = 0;
    let mut sigB: uint_fast16_t = 0;
    let mut normExpSig: exp8_sig16 = exp8_sig16 { exp: 0, sig: 0 };
    let mut rem: uint16_t = 0;
    let mut expDiff: int_fast8_t = 0;
    let mut q: uint_fast16_t = 0;
    let mut recip32: uint32_t = 0;
    let mut q32: uint32_t = 0;
    let mut altRem: uint16_t = 0;
    let mut meanRem: uint16_t = 0;
    let mut signRem: bool = false;
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
    expB = ((uiB >> 10 as i32) as int_fast8_t as i32
        & 0x1f as i32) as int_fast8_t;
    sigB = uiB & 0x3ff as i32 as u64;
    if expA as i32 == 0x1f as i32 {
        if sigA != 0 || expB as i32 == 0x1f as i32 && sigB != 0 {
            current_block = 4183820269179547385;
        } else {
            current_block = 12992985024832137741;
        }
    } else if expB as i32 == 0x1f as i32 {
        if sigB != 0 {
            current_block = 4183820269179547385;
        } else {
            return a
        }
    } else {
        if expB == 0 {
            if sigB == 0 {
                current_block = 12992985024832137741;
            } else {
                normExpSig = softfloat_normSubnormalF16Sig(sigB);
                expB = normExpSig.exp;
                sigB = normExpSig.sig;
                current_block = 15089075282327824602;
            }
        } else {
            current_block = 15089075282327824602;
        }
        match current_block {
            12992985024832137741 => {}
            _ => {
                if expA == 0 {
                    if sigA == 0 {
                        return a;
                    }
                    normExpSig = softfloat_normSubnormalF16Sig(sigA);
                    expA = normExpSig.exp;
                    sigA = normExpSig.sig;
                }
                rem = (sigA | 0x400 as i32 as u64) as uint16_t;
                sigB |= 0x400 as i32 as u64;
                expDiff = (expA as i32 - expB as i32) as int_fast8_t;
                if (expDiff as i32) < 1 as i32 {
                    if (expDiff as i32) < -(1 as i32) {
                        return a;
                    }
                    sigB <<= 3 as i32;
                    if expDiff != 0 {
                        rem = ((rem as i32) << 2 as i32) as uint16_t;
                        q = 0 as i32 as uint_fast16_t;
                    } else {
                        rem = ((rem as i32) << 3 as i32) as uint16_t;
                        q = (sigB <= rem as u64) as i32
                            as uint_fast16_t;
                        if q != 0 {
                            rem = (rem as u64).wrapping_sub(sigB) as uint16_t
                                as uint16_t;
                        }
                    }
                } else {
                    recip32 = (0x7fffffffffffffff as u64)
                        .wrapping_div(
                            (sigB << 21 as i32) as uint32_t as u64,
                        ) as uint32_t;
                    rem = ((rem as i32) << 4 as i32) as uint16_t;
                    expDiff = (expDiff as i32 - 31 as i32)
                        as int_fast8_t;
                    sigB <<= 3 as i32;
                    loop {
                        q32 = ((rem as u64)
                            .wrapping_mul(recip32 as uint_fast64_t) >> 16 as i32)
                            as uint32_t;
                        if (expDiff as i32) < 0 as i32 {
                            break;
                        }
                        rem = (q32 as uint_fast16_t).wrapping_mul(sigB).wrapping_neg()
                            as uint16_t;
                        expDiff = (expDiff as i32 - 29 as i32)
                            as int_fast8_t;
                    }
                    q32 >>= !(expDiff as i32) & 31 as i32;
                    q = q32 as uint_fast16_t;
                    rem = (((rem as i32)
                        << expDiff as i32 + 30 as i32) as u64)
                        .wrapping_sub(q.wrapping_mul(sigB)) as uint16_t;
                }
                loop {
                    altRem = rem;
                    q = q.wrapping_add(1);
                    rem = (rem as u64).wrapping_sub(sigB) as uint16_t
                        as uint16_t;
                    if !(rem as i32 & 0x8000 as i32 == 0) {
                        break;
                    }
                }
                meanRem = (rem as i32 + altRem as i32) as uint16_t;
                if meanRem as i32 & 0x8000 as i32 != 0
                    || meanRem == 0 && q & 1 as i32 as u64 != 0
                {
                    rem = altRem;
                }
                signRem = signA;
                if 0x8000 as i32 <= rem as i32 {
                    signRem = !signRem;
                    rem = -(rem as i32) as uint16_t;
                }
                return softfloat_normRoundPackToF16(
                    signRem,
                    expB as int_fast16_t,
                    rem as uint_fast16_t,
                );
            }
        }
    }
    match current_block {
        4183820269179547385 => {
            uiZ = softfloat_propagateNaNF16UI(uiA, uiB);
        }
        _ => {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
            uiZ = 0xfe00 as i32 as uint_fast16_t;
        }
    }
    uZ.ui = uiZ as uint16_t;
    return uZ.f;
}
