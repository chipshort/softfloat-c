use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type uint_fast8_t = u8;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;
pub type C2RustUnnamed = u32;
pub const softfloat_flag_invalid: C2RustUnnamed = 16;
pub const softfloat_flag_infinite: C2RustUnnamed = 8;
pub const softfloat_flag_overflow: C2RustUnnamed = 4;
pub const softfloat_flag_underflow: C2RustUnnamed = 2;
pub const softfloat_flag_inexact: C2RustUnnamed = 1;

pub unsafe fn softfloat_f32UIToCommonNaN(
    mut uiA: uint_fast32_t,
    mut zPtr: *mut commonNaN,
) {
    if uiA & 0x7fc00000 as i32 as u64
        == 0x7f800000 as i32 as u64
        && uiA & 0x3fffff as i32 as u64 != 0
    {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
    }
    (*zPtr).sign = uiA >> 31 as i32 != 0;
    (*zPtr).v64 = uiA << 41 as i32;
    (*zPtr).v0 = 0 as i32 as uint64_t;
}
