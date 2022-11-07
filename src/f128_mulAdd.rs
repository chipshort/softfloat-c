use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type uint_fast8_t = u8;
pub type uint_fast64_t = u64;


#[derive(Copy, Clone)]
#[repr(C)]
pub union ui128_f128 {
    pub ui: uint128,
    pub f: float128_t,
}
pub unsafe fn f128_mulAdd(
    mut a: float128_t,
    mut b: float128_t,
    mut c: float128_t,
) -> float128_t {
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiA64: uint_fast64_t = 0;
    let mut uiA0: uint_fast64_t = 0;
    let mut uB: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiB64: uint_fast64_t = 0;
    let mut uiB0: uint_fast64_t = 0;
    let mut uC: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiC64: uint_fast64_t = 0;
    let mut uiC0: uint_fast64_t = 0;
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    uB.f = b;
    uiB64 = uB.ui.v64;
    uiB0 = uB.ui.v0;
    uC.f = c;
    uiC64 = uC.ui.v64;
    uiC0 = uC.ui.v0;
    return softfloat_mulAddF128(
        uiA64,
        uiA0,
        uiB64,
        uiB0,
        uiC64,
        uiC0,
        0 as i32 as uint_fast8_t,
    );
}
