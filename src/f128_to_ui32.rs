use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast32_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;


#[derive(Copy, Clone)]
#[repr(C)]
pub union ui128_f128 {
    pub ui: uint128,
    pub f: float128_t,
}
#[inline]
unsafe fn softfloat_shiftRightJam64(
    mut a: uint64_t,
    mut dist: uint_fast32_t,
) -> uint64_t {
    return if dist < 63 as i32 as u64 {
        a >> dist
            | (a << (dist.wrapping_neg() & 63 as i32 as u64)
                != 0 as i32 as u64) as i32 as u64
    } else {
        (a != 0 as i32 as u64) as i32 as u64
    };
}
pub unsafe fn f128_to_ui32(
    mut a: float128_t,
    mut roundingMode: uint_fast8_t,
    mut exact: bool,
) -> uint_fast32_t {
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiA64: uint_fast64_t = 0;
    let mut uiA0: uint_fast64_t = 0;
    let mut sign: bool = false;
    let mut exp: int_fast32_t = 0;
    let mut sig64: uint_fast64_t = 0;
    let mut shiftDist: int_fast32_t = 0;
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    sign = uiA64 >> 63 as i32 != 0;
    exp = (uiA64 >> 48 as i32) as int_fast32_t
        & 0x7fff as i32 as i64;
    sig64 = uiA64 & 0xffffffffffff as u64
        | (uiA0 != 0 as i32 as u64) as i32 as u64;
    if exp != 0 {
        sig64 |= 0x1000000000000 as u64;
    }
    shiftDist = 0x4023 as i32 as i64 - exp;
    if (0 as i32 as i64) < shiftDist {
        sig64 = softfloat_shiftRightJam64(sig64, shiftDist as uint_fast32_t);
    }
    return softfloat_roundToUI32(sign, sig64, roundingMode, exact);
}
