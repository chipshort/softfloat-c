use crate::*;

pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui32_f32 {
    pub ui: uint32_t,
    pub f: float32_t,
}
pub unsafe fn f32_isSignalingNaN(mut a: float32_t) -> bool {
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    uA.f = a;
    return uA.ui & 0x7fc00000 as i32 as u32
        == 0x7f800000 as i32 as u32
        && uA.ui & 0x3fffff as i32 as u32 != 0;
}
