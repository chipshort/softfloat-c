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
pub unsafe fn f32_rem(mut a: float32_t, mut b: float32_t) -> float32_t {
    let mut current_block: u64;
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiA: uint_fast32_t = 0;
    let mut signA: bool = false;
    let mut expA: int_fast16_t = 0;
    let mut sigA: uint_fast32_t = 0;
    let mut uB: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiB: uint_fast32_t = 0;
    let mut expB: int_fast16_t = 0;
    let mut sigB: uint_fast32_t = 0;
    let mut normExpSig: exp16_sig32 = exp16_sig32 { exp: 0, sig: 0 };
    let mut rem: uint32_t = 0;
    let mut expDiff: int_fast16_t = 0;
    let mut q: uint32_t = 0;
    let mut recip32: uint32_t = 0;
    let mut altRem: uint32_t = 0;
    let mut meanRem: uint32_t = 0;
    let mut signRem: bool = false;
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
    expB = (uiB >> 23 as i32) as int_fast16_t
        & 0xff as i32 as i64;
    sigB = uiB & 0x7fffff as i32 as u64;
    if expA == 0xff as i32 as i64 {
        if sigA != 0 || expB == 0xff as i32 as i64 && sigB != 0 {
            current_block = 18057179269304210558;
        } else {
            current_block = 5399300132870637682;
        }
    } else if expB == 0xff as i32 as i64 {
        if sigB != 0 {
            current_block = 18057179269304210558;
        } else {
            return a
        }
    } else {
        if expB == 0 {
            if sigB == 0 {
                current_block = 5399300132870637682;
            } else {
                normExpSig = softfloat_normSubnormalF32Sig(sigB);
                expB = normExpSig.exp;
                sigB = normExpSig.sig;
                current_block = 12147880666119273379;
            }
        } else {
            current_block = 12147880666119273379;
        }
        match current_block {
            5399300132870637682 => {}
            _ => {
                if expA == 0 {
                    if sigA == 0 {
                        return a;
                    }
                    normExpSig = softfloat_normSubnormalF32Sig(sigA);
                    expA = normExpSig.exp;
                    sigA = normExpSig.sig;
                }
                rem = (sigA | 0x800000 as i32 as u64) as uint32_t;
                sigB |= 0x800000 as i32 as u64;
                expDiff = expA - expB;
                if expDiff < 1 as i32 as i64 {
                    if expDiff < -(1 as i32) as i64 {
                        return a;
                    }
                    sigB <<= 6 as i32;
                    if expDiff != 0 {
                        rem <<= 5 as i32;
                        q = 0 as i32 as uint32_t;
                    } else {
                        rem <<= 6 as i32;
                        q = (sigB <= rem as u64) as i32 as uint32_t;
                        if q != 0 {
                            rem = (rem as u64).wrapping_sub(sigB) as uint32_t
                                as uint32_t;
                        }
                    }
                } else {
                    recip32 = (0x7fffffffffffffff as u64)
                        .wrapping_div(
                            (sigB << 8 as i32) as uint32_t as u64,
                        ) as uint32_t;
                    rem <<= 7 as i32;
                    expDiff -= 31 as i32 as i64;
                    sigB <<= 6 as i32;
                    loop {
                        q = ((rem as u64)
                            .wrapping_mul(recip32 as uint_fast64_t) >> 32 as i32)
                            as uint32_t;
                        if expDiff < 0 as i32 as i64 {
                            break;
                        }
                        rem = q.wrapping_mul(sigB as uint32_t).wrapping_neg();
                        expDiff -= 29 as i32 as i64;
                    }
                    q >>= !expDiff & 31 as i32 as i64;
                    rem = (rem << expDiff + 30 as i32 as i64)
                        .wrapping_sub(q.wrapping_mul(sigB as uint32_t));
                }
                loop {
                    altRem = rem;
                    q = q.wrapping_add(1);
                    rem = (rem as u64).wrapping_sub(sigB) as uint32_t
                        as uint32_t;
                    if !(rem & 0x80000000 as u32 == 0) {
                        break;
                    }
                }
                meanRem = rem.wrapping_add(altRem);
                if meanRem & 0x80000000 as u32 != 0
                    || meanRem == 0 && q & 1 as i32 as u32 != 0
                {
                    rem = altRem;
                }
                signRem = signA;
                if 0x80000000 as u32 <= rem {
                    signRem = !signRem;
                    rem = rem.wrapping_neg();
                }
                return softfloat_normRoundPackToF32(signRem, expB, rem as uint_fast32_t);
            }
        }
    }
    match current_block {
        18057179269304210558 => {
            uiZ = softfloat_propagateNaNF32UI(uiA, uiB);
        }
        _ => {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
            uiZ = 0xffc00000 as u32 as uint_fast32_t;
        }
    }
    uZ.ui = uiZ as uint32_t;
    return uZ.f;
}
