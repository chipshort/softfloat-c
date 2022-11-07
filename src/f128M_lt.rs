use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;

pub unsafe fn f128M_lt(
    mut aPtr: *const float128_t,
    mut bPtr: *const float128_t,
) -> bool {
    return f128_lt(*aPtr, *bPtr);
}
