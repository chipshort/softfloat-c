use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast16_t = i64;
pub type int_fast32_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;

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
pub unsafe fn f64_to_i32_r_minMag(
    mut a: float64_t,
    mut exact: bool,
) -> int_fast32_t {
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: uint_fast64_t = 0;
    let mut exp: int_fast16_t = 0;
    let mut sig: uint_fast64_t = 0;
    let mut shiftDist: int_fast16_t = 0;
    let mut sign: bool = false;
    let mut absZ: int_fast32_t = 0;
    uA.f = a;
    uiA = uA.ui;
    exp = (uiA >> 52 as i32) as int_fast16_t
        & 0x7ff as i32 as i64;
    sig = uiA & 0xfffffffffffff as u64;
    shiftDist = 0x433 as i32 as i64 - exp;
    if 53 as i32 as i64 <= shiftDist {
        if exact as i32 != 0 && exp as u64 | sig != 0 {
            softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                | softfloat_flag_inexact as i32) as uint_fast8_t;
        }
        return 0 as i32 as int_fast32_t;
    }
    sign = uiA >> 63 as i32 != 0;
    if shiftDist < 22 as i32 as i64 {
        if sign as i32 != 0 && exp == 0x41e as i32 as i64
            && sig < 0x200000 as u64
        {
            if exact as i32 != 0 && sig != 0 {
                softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                    | softfloat_flag_inexact as i32) as uint_fast8_t;
            }
            return (-(0x7fffffff as i32) - 1 as i32) as int_fast32_t;
        }
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
        return (if exp == 0x7ff as i32 as i64 && sig != 0 {
            -(0x7fffffff as i32) - 1 as i32
        } else if sign as i32 != 0 {
            -(0x7fffffff as i32) - 1 as i32
        } else {
            -(0x7fffffff as i32) - 1 as i32
        }) as int_fast32_t;
    }
    sig |= 0x10000000000000 as u64;
    absZ = (sig >> shiftDist) as int_fast32_t;
    if exact as i32 != 0 && (absZ as uint_fast32_t) << shiftDist != sig {
        softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
            | softfloat_flag_inexact as i32) as uint_fast8_t;
    }
    return if sign as i32 != 0 { -absZ } else { absZ };
}
