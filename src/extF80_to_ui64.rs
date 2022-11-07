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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uint64_extra {
    pub extra: uint64_t,
    pub v: uint64_t,
}

pub type extFloat80_t = extFloat80M;
pub type C2RustUnnamed = u32;
pub const softfloat_flag_invalid: C2RustUnnamed = 16;
pub const softfloat_flag_infinite: C2RustUnnamed = 8;
pub const softfloat_flag_overflow: C2RustUnnamed = 4;
pub const softfloat_flag_underflow: C2RustUnnamed = 2;
pub const softfloat_flag_inexact: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub s: extFloat80M,
    pub f: extFloat80_t,
}
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
pub unsafe fn extF80_to_ui64(
    mut a: extFloat80_t,
    mut roundingMode: uint_fast8_t,
    mut exact: bool,
) -> uint_fast64_t {
    let mut uA: C2RustUnnamed_0 = C2RustUnnamed_0 {
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
    let mut sigExtra: uint_fast64_t = 0;
    let mut sig64Extra: uint64_extra = uint64_extra { extra: 0, v: 0 };
    uA.f = a;
    uiA64 = uA.s.signExp as uint_fast16_t;
    sign = uiA64 as uint16_t as i32 >> 15 as i32 != 0;
    exp = (uiA64 & 0x7fff as i32 as u64) as int_fast32_t;
    sig = uA.s.signif;
    shiftDist = 0x403e as i32 as i64 - exp;
    if shiftDist < 0 as i32 as i64 {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
        return if exp == 0x7fff as i32 as i64
            && sig & 0x7fffffffffffffff as u64 != 0
        {
            0xffffffffffffffff as u64
        } else if sign as i32 != 0 {
            0xffffffffffffffff as u64
        } else {
            0xffffffffffffffff as u64
        };
    }
    sigExtra = 0 as i32 as uint_fast64_t;
    if shiftDist != 0 {
        sig64Extra = softfloat_shiftRightJam64Extra(
            sig,
            0 as i32 as uint64_t,
            shiftDist as uint_fast32_t,
        );
        sig = sig64Extra.v;
        sigExtra = sig64Extra.extra;
    }
    return softfloat_roundToUI64(sign, sig, sigExtra, roundingMode, exact);
}
