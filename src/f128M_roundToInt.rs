use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type uint_fast8_t = u8;

pub unsafe fn f128M_roundToInt(
    mut aPtr: *const float128_t,
    mut roundingMode: uint_fast8_t,
    mut exact: bool,
    mut zPtr: *mut float128_t,
) {
    *zPtr = f128_roundToInt(*aPtr, roundingMode, exact);
}
