use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type uint_fast8_t = u8;


pub unsafe fn softfloat_shortShiftRightJam128Extra(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut extra: uint64_t,
    mut dist: uint_fast8_t,
) -> uint128_extra {
    let mut uNegDist: uint_fast8_t = 0;
    let mut z: uint128_extra = uint128_extra {
        extra: 0,
        v: uint128 { v0: 0, v64: 0 },
    };
    uNegDist = -(dist as i32) as uint_fast8_t;
    z.v.v64 = a64 >> dist as i32;
    z
        .v
        .v0 = a64 << (uNegDist as i32 & 63 as i32)
        | a0 >> dist as i32;
    z
        .extra = a0 << (uNegDist as i32 & 63 as i32)
        | (extra != 0 as i32 as u64) as i32 as u64;
    return z;
}
