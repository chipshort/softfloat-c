use crate::*;

pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
pub type uint_fast16_t = u64;
pub unsafe fn softfloat_shiftRightJam32(
    mut a: uint32_t,
    mut dist: uint_fast16_t,
) -> uint32_t {
    return if dist < 31 as i32 as u64 {
        a >> dist
            | (a << (dist.wrapping_neg() & 31 as i32 as u64)
                != 0 as i32 as u32) as i32 as u32
    } else {
        (a != 0 as i32 as u32) as i32 as u32
    };
}
