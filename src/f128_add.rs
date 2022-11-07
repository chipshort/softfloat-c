use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type uint_fast64_t = u64;


#[derive(Copy, Clone)]
#[repr(C)]
pub union ui128_f128 {
    pub ui: uint128,
    pub f: float128_t,
}
pub unsafe fn f128_add(mut a: float128_t, mut b: float128_t) -> float128_t {
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiA64: uint_fast64_t = 0;
    let mut uiA0: uint_fast64_t = 0;
    let mut signA: bool = false;
    let mut uB: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiB64: uint_fast64_t = 0;
    let mut uiB0: uint_fast64_t = 0;
    let mut signB: bool = false;
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    signA = uiA64 >> 63 as i32 != 0;
    uB.f = b;
    uiB64 = uB.ui.v64;
    uiB0 = uB.ui.v0;
    signB = uiB64 >> 63 as i32 != 0;
    if signA as i32 == signB as i32 {
        return softfloat_addMagsF128(uiA64, uiA0, uiB64, uiB0, signA)
    } else {
        return softfloat_subMagsF128(uiA64, uiA0, uiB64, uiB0, signA)
    };
}
