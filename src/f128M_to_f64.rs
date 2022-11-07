use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;


pub unsafe fn f128M_to_f64(mut aPtr: *const float128_t) -> float64_t {
    return f128_to_f64(*aPtr);
}
