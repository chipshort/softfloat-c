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
pub type C2RustUnnamed = u32;
pub const softfloat_flag_invalid: C2RustUnnamed = 16;
pub const softfloat_flag_infinite: C2RustUnnamed = 8;
pub const softfloat_flag_overflow: C2RustUnnamed = 4;
pub const softfloat_flag_underflow: C2RustUnnamed = 2;
pub const softfloat_flag_inexact: C2RustUnnamed = 1;
#[inline]
unsafe fn softfloat_lt128(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut b64: uint64_t,
    mut b0: uint64_t,
) -> bool {
    return a64 < b64 || a64 == b64 && a0 < b0;
}
pub unsafe fn f128_le(mut a: float128_t, mut b: float128_t) -> bool {
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
    let mut signA: bool = false;
    let mut signB: bool = false;
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    uB.f = b;
    uiB64 = uB.ui.v64;
    uiB0 = uB.ui.v0;
    if !uiA64 & 0x7fff000000000000 as u64 == 0 as i32 as u64
        && (uiA0 != 0 || uiA64 & 0xffffffffffff as u64 != 0)
        || !uiB64 & 0x7fff000000000000 as u64
            == 0 as i32 as u64
            && (uiB0 != 0 || uiB64 & 0xffffffffffff as u64 != 0)
    {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
        return 0 as i32 != 0;
    }
    signA = uiA64 >> 63 as i32 != 0;
    signB = uiB64 >> 63 as i32 != 0;
    return if signA as i32 != signB as i32 {
        (signA as i32 != 0
            || (uiA64 | uiB64) & 0x7fffffffffffffff as u64 | uiA0 | uiB0 == 0)
            as i32
    } else {
        (uiA64 == uiB64 && uiA0 == uiB0
            || signA as i32
                ^ softfloat_lt128(uiA64, uiA0, uiB64, uiB0) as i32 != 0)
            as i32
    } != 0;
}
