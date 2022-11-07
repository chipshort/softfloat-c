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
pub struct uint64_extra {
    pub extra: uint64_t,
    pub v: uint64_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui32_f32 {
    pub ui: uint32_t,
    pub f: float32_t,
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
pub unsafe fn f32_to_ui64(
    mut a: float32_t,
    mut roundingMode: uint_fast8_t,
    mut exact: bool,
) -> uint_fast64_t {
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiA: uint_fast32_t = 0;
    let mut sign: bool = false;
    let mut exp: int_fast16_t = 0;
    let mut sig: uint_fast32_t = 0;
    let mut shiftDist: int_fast16_t = 0;
    let mut sig64: uint_fast64_t = 0;
    let mut extra: uint_fast64_t = 0;
    let mut sig64Extra: uint64_extra = uint64_extra { extra: 0, v: 0 };
    uA.f = a;
    uiA = uA.ui as uint_fast32_t;
    sign = uiA as uint32_t >> 31 as i32 != 0;
    exp = (uiA >> 23 as i32) as int_fast16_t
        & 0xff as i32 as i64;
    sig = uiA & 0x7fffff as i32 as u64;
    shiftDist = 0xbe as i32 as i64 - exp;
    if shiftDist < 0 as i32 as i64 {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
        return if exp == 0xff as i32 as i64 && sig != 0 {
            0xffffffffffffffff as u64
        } else if sign as i32 != 0 {
            0xffffffffffffffff as u64
        } else {
            0xffffffffffffffff as u64
        };
    }
    if exp != 0 {
        sig |= 0x800000 as i32 as u64;
    }
    sig64 = sig << 40 as i32;
    extra = 0 as i32 as uint_fast64_t;
    if shiftDist != 0 {
        sig64Extra = softfloat_shiftRightJam64Extra(
            sig64,
            0 as i32 as uint64_t,
            shiftDist as uint_fast32_t,
        );
        sig64 = sig64Extra.v;
        extra = sig64Extra.extra;
    }
    return softfloat_roundToUI64(sign, sig64, extra, roundingMode, exact);
}
