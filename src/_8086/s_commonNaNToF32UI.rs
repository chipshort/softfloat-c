use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type uint_fast32_t = u64;

pub unsafe fn softfloat_commonNaNToF32UI(
    mut aPtr: *const commonNaN,
) -> uint_fast32_t {
    return ((*aPtr).sign as uint_fast32_t) << 31 as i32
        | 0x7fc00000 as i32 as u64 | (*aPtr).v64 >> 41 as i32;
}
