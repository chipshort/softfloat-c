use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type uint_fast64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui64_f64 {
    pub ui: uint64_t,
    pub f: float64_t,
}
pub unsafe fn f64_add(mut a: float64_t, mut b: float64_t) -> float64_t {
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: uint_fast64_t = 0;
    let mut signA: bool = false;
    let mut uB: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiB: uint_fast64_t = 0;
    let mut signB: bool = false;
    uA.f = a;
    uiA = uA.ui;
    signA = uiA >> 63 as i32 != 0;
    uB.f = b;
    uiB = uB.ui;
    signB = uiB >> 63 as i32 != 0;
    if signA as i32 == signB as i32 {
        return softfloat_addMagsF64(uiA, uiB, signA)
    } else {
        return softfloat_subMagsF64(uiA, uiB, signA)
    };
}
