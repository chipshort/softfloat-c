use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast8_t = i8;
pub type int_fast32_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast64_t = u64;

#[inline]
unsafe fn softfloat_countLeadingZeros64(mut a: uint64_t) -> uint_fast8_t {
    return (if a != 0 {
        (a as u64).leading_zeros() as i32
    } else {
        64 as i32
    }) as uint_fast8_t;
}
pub unsafe fn softfloat_normSubnormalExtF80Sig(
    mut sig: uint_fast64_t,
) -> exp32_sig64 {
    let mut shiftDist: int_fast8_t = 0;
    let mut z: exp32_sig64 = exp32_sig64 { exp: 0, sig: 0 };
    shiftDist = softfloat_countLeadingZeros64(sig) as int_fast8_t;
    z.exp = -(shiftDist as i32) as int_fast32_t;
    z.sig = sig << shiftDist as i32;
    return z;
}
