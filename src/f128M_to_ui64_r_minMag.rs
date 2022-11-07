use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type uint_fast64_t = u64;

pub unsafe fn f128M_to_ui64_r_minMag(
    mut aPtr: *const float128_t,
    mut exact: bool,
) -> uint_fast64_t {
    return f128_to_ui64_r_minMag(*aPtr, exact);
}
