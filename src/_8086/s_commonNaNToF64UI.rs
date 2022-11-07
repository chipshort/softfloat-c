use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type uint_fast64_t = u64;

pub unsafe fn softfloat_commonNaNToF64UI(
    mut aPtr: *const commonNaN,
) -> uint_fast64_t {
    return ((*aPtr).sign as uint_fast64_t) << 63 as i32
        | 0x7ff8000000000000 as u64 | (*aPtr).v64 >> 12 as i32;
}
