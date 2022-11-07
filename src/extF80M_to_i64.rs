use crate::*;

pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type int_fast64_t = i64;
pub type uint_fast8_t = u8;

pub type extFloat80_t = extFloat80M;
pub unsafe fn extF80M_to_i64(
    mut aPtr: *const extFloat80_t,
    mut roundingMode: uint_fast8_t,
    mut exact: bool,
) -> int_fast64_t {
    return extF80_to_i64(*aPtr, roundingMode, exact);
}
