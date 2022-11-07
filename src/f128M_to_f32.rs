use crate::*;

pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;


pub unsafe fn f128M_to_f32(mut aPtr: *const float128_t) -> float32_t {
    return f128_to_f32(*aPtr);
}
