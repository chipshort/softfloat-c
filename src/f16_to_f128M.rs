use crate::*;

pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;


pub unsafe fn f16_to_f128M(mut a: float16_t, mut zPtr: *mut float128_t) {
    *zPtr = f16_to_f128(a);
}
