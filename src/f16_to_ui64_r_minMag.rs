use crate::*;

pub type __uint16_t = u16;
pub type uint16_t = __uint16_t;
pub type int_fast8_t = i8;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui16_f16 {
    pub ui: uint16_t,
    pub f: float16_t,
}
pub type C2RustUnnamed = u32;
pub const softfloat_flag_invalid: C2RustUnnamed = 16;
pub const softfloat_flag_infinite: C2RustUnnamed = 8;
pub const softfloat_flag_overflow: C2RustUnnamed = 4;
pub const softfloat_flag_underflow: C2RustUnnamed = 2;
pub const softfloat_flag_inexact: C2RustUnnamed = 1;
pub unsafe fn f16_to_ui64_r_minMag(
    mut a: float16_t,
    mut exact: bool,
) -> uint_fast64_t {
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiA: uint_fast16_t = 0;
    let mut exp: int_fast8_t = 0;
    let mut frac: uint_fast16_t = 0;
    let mut shiftDist: int_fast8_t = 0;
    let mut sign: bool = false;
    let mut alignedSig: uint_fast32_t = 0;
    uA.f = a;
    uiA = uA.ui as uint_fast16_t;
    exp = ((uiA >> 10 as i32) as int_fast8_t as i32
        & 0x1f as i32) as int_fast8_t;
    frac = uiA & 0x3ff as i32 as u64;
    shiftDist = (exp as i32 - 0xf as i32) as int_fast8_t;
    if (shiftDist as i32) < 0 as i32 {
        if exact as i32 != 0 && exp as u64 | frac != 0 {
            softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                | softfloat_flag_inexact as i32) as uint_fast8_t;
        }
        return 0 as i32 as uint_fast64_t;
    }
    sign = uiA as uint16_t as i32 >> 15 as i32 != 0;
    if sign as i32 != 0 || exp as i32 == 0x1f as i32 {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
        return if exp as i32 == 0x1f as i32 && frac != 0 {
            0xffffffffffffffff as u64
        } else if sign as i32 != 0 {
            0xffffffffffffffff as u64
        } else {
            0xffffffffffffffff as u64
        };
    }
    alignedSig = (frac | 0x400 as i32 as u64)
        << shiftDist as i32;
    if exact as i32 != 0
        && alignedSig & 0x3ff as i32 as u64 != 0
    {
        softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
            | softfloat_flag_inexact as i32) as uint_fast8_t;
    }
    return alignedSig >> 10 as i32;
}
