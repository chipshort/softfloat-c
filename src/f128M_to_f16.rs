use crate::*;

pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;


pub unsafe fn f128M_to_f16(mut aPtr: *const float128_t) -> float16_t {
    return f128_to_f16(*aPtr);
}
