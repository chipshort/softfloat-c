use crate::*;

pub type __uint16_t = u16;
pub type uint16_t = __uint16_t;
pub type uint_fast16_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui16_f16 {
    pub ui: uint16_t,
    pub f: float16_t,
}
pub unsafe fn f16_sub(mut a: float16_t, mut b: float16_t) -> float16_t {
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiA: uint_fast16_t = 0;
    let mut uB: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiB: uint_fast16_t = 0;
    uA.f = a;
    uiA = uA.ui as uint_fast16_t;
    uB.f = b;
    uiB = uB.ui as uint_fast16_t;
    if (uiA ^ uiB) as uint16_t as i32 >> 15 as i32 != 0 {
        return softfloat_addMagsF16(uiA, uiB)
    } else {
        return softfloat_subMagsF16(uiA, uiB)
    };
}
