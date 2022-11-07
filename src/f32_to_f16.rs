use crate::*;

pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type int_fast16_t = i64;
pub type uint_fast16_t = u64;
pub type uint_fast32_t = u64;


#[derive(Copy, Clone)]
#[repr(C)]
pub union ui16_f16 {
    pub ui: uint16_t,
    pub f: float16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union ui32_f32 {
    pub ui: uint32_t,
    pub f: float32_t,
}

pub unsafe fn f32_to_f16(mut a: float32_t) -> float16_t {
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiA: uint_fast32_t = 0;
    let mut sign: bool = false;
    let mut exp: int_fast16_t = 0;
    let mut frac: uint_fast32_t = 0;
    let mut commonNaN: commonNaN = commonNaN {
        sign: false,
        v0: 0,
        v64: 0,
    };
    let mut uiZ: uint_fast16_t = 0;
    let mut frac16: uint_fast16_t = 0;
    let mut uZ: ui16_f16 = ui16_f16 { ui: 0 };
    uA.f = a;
    uiA = uA.ui as uint_fast32_t;
    sign = uiA as uint32_t >> 31 as i32 != 0;
    exp = (uiA >> 23 as i32) as int_fast16_t
        & 0xff as i32 as i64;
    frac = uiA & 0x7fffff as i32 as u64;
    if exp == 0xff as i32 as i64 {
        if frac != 0 {
            softfloat_f32UIToCommonNaN(uiA, &mut commonNaN);
            uiZ = softfloat_commonNaNToF16UI(&mut commonNaN);
        } else {
            uiZ = (((sign as uint16_t as i32) << 15 as i32)
                + ((0x1f as i32 as uint16_t as i32) << 10 as i32)
                + 0 as i32) as uint_fast16_t;
        }
    } else {
        frac16 = frac >> 9 as i32
            | (frac & 0x1ff as i32 as u64
                != 0 as i32 as u64) as i32 as u64;
        if exp as u64 | frac16 == 0 {
            uiZ = (((sign as uint16_t as i32) << 15 as i32)
                + ((0 as i32 as uint16_t as i32) << 10 as i32)
                + 0 as i32) as uint_fast16_t;
        } else {
            return softfloat_roundPackToF16(
                sign,
                exp - 0x71 as i32 as i64,
                frac16 | 0x4000 as i32 as u64,
            )
        }
    }
    uZ.ui = uiZ as uint16_t;
    return uZ.f;
}
