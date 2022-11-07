use crate::*;

pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type int_fast32_t = i64;

pub type extFloat80_t = extFloat80M;
pub unsafe fn extF80M_to_i32_r_minMag(
    mut aPtr: *const extFloat80_t,
    mut exact: bool,
) -> int_fast32_t {
    return extF80_to_i32_r_minMag(*aPtr, exact);
}
