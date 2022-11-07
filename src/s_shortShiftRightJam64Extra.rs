use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type uint_fast8_t = u8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uint64_extra {
    pub extra: uint64_t,
    pub v: uint64_t,
}
pub unsafe fn softfloat_shortShiftRightJam64Extra(
    mut a: uint64_t,
    mut extra: uint64_t,
    mut dist: uint_fast8_t,
) -> uint64_extra {
    let mut z: uint64_extra = uint64_extra { extra: 0, v: 0 };
    z.v = a >> dist as i32;
    z
        .extra = a << (-(dist as i32) & 63 as i32)
        | (extra != 0 as i32 as u64) as i32 as u64;
    return z;
}
