use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type uint_fast8_t = u8;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;

pub unsafe fn softfloat_shiftRightJam128(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut dist: uint_fast32_t,
) -> uint128 {
    let mut u8NegDist: uint_fast8_t = 0;
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    if dist < 64 as i32 as u64 {
        u8NegDist = dist.wrapping_neg() as uint_fast8_t;
        z.v64 = a64 >> dist;
        z
            .v0 = a64 << (u8NegDist as i32 & 63 as i32) | a0 >> dist
            | (a0 << (u8NegDist as i32 & 63 as i32)
                != 0 as i32 as u64) as i32 as u64;
    } else {
        z.v64 = 0 as i32 as uint64_t;
        z
            .v0 = if dist < 127 as i32 as u64 {
            a64 >> (dist & 63 as i32 as u64)
                | (a64
                    & ((1 as i32 as uint_fast64_t)
                        << (dist & 63 as i32 as u64))
                        .wrapping_sub(1 as i32 as u64) | a0
                    != 0 as i32 as u64) as i32 as u64
        } else {
            (a64 | a0 != 0 as i32 as u64) as i32
                as u64
        };
    }
    return z;
}
