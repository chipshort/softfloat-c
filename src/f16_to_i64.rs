use crate::*;

pub type __uint16_t = u16;
pub type uint16_t = __uint16_t;
pub type int_fast8_t = i8;
pub type int_fast32_t = i64;
pub type int_fast64_t = i64;
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
pub unsafe fn f16_to_i64(
    mut a: float16_t,
    mut roundingMode: uint_fast8_t,
    mut exact: bool,
) -> int_fast64_t {
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiA: uint_fast16_t = 0;
    let mut sign: bool = false;
    let mut exp: int_fast8_t = 0;
    let mut frac: uint_fast16_t = 0;
    let mut sig32: int_fast32_t = 0;
    let mut shiftDist: int_fast8_t = 0;
    uA.f = a;
    uiA = uA.ui as uint_fast16_t;
    sign = uiA as uint16_t as i32 >> 15 as i32 != 0;
    exp = ((uiA >> 10 as i32) as int_fast8_t as i32
        & 0x1f as i32) as int_fast8_t;
    frac = uiA & 0x3ff as i32 as u64;
    if exp as i32 == 0x1f as i32 {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
        return if frac != 0 {
            -(0x7fffffffffffffff as i64) - 1 as i32 as i64
        } else if sign as i32 != 0 {
            -(0x7fffffffffffffff as i64) - 1 as i32 as i64
        } else {
            -(0x7fffffffffffffff as i64) - 1 as i32 as i64
        };
    }
    sig32 = frac as int_fast32_t;
    if exp != 0 {
        sig32 |= 0x400 as i32 as i64;
        shiftDist = (exp as i32 - 0x19 as i32) as int_fast8_t;
        if 0 as i32 <= shiftDist as i32 {
            sig32 <<= shiftDist as i32;
            return if sign as i32 != 0 { -sig32 } else { sig32 };
        }
        shiftDist = (exp as i32 - 0xd as i32) as int_fast8_t;
        if (0 as i32) < shiftDist as i32 {
            sig32 <<= shiftDist as i32;
        }
    }
    return softfloat_roundToI32(sign, sig32 as uint_fast32_t, roundingMode, exact);
}
