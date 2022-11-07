use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast32_t = i64;
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
unsafe fn softfloat_shortShiftLeft128(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut dist: uint_fast8_t,
) -> uint128 {
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    z
        .v64 = a64 << dist as i32
        | a0 >> (-(dist as i32) & 63 as i32);
    z.v0 = a0 << dist as i32;
    return z;
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
pub unsafe fn f128_to_ui64(
    mut a: float128_t,
    mut roundingMode: uint_fast8_t,
    mut exact: bool,
) -> uint_fast64_t {
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiA64: uint_fast64_t = 0;
    let mut uiA0: uint_fast64_t = 0;
    let mut sign: bool = false;
    let mut exp: int_fast32_t = 0;
    let mut sig64: uint_fast64_t = 0;
    let mut sig0: uint_fast64_t = 0;
    let mut shiftDist: int_fast32_t = 0;
    let mut sig128: uint128 = uint128 { v0: 0, v64: 0 };
    let mut sigExtra: uint64_extra = uint64_extra { extra: 0, v: 0 };
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    sign = uiA64 >> 63 as i32 != 0;
    exp = (uiA64 >> 48 as i32) as int_fast32_t
        & 0x7fff as i32 as i64;
    sig64 = uiA64 & 0xffffffffffff as u64;
    sig0 = uiA0;
    shiftDist = 0x402f as i32 as i64 - exp;
    if shiftDist <= 0 as i32 as i64 {
        if shiftDist < -(15 as i32) as i64 {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
            return if exp == 0x7fff as i32 as i64 && sig64 | sig0 != 0 {
                0xffffffffffffffff as u64
            } else if sign as i32 != 0 {
                0xffffffffffffffff as u64
            } else {
                0xffffffffffffffff as u64
            };
        }
        sig64 |= 0x1000000000000 as u64;
        if shiftDist != 0 {
            sig128 = softfloat_shortShiftLeft128(
                sig64,
                sig0,
                -shiftDist as uint_fast8_t,
            );
            sig64 = sig128.v64;
            sig0 = sig128.v0;
        }
    } else {
        if exp != 0 {
            sig64 |= 0x1000000000000 as u64;
        }
        sigExtra = softfloat_shiftRightJam64Extra(
            sig64,
            sig0,
            shiftDist as uint_fast32_t,
        );
        sig64 = sigExtra.v;
        sig0 = sigExtra.extra;
    }
    return softfloat_roundToUI64(sign, sig64, sig0, roundingMode, exact);
}
