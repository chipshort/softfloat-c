use crate::*;

pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;

pub type extFloat80_t = extFloat80M;
pub unsafe fn extF80M_rem(
    mut aPtr: *const extFloat80_t,
    mut bPtr: *const extFloat80_t,
    mut zPtr: *mut extFloat80_t,
) {
    *zPtr = extF80_rem(*aPtr, *bPtr);
}
