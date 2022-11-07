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
pub unsafe fn softfloat_normSubnormalF128Sig(
    mut sig64: uint_fast64_t,
    mut sig0: uint_fast64_t,
) -> exp32_sig128 {
    let mut shiftDist: int_fast8_t = 0;
    let mut z: exp32_sig128 = exp32_sig128 {
        exp: 0,
        sig: uint128 { v0: 0, v64: 0 },
    };
    if sig64 == 0 {
        shiftDist = (softfloat_countLeadingZeros64(sig0) as i32
            - 15 as i32) as int_fast8_t;
        z.exp = (-(63 as i32) - shiftDist as i32) as int_fast32_t;
        if (shiftDist as i32) < 0 as i32 {
            z.sig.v64 = sig0 >> -(shiftDist as i32);
            z.sig.v0 = sig0 << (shiftDist as i32 & 63 as i32);
        } else {
            z.sig.v64 = sig0 << shiftDist as i32;
            z.sig.v0 = 0 as i32 as uint64_t;
        }
    } else {
        shiftDist = (softfloat_countLeadingZeros64(sig64) as i32
            - 15 as i32) as int_fast8_t;
        z.exp = (1 as i32 - shiftDist as i32) as int_fast32_t;
        z.sig = softfloat_shortShiftLeft128(sig64, sig0, shiftDist as uint_fast8_t);
    }
    return z;
}
