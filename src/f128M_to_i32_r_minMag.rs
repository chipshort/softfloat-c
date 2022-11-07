use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast32_t = i64;

pub unsafe fn f128M_to_i32_r_minMag(
    mut aPtr: *const float128_t,
    mut exact: bool,
) -> int_fast32_t {
    return f128_to_i32_r_minMag(*aPtr, exact);
}
