use crate::*;

pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;

pub unsafe fn ui32_to_f128M(mut a: uint32_t, mut zPtr: *mut float128_t) {
    *zPtr = ui32_to_f128(a);
}
