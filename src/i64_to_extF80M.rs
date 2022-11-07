use crate::*;

pub type __uint16_t = u16;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int64_t = __int64_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;

pub type extFloat80_t = extFloat80M;
pub unsafe fn i64_to_extF80M(mut a: int64_t, mut zPtr: *mut extFloat80_t) {
    *zPtr = i64_to_extF80(a);
}
