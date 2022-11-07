use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast64_t = i64;
pub type uint_fast8_t = u8;

pub unsafe fn f128M_to_i64(
    mut aPtr: *const float128_t,
    mut roundingMode: uint_fast8_t,
    mut exact: bool,
) -> int_fast64_t {
    return f128_to_i64(*aPtr, roundingMode, exact);
}
