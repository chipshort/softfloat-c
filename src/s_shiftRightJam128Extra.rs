use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type uint_fast8_t = u8;
pub type uint_fast32_t = u64;


pub unsafe fn softfloat_shiftRightJam128Extra(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut extra: uint64_t,
    mut dist: uint_fast32_t,
) -> uint128_extra {
    let mut u8NegDist: uint_fast8_t = 0;
    let mut z: uint128_extra = uint128_extra {
        extra: 0,
        v: uint128 { v0: 0, v64: 0 },
    };
    u8NegDist = dist.wrapping_neg() as uint_fast8_t;
    if dist < 64 as i32 as u64 {
        z.v.v64 = a64 >> dist;
        z.v.v0 = a64 << (u8NegDist as i32 & 63 as i32) | a0 >> dist;
        z.extra = a0 << (u8NegDist as i32 & 63 as i32);
    } else {
        z.v.v64 = 0 as i32 as uint64_t;
        if dist == 64 as i32 as u64 {
            z.v.v0 = a64;
            z.extra = a0;
        } else {
            extra |= a0;
            if dist < 128 as i32 as u64 {
                z.v.v0 = a64 >> (dist & 63 as i32 as u64);
                z.extra = a64 << (u8NegDist as i32 & 63 as i32);
            } else {
                z.v.v0 = 0 as i32 as uint64_t;
                z
                    .extra = if dist == 128 as i32 as u64 {
                    a64
                } else {
                    (a64 != 0 as i32 as u64) as i32
                        as u64
                };
            }
        }
    }
    z.extra
        |= (extra != 0 as i32 as u64) as i32 as u64;
    return z;
}
