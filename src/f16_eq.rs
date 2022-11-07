use crate::*;

pub type __uint16_t = u16;
pub type uint16_t = __uint16_t;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;

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
pub unsafe fn f16_eq(mut a: float16_t, mut b: float16_t) -> bool {
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiA: uint_fast16_t = 0;
    let mut uB: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiB: uint_fast16_t = 0;
    uA.f = a;
    uiA = uA.ui as uint_fast16_t;
    uB.f = b;
    uiB = uB.ui as uint_fast16_t;
    if !uiA & 0x7c00 as i32 as u64 == 0 as i32 as u64
        && uiA & 0x3ff as i32 as u64 != 0
        || !uiB & 0x7c00 as i32 as u64
            == 0 as i32 as u64
            && uiB & 0x3ff as i32 as u64 != 0
    {
        if uiA & 0x7e00 as i32 as u64
            == 0x7c00 as i32 as u64
            && uiA & 0x1ff as i32 as u64 != 0
            || uiB & 0x7e00 as i32 as u64
                == 0x7c00 as i32 as u64
                && uiB & 0x1ff as i32 as u64 != 0
        {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
        }
        return 0 as i32 != 0;
    }
    return uiA == uiB || ((uiA | uiB) << 1 as i32) as uint16_t == 0;
}
