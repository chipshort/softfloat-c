use crate::*;

pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
pub type uint_fast32_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui32_f32 {
    pub ui: uint32_t,
    pub f: float32_t,
}
pub unsafe fn f32_sub(mut a: float32_t, mut b: float32_t) -> float32_t {
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiA: uint_fast32_t = 0;
    let mut uB: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiB: uint_fast32_t = 0;
    uA.f = a;
    uiA = uA.ui as uint_fast32_t;
    uB.f = b;
    uiB = uB.ui as uint_fast32_t;
    if (uiA ^ uiB) as uint32_t >> 31 as i32 != 0 {
        return softfloat_addMagsF32(uiA, uiB)
    } else {
        return softfloat_subMagsF32(uiA, uiB)
    };
}
