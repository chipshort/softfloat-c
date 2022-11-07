use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast16_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui64_f64 {
    pub ui: uint64_t,
    pub f: float64_t,
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
pub unsafe fn f64_to_ui32(
    mut a: float64_t,
    mut roundingMode: uint_fast8_t,
    mut exact: bool,
) -> uint_fast32_t {
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: uint_fast64_t = 0;
    let mut sign: bool = false;
    let mut exp: int_fast16_t = 0;
    let mut sig: uint_fast64_t = 0;
    let mut shiftDist: int_fast16_t = 0;
    uA.f = a;
    uiA = uA.ui;
    sign = uiA >> 63 as i32 != 0;
    exp = (uiA >> 52 as i32) as int_fast16_t
        & 0x7ff as i32 as i64;
    sig = uiA & 0xfffffffffffff as u64;
    if exp != 0 {
        sig |= 0x10000000000000 as u64;
    }
    shiftDist = 0x427 as i32 as i64 - exp;
    if (0 as i32 as i64) < shiftDist {
        sig = softfloat_shiftRightJam64(sig, shiftDist as uint_fast32_t);
    }
    return softfloat_roundToUI32(sign, sig, roundingMode, exact);
}
