use crate::*;

pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int64_t = __int64_t;
pub type uint64_t = __uint64_t;

pub unsafe fn i64_to_f128M(mut a: int64_t, mut zPtr: *mut float128_t) {
    *zPtr = i64_to_f128(a);
}
