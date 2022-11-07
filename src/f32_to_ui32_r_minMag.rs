use crate::*;

pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
pub type int_fast16_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast32_t = u64;

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
pub unsafe fn f32_to_ui32_r_minMag(
    mut a: float32_t,
    mut exact: bool,
) -> uint_fast32_t {
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiA: uint_fast32_t = 0;
    let mut exp: int_fast16_t = 0;
    let mut sig: uint_fast32_t = 0;
    let mut shiftDist: int_fast16_t = 0;
    let mut sign: bool = false;
    let mut z: uint_fast32_t = 0;
    uA.f = a;
    uiA = uA.ui as uint_fast32_t;
    exp = (uiA >> 23 as i32) as int_fast16_t
        & 0xff as i32 as i64;
    sig = uiA & 0x7fffff as i32 as u64;
    shiftDist = 0x9e as i32 as i64 - exp;
    if 32 as i32 as i64 <= shiftDist {
        if exact as i32 != 0 && exp as u64 | sig != 0 {
            softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                | softfloat_flag_inexact as i32) as uint_fast8_t;
        }
        return 0 as i32 as uint_fast32_t;
    }
    sign = uiA as uint32_t >> 31 as i32 != 0;
    if sign as i32 != 0 || shiftDist < 0 as i32 as i64 {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
        return (if exp == 0xff as i32 as i64 && sig != 0 {
            0xffffffff as u32
        } else if sign as i32 != 0 {
            0xffffffff as u32
        } else {
            0xffffffff as u32
        }) as uint_fast32_t;
    }
    sig = (sig | 0x800000 as i32 as u64) << 8 as i32;
    z = sig >> shiftDist;
    if exact as i32 != 0 && z << shiftDist != sig {
        softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
            | softfloat_flag_inexact as i32) as uint_fast8_t;
    }
    return z;
}
