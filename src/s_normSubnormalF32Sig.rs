use crate::*;

pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
pub type int_fast8_t = i8;
pub type int_fast16_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast32_t = u64;

#[inline]
unsafe fn softfloat_countLeadingZeros32(mut a: uint32_t) -> uint_fast8_t {
    return (if a != 0 { a.leading_zeros() as i32 } else { 32 as i32 })
        as uint_fast8_t;
}
pub unsafe fn softfloat_normSubnormalF32Sig(
    mut sig: uint_fast32_t,
) -> exp16_sig32 {
    let mut shiftDist: int_fast8_t = 0;
    let mut z: exp16_sig32 = exp16_sig32 { exp: 0, sig: 0 };
    shiftDist = (softfloat_countLeadingZeros32(sig as uint32_t) as i32
        - 8 as i32) as int_fast8_t;
    z.exp = (1 as i32 - shiftDist as i32) as int_fast16_t;
    z.sig = sig << shiftDist as i32;
    return z;
}
