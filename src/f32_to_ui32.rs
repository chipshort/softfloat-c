use crate::*;

pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type int_fast16_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui32_f32 {
    pub ui: uint32_t,
    pub f: float32_t,
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
pub unsafe fn f32_to_ui32(
    mut a: float32_t,
    mut roundingMode: uint_fast8_t,
    mut exact: bool,
) -> uint_fast32_t {
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiA: uint_fast32_t = 0;
    let mut sign: bool = false;
    let mut exp: int_fast16_t = 0;
    let mut sig: uint_fast32_t = 0;
    let mut sig64: uint_fast64_t = 0;
    let mut shiftDist: int_fast16_t = 0;
    uA.f = a;
    uiA = uA.ui as uint_fast32_t;
    sign = uiA as uint32_t >> 31 as i32 != 0;
    exp = (uiA >> 23 as i32) as int_fast16_t
        & 0xff as i32 as i64;
    sig = uiA & 0x7fffff as i32 as u64;
    if exp != 0 {
        sig |= 0x800000 as i32 as u64;
    }
    sig64 = sig << 32 as i32;
    shiftDist = 0xaa as i32 as i64 - exp;
    if (0 as i32 as i64) < shiftDist {
        sig64 = softfloat_shiftRightJam64(sig64, shiftDist as uint_fast32_t);
    }
    return softfloat_roundToUI32(sign, sig64, roundingMode, exact);
}
