use crate::*;

pub type __int32_t = i32;
pub type __uint64_t = u64;
pub type int32_t = __int32_t;
pub type uint64_t = __uint64_t;

pub unsafe fn i32_to_f128M(mut a: int32_t, mut zPtr: *mut float128_t) {
    *zPtr = i32_to_f128(a);
}
