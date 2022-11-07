use crate::*;

pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type int_fast8_t = i8;
pub type int_fast32_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast64_t = u64;


pub type extFloat80_t = extFloat80M;
#[inline]
unsafe fn softfloat_countLeadingZeros64(mut a: uint64_t) -> uint_fast8_t {
    return (if a != 0 {
        (a as u64).leading_zeros() as i32
    } else {
        64 as i32
    }) as uint_fast8_t;
}
#[inline]
unsafe fn softfloat_shortShiftLeft128(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut dist: uint_fast8_t,
) -> uint128 {
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    z
        .v64 = a64 << dist as i32
        | a0 >> (-(dist as i32) & 63 as i32);
    z.v0 = a0 << dist as i32;
    return z;
}
pub unsafe fn softfloat_normRoundPackToExtF80(
    mut sign: bool,
    mut exp: int_fast32_t,
    mut sig: uint_fast64_t,
    mut sigExtra: uint_fast64_t,
    mut roundingPrecision: uint_fast8_t,
) -> extFloat80_t {
    let mut shiftDist: int_fast8_t = 0;
    let mut sig128: uint128 = uint128 { v0: 0, v64: 0 };
    if sig == 0 {
        exp -= 64 as i32 as i64;
        sig = sigExtra;
        sigExtra = 0 as i32 as uint_fast64_t;
    }
    shiftDist = softfloat_countLeadingZeros64(sig) as int_fast8_t;
    exp -= shiftDist as i64;
    if shiftDist != 0 {
        sig128 = softfloat_shortShiftLeft128(sig, sigExtra, shiftDist as uint_fast8_t);
        sig = sig128.v64;
        sigExtra = sig128.v0;
    }
    return softfloat_roundPackToExtF80(sign, exp, sig, sigExtra, roundingPrecision);
}
