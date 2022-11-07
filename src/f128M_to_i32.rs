use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast32_t = i64;
pub type uint_fast8_t = u8;

pub unsafe fn f128M_to_i32(
    mut aPtr: *const float128_t,
    mut roundingMode: uint_fast8_t,
    mut exact: bool,
) -> int_fast32_t {
    return f128_to_i32(*aPtr, roundingMode, exact);
}
