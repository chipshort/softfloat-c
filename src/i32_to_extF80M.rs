use crate::*;

pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint64_t = u64;
pub type int32_t = __int32_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;

pub type extFloat80_t = extFloat80M;
pub unsafe fn i32_to_extF80M(mut a: int32_t, mut zPtr: *mut extFloat80_t) {
    *zPtr = i32_to_extF80(a);
}
