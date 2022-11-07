use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast16_t = i64;
pub type int_fast64_t = i64;
pub type uint_fast8_t = u8;
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
pub unsafe fn f64_to_i64_r_minMag(
    mut a: float64_t,
    mut exact: bool,
) -> int_fast64_t {
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: uint_fast64_t = 0;
    let mut sign: bool = false;
    let mut exp: int_fast16_t = 0;
    let mut sig: uint_fast64_t = 0;
    let mut shiftDist: int_fast16_t = 0;
    let mut absZ: int_fast64_t = 0;
    uA.f = a;
    uiA = uA.ui;
    sign = uiA >> 63 as i32 != 0;
    exp = (uiA >> 52 as i32) as int_fast16_t
        & 0x7ff as i32 as i64;
    sig = uiA & 0xfffffffffffff as u64;
    shiftDist = 0x433 as i32 as i64 - exp;
    if shiftDist <= 0 as i32 as i64 {
        if shiftDist < -(10 as i32) as i64 {
            if uiA
                == ((1 as i32 as uint_fast64_t) << 63 as i32)
                    .wrapping_add(
                        (0x43e as i32 as uint_fast64_t) << 52 as i32,
                    )
                    .wrapping_add(0 as i32 as u64)
            {
                return -(0x7fffffffffffffff as i64)
                    - 1 as i32 as i64;
            }
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
            return if exp == 0x7ff as i32 as i64 && sig != 0 {
                -(0x7fffffffffffffff as i64) - 1 as i32 as i64
            } else if sign as i32 != 0 {
                -(0x7fffffffffffffff as i64) - 1 as i32 as i64
            } else {
                -(0x7fffffffffffffff as i64) - 1 as i32 as i64
            };
        }
        sig |= 0x10000000000000 as u64;
        absZ = (sig << -shiftDist) as int_fast64_t;
    } else {
        if 53 as i32 as i64 <= shiftDist {
            if exact as i32 != 0 && exp as u64 | sig != 0 {
                softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                    | softfloat_flag_inexact as i32) as uint_fast8_t;
            }
            return 0 as i32 as int_fast64_t;
        }
        sig |= 0x10000000000000 as u64;
        absZ = (sig >> shiftDist) as int_fast64_t;
        if exact as i32 != 0 && (absZ << shiftDist) as u64 != sig {
            softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                | softfloat_flag_inexact as i32) as uint_fast8_t;
        }
    }
    return if sign as i32 != 0 { -absZ } else { absZ };
}
