use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;

pub unsafe fn ui64_to_f128M(mut a: uint64_t, mut zPtr: *mut float128_t) {
    *zPtr = ui64_to_f128(a);
}
