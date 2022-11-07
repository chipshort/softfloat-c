use crate::*;

pub type __uint16_t = u16;
pub type uint16_t = __uint16_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui16_f16 {
    pub ui: uint16_t,
    pub f: float16_t,
}
pub unsafe fn f16_isSignalingNaN(mut a: float16_t) -> bool {
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    uA.f = a;
    return uA.ui as i32 & 0x7e00 as i32 == 0x7c00 as i32
        && uA.ui as i32 & 0x1ff as i32 != 0;
}
