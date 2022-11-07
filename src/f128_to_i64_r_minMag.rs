use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast8_t = i8;
pub type int_fast32_t = i64;
pub type int_fast64_t = i64;
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
pub unsafe fn f128_to_i64_r_minMag(
    mut a: float128_t,
    mut exact: bool,
) -> int_fast64_t {
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
    let mut negShiftDist: int_fast8_t = 0;
    let mut absZ: int_fast64_t = 0;
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    sign = uiA64 >> 63 as i32 != 0;
    exp = (uiA64 >> 48 as i32) as int_fast32_t
        & 0x7fff as i32 as i64;
    sig64 = uiA64 & 0xffffffffffff as u64;
    sig0 = uiA0;
    shiftDist = 0x402f as i32 as i64 - exp;
    if shiftDist < 0 as i32 as i64 {
        if shiftDist < -(14 as i32) as i64 {
            if uiA64 == 0xc03e000000000000 as u64
                && sig0 < 0x2000000000000 as u64
            {
                if exact as i32 != 0 && sig0 != 0 {
                    softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                        | softfloat_flag_inexact as i32) as uint_fast8_t;
                }
                return -(0x7fffffffffffffff as i64)
                    - 1 as i32 as i64;
            }
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
            return if exp == 0x7fff as i32 as i64 && sig64 | sig0 != 0 {
                -(0x7fffffffffffffff as i64) - 1 as i32 as i64
            } else if sign as i32 != 0 {
                -(0x7fffffffffffffff as i64) - 1 as i32 as i64
            } else {
                -(0x7fffffffffffffff as i64) - 1 as i32 as i64
            };
        }
        sig64 |= 0x1000000000000 as u64;
        negShiftDist = -shiftDist as int_fast8_t;
        absZ = (sig64 << negShiftDist as i32
            | sig0 >> (shiftDist & 63 as i32 as i64)) as int_fast64_t;
        if exact as i32 != 0 && sig0 << negShiftDist as i32 != 0 {
            softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                | softfloat_flag_inexact as i32) as uint_fast8_t;
        }
    } else {
        if 49 as i32 as i64 <= shiftDist {
            if exact as i32 != 0 && exp as u64 | sig64 | sig0 != 0 {
                softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                    | softfloat_flag_inexact as i32) as uint_fast8_t;
            }
            return 0 as i32 as int_fast64_t;
        }
        sig64 |= 0x1000000000000 as u64;
        absZ = (sig64 >> shiftDist) as int_fast64_t;
        if exact as i32 != 0
            && (sig0 != 0 || (absZ << shiftDist) as u64 != sig64)
        {
            softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                | softfloat_flag_inexact as i32) as uint_fast8_t;
        }
    }
    return if sign as i32 != 0 { -absZ } else { absZ };
}
