use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;


#[derive(Copy, Clone)]
#[repr(C)]
pub union ui128_f128 {
    pub ui: uint128,
    pub f: float128_t,
}
pub unsafe fn f128_isSignalingNaN(mut a: float128_t) -> bool {
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    uA.f = a;
    return uA.ui.v64 & 0x7fff800000000000 as u64
        == 0x7fff000000000000 as u64
        && (uA.ui.v0 != 0 || uA.ui.v64 & 0x7fffffffffff as u64 != 0);
}
