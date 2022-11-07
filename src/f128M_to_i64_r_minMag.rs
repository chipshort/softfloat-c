use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast64_t = i64;

pub unsafe fn f128M_to_i64_r_minMag(
    mut aPtr: *const float128_t,
    mut exact: bool,
) -> int_fast64_t {
    return f128_to_i64_r_minMag(*aPtr, exact);
}
