use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type uint_fast16_t = u64;

pub unsafe fn softfloat_commonNaNToF16UI(
    mut aPtr: *const commonNaN,
) -> uint_fast16_t {
    return ((*aPtr).sign as uint_fast16_t) << 15 as i32
        | 0x7e00 as i32 as u64 | (*aPtr).v64 >> 54 as i32;
}
