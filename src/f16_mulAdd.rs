use crate::*;

pub type __uint16_t = u16;
pub type uint16_t = __uint16_t;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui16_f16 {
    pub ui: uint16_t,
    pub f: float16_t,
}
pub unsafe fn f16_mulAdd(
    mut a: float16_t,
    mut b: float16_t,
    mut c: float16_t,
) -> float16_t {
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiA: uint_fast16_t = 0;
    let mut uB: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiB: uint_fast16_t = 0;
    let mut uC: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiC: uint_fast16_t = 0;
    uA.f = a;
    uiA = uA.ui as uint_fast16_t;
    uB.f = b;
    uiB = uB.ui as uint_fast16_t;
    uC.f = c;
    uiC = uC.ui as uint_fast16_t;
    return softfloat_mulAddF16(uiA, uiB, uiC, 0 as i32 as uint_fast8_t);
}
