use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast16_t = i64;
pub type int_fast64_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uint64_extra {
    pub extra: uint64_t,
    pub v: uint64_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui64_f64 {
    pub ui: uint64_t,
    pub f: float64_t,
}
pub type C2RustUnnamed = u32;
pub const softfloat_flag_invalid: C2RustUnnamed = 16;
pub const softfloat_flag_infinite: C2RustUnnamed = 8;
pub const softfloat_flag_overflow: C2RustUnnamed = 4;
pub const softfloat_flag_underflow: C2RustUnnamed = 2;
pub const softfloat_flag_inexact: C2RustUnnamed = 1;
#[inline]
unsafe fn softfloat_shiftRightJam64Extra(
    mut a: uint64_t,
    mut extra: uint64_t,
    mut dist: uint_fast32_t,
) -> uint64_extra {
    let mut z: uint64_extra = uint64_extra { extra: 0, v: 0 };
    if dist < 64 as i32 as u64 {
        z.v = a >> dist;
        z.extra = a << (dist.wrapping_neg() & 63 as i32 as u64);
    } else {
        z.v = 0 as i32 as uint64_t;
        z
            .extra = if dist == 64 as i32 as u64 {
            a
        } else {
            (a != 0 as i32 as u64) as i32 as u64
        };
    }
    z.extra
        |= (extra != 0 as i32 as u64) as i32 as u64;
    return z;
}
pub unsafe fn f64_to_i64(
    mut a: float64_t,
    mut roundingMode: uint_fast8_t,
    mut exact: bool,
) -> int_fast64_t {
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: uint_fast64_t = 0;
    let mut sign: bool = false;
    let mut exp: int_fast16_t = 0;
    let mut sig: uint_fast64_t = 0;
    let mut shiftDist: int_fast16_t = 0;
    let mut sigExtra: uint64_extra = uint64_extra { extra: 0, v: 0 };
    uA.f = a;
    uiA = uA.ui;
    sign = uiA >> 63 as i32 != 0;
    exp = (uiA >> 52 as i32) as int_fast16_t
        & 0x7ff as i32 as i64;
    sig = uiA & 0xfffffffffffff as u64;
    if exp != 0 {
        sig |= 0x10000000000000 as u64;
    }
    shiftDist = 0x433 as i32 as i64 - exp;
    if shiftDist <= 0 as i32 as i64 {
        if shiftDist < -(11 as i32) as i64 {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
            return if exp == 0x7ff as i32 as i64
                && uiA & 0xfffffffffffff as u64 != 0
            {
                -(0x7fffffffffffffff as i64) - 1 as i32 as i64
            } else if sign as i32 != 0 {
                -(0x7fffffffffffffff as i64) - 1 as i32 as i64
            } else {
                -(0x7fffffffffffffff as i64) - 1 as i32 as i64
            };
        } else {
            sigExtra.v = sig << -shiftDist;
            sigExtra.extra = 0 as i32 as uint64_t;
        }
    } else {
        sigExtra = softfloat_shiftRightJam64Extra(
            sig,
            0 as i32 as uint64_t,
            shiftDist as uint_fast32_t,
        );
    }
    return softfloat_roundToI64(sign, sigExtra.v, sigExtra.extra, roundingMode, exact);
}
