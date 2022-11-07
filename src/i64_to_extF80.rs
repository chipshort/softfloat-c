use crate::*;

pub type __uint16_t = u16;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int64_t = __int64_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type int_fast8_t = i8;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast64_t = u64;

pub type extFloat80_t = extFloat80M;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s: extFloat80M,
    pub f: extFloat80_t,
}
#[inline]
unsafe fn softfloat_countLeadingZeros64(mut a: uint64_t) -> uint_fast8_t {
    return (if a != 0 {
        (a as u64).leading_zeros() as i32
    } else {
        64 as i32
    }) as uint_fast8_t;
}
pub unsafe fn i64_to_extF80(mut a: int64_t) -> extFloat80_t {
    let mut uiZ64: uint_fast16_t = 0;
    let mut absA: uint_fast64_t = 0;
    let mut sign: bool = false;
    let mut shiftDist: int_fast8_t = 0;
    let mut uZ: C2RustUnnamed = C2RustUnnamed {
        s: extFloat80M {
            signif: 0,
            signExp: 0,
        },
    };
    uiZ64 = 0 as i32 as uint_fast16_t;
    absA = 0 as i32 as uint_fast64_t;
    if a != 0 {
        sign = a < 0 as i32 as i64;
        absA = if sign as i32 != 0 {
            (a as uint_fast64_t).wrapping_neg()
        } else {
            a as uint_fast64_t
        };
        shiftDist = softfloat_countLeadingZeros64(absA) as int_fast8_t;
        uiZ64 = (sign as uint_fast16_t) << 15 as i32
            | (0x403e as i32 - shiftDist as i32) as u64;
        absA <<= shiftDist as i32;
    }
    uZ.s.signExp = uiZ64 as uint16_t;
    uZ.s.signif = absA;
    return uZ.f;
}
