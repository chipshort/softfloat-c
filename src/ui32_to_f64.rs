use crate::*;

pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type int_fast8_t = i8;
pub type uint_fast8_t = u8;
pub type uint_fast64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui64_f64 {
    pub ui: uint64_t,
    pub f: float64_t,
}
#[inline]
unsafe fn softfloat_countLeadingZeros32(mut a: uint32_t) -> uint_fast8_t {
    return (if a != 0 { a.leading_zeros() as i32 } else { 32 as i32 })
        as uint_fast8_t;
}
pub unsafe fn ui32_to_f64(mut a: uint32_t) -> float64_t {
    let mut uiZ: uint_fast64_t = 0;
    let mut shiftDist: int_fast8_t = 0;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    if a == 0 {
        uiZ = 0 as i32 as uint_fast64_t;
    } else {
        shiftDist = (softfloat_countLeadingZeros32(a) as i32 + 21 as i32)
            as int_fast8_t;
        uiZ = ((0 as i32 as uint_fast64_t) << 63 as i32)
            .wrapping_add(
                ((0x432 as i32 - shiftDist as i32) as uint_fast64_t)
                    << 52 as i32,
            )
            .wrapping_add((a as uint_fast64_t) << shiftDist as i32);
    }
    uZ.ui = uiZ;
    return uZ.f;
}
