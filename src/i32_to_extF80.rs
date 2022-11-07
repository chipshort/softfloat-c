use crate::*;

pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int32_t = __int32_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type int_fast8_t = i8;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;

pub type extFloat80_t = extFloat80M;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s: extFloat80M,
    pub f: extFloat80_t,
}
#[inline]
unsafe fn softfloat_countLeadingZeros32(mut a: uint32_t) -> uint_fast8_t {
    return (if a != 0 { a.leading_zeros() as i32 } else { 32 as i32 })
        as uint_fast8_t;
}
pub unsafe fn i32_to_extF80(mut a: int32_t) -> extFloat80_t {
    let mut uiZ64: uint_fast16_t = 0;
    let mut absA: uint_fast32_t = 0;
    let mut sign: bool = false;
    let mut shiftDist: int_fast8_t = 0;
    let mut uZ: C2RustUnnamed = C2RustUnnamed {
        s: extFloat80M {
            signif: 0,
            signExp: 0,
        },
    };
    uiZ64 = 0 as i32 as uint_fast16_t;
    absA = 0 as i32 as uint_fast32_t;
    if a != 0 {
        sign = a < 0 as i32;
        absA = if sign as i32 != 0 {
            (a as uint_fast32_t).wrapping_neg()
        } else {
            a as uint_fast32_t
        };
        shiftDist = softfloat_countLeadingZeros32(absA as uint32_t) as int_fast8_t;
        uiZ64 = (sign as uint_fast16_t) << 15 as i32
            | (0x401e as i32 - shiftDist as i32) as u64;
        absA <<= shiftDist as i32;
    }
    uZ.s.signExp = uiZ64 as uint16_t;
    uZ.s.signif = absA << 32 as i32;
    return uZ.f;
}
