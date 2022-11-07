use crate::*;

pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
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
pub unsafe fn f32_le(mut a: float32_t, mut b: float32_t) -> bool {
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiA: uint_fast32_t = 0;
    let mut uB: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiB: uint_fast32_t = 0;
    let mut signA: bool = false;
    let mut signB: bool = false;
    uA.f = a;
    uiA = uA.ui as uint_fast32_t;
    uB.f = b;
    uiB = uB.ui as uint_fast32_t;
    if !uiA & 0x7f800000 as i32 as u64
        == 0 as i32 as u64
        && uiA & 0x7fffff as i32 as u64 != 0
        || !uiB & 0x7f800000 as i32 as u64
            == 0 as i32 as u64
            && uiB & 0x7fffff as i32 as u64 != 0
    {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
        return 0 as i32 != 0;
    }
    signA = uiA as uint32_t >> 31 as i32 != 0;
    signB = uiB as uint32_t >> 31 as i32 != 0;
    return if signA as i32 != signB as i32 {
        (signA as i32 != 0 || ((uiA | uiB) << 1 as i32) as uint32_t == 0)
            as i32
    } else {
        (uiA == uiB || signA as i32 ^ (uiA < uiB) as i32 != 0)
            as i32
    } != 0;
}
