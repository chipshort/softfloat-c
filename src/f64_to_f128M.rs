use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;


pub unsafe fn f64_to_f128M(mut a: float64_t, mut zPtr: *mut float128_t) {
    *zPtr = f64_to_f128(a);
}
