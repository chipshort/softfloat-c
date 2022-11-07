use crate::*;

pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type int_fast8_t = i8;
pub type uint_fast8_t = u8;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;


#[derive(Copy, Clone)]
#[repr(C)]
pub union ui128_f128 {
    pub ui: uint128,
    pub f: float128_t,
}
#[inline]
unsafe fn softfloat_countLeadingZeros32(mut a: uint32_t) -> uint_fast8_t {
    return (if a != 0 { a.leading_zeros() as i32 } else { 32 as i32 })
        as uint_fast8_t;
}
pub unsafe fn i32_to_f128(mut a: int32_t) -> float128_t {
    let mut uiZ64: uint_fast64_t = 0;
    let mut sign: bool = false;
    let mut absA: uint_fast32_t = 0;
    let mut shiftDist: int_fast8_t = 0;
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    uiZ64 = 0 as i32 as uint_fast64_t;
    if a != 0 {
        sign = a < 0 as i32;
        absA = if sign as i32 != 0 {
            (a as uint_fast32_t).wrapping_neg()
        } else {
            a as uint_fast32_t
        };
        shiftDist = (softfloat_countLeadingZeros32(absA as uint32_t) as i32
            + 17 as i32) as int_fast8_t;
        uiZ64 = ((sign as uint_fast64_t) << 63 as i32)
            .wrapping_add(
                ((0x402e as i32 - shiftDist as i32) as uint_fast64_t)
                    << 48 as i32,
            )
            .wrapping_add(absA << shiftDist as i32);
    }
    uZ.ui.v64 = uiZ64;
    uZ.ui.v0 = 0 as i32 as uint64_t;
    return uZ.f;
}
