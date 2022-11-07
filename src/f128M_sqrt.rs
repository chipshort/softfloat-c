use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;

pub unsafe fn f128M_sqrt(
    mut aPtr: *const float128_t,
    mut zPtr: *mut float128_t,
) {
    *zPtr = f128_sqrt(*aPtr);
}
