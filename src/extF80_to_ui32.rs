use crate::*;

pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type int_fast32_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;

pub type extFloat80_t = extFloat80M;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s: extFloat80M,
    pub f: extFloat80_t,
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
pub unsafe fn extF80_to_ui32(
    mut a: extFloat80_t,
    mut roundingMode: uint_fast8_t,
    mut exact: bool,
) -> uint_fast32_t {
    let mut uA: C2RustUnnamed = C2RustUnnamed {
        s: extFloat80_t {
            signif: 0,
            signExp: 0,
        },
    };
    let mut uiA64: uint_fast16_t = 0;
    let mut sign: bool = false;
    let mut exp: int_fast32_t = 0;
    let mut sig: uint_fast64_t = 0;
    let mut shiftDist: int_fast32_t = 0;
    uA.f = a;
    uiA64 = uA.s.signExp as uint_fast16_t;
    sign = uiA64 as uint16_t as i32 >> 15 as i32 != 0;
    exp = (uiA64 & 0x7fff as i32 as u64) as int_fast32_t;
    sig = uA.s.signif;
    shiftDist = 0x4032 as i32 as i64 - exp;
    if shiftDist <= 0 as i32 as i64 {
        shiftDist = 1 as i32 as int_fast32_t;
    }
    sig = softfloat_shiftRightJam64(sig, shiftDist as uint_fast32_t);
    return softfloat_roundToUI32(sign, sig, roundingMode, exact);
}
