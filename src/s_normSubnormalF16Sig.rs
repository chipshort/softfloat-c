use crate::*;

pub type __uint16_t = u16;
pub type uint16_t = __uint16_t;
pub type int_fast8_t = i8;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;

#[inline]
unsafe fn softfloat_countLeadingZeros16(mut a: uint16_t) -> uint_fast8_t {
    return (if a as i32 != 0 {
        (a as u32).leading_zeros() as i32 - 16 as i32
    } else {
        16 as i32
    }) as uint_fast8_t;
}
pub unsafe fn softfloat_normSubnormalF16Sig(
    mut sig: uint_fast16_t,
) -> exp8_sig16 {
    let mut shiftDist: int_fast8_t = 0;
    let mut z: exp8_sig16 = exp8_sig16 { exp: 0, sig: 0 };
    shiftDist = (softfloat_countLeadingZeros16(sig as uint16_t) as i32
        - 5 as i32) as int_fast8_t;
    z.exp = (1 as i32 - shiftDist as i32) as int_fast8_t;
    z.sig = sig << shiftDist as i32;
    return z;
}
