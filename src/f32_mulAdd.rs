use crate::*;

pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
pub type uint_fast8_t = u8;
pub type uint_fast32_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui32_f32 {
    pub ui: uint32_t,
    pub f: float32_t,
}
pub unsafe fn f32_mulAdd(
    mut a: float32_t,
    mut b: float32_t,
    mut c: float32_t,
) -> float32_t {
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiA: uint_fast32_t = 0;
    let mut uB: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiB: uint_fast32_t = 0;
    let mut uC: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiC: uint_fast32_t = 0;
    uA.f = a;
    uiA = uA.ui as uint_fast32_t;
    uB.f = b;
    uiB = uB.ui as uint_fast32_t;
    uC.f = c;
    uiC = uC.ui as uint_fast32_t;
    return softfloat_mulAddF32(uiA, uiB, uiC, 0 as i32 as uint_fast8_t);
}
