use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
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
pub unsafe fn f64_lt_quiet(mut a: float64_t, mut b: float64_t) -> bool {
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: uint_fast64_t = 0;
    let mut uB: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiB: uint_fast64_t = 0;
    let mut signA: bool = false;
    let mut signB: bool = false;
    uA.f = a;
    uiA = uA.ui;
    uB.f = b;
    uiB = uB.ui;
    if !uiA & 0x7ff0000000000000 as u64 == 0 as i32 as u64
        && uiA & 0xfffffffffffff as u64 != 0
        || !uiB & 0x7ff0000000000000 as u64
            == 0 as i32 as u64
            && uiB & 0xfffffffffffff as u64 != 0
    {
        if uiA & 0x7ff8000000000000 as u64
            == 0x7ff0000000000000 as u64
            && uiA & 0x7ffffffffffff as u64 != 0
            || uiB & 0x7ff8000000000000 as u64
                == 0x7ff0000000000000 as u64
                && uiB & 0x7ffffffffffff as u64 != 0
        {
            softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
        }
        return 0 as i32 != 0;
    }
    signA = uiA >> 63 as i32 != 0;
    signB = uiB >> 63 as i32 != 0;
    return if signA as i32 != signB as i32 {
        (signA as i32 != 0
            && (uiA | uiB) & 0x7fffffffffffffff as u64 != 0) as i32
    } else {
        (uiA != uiB && signA as i32 ^ (uiA < uiB) as i32 != 0)
            as i32
    } != 0;
}
