use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui64_f64 {
    pub ui: uint64_t,
    pub f: float64_t,
}
pub unsafe fn f64_isSignalingNaN(mut a: float64_t) -> bool {
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    uA.f = a;
    return uA.ui & 0x7ff8000000000000 as u64
        == 0x7ff0000000000000 as u64
        && uA.ui & 0x7ffffffffffff as u64 != 0;
}
