use crate::*;

pub type __uint16_t = u16;
pub type uint16_t = __uint16_t;
pub type int_fast8_t = i8;
pub type int_fast16_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui16_f16 {
    pub ui: uint16_t,
    pub f: float16_t,
}
#[inline]
unsafe fn softfloat_countLeadingZeros16(mut a: uint16_t) -> uint_fast8_t {
    return (if a as i32 != 0 {
        (a as u32).leading_zeros() as i32 - 16 as i32
    } else {
        16 as i32
    }) as uint_fast8_t;
}
pub unsafe fn softfloat_normRoundPackToF16(
    mut sign: bool,
    mut exp: int_fast16_t,
    mut sig: uint_fast16_t,
) -> float16_t {
    let mut shiftDist: int_fast8_t = 0;
    let mut uZ: ui16_f16 = ui16_f16 { ui: 0 };
    shiftDist = (softfloat_countLeadingZeros16(sig as uint16_t) as i32
        - 1 as i32) as int_fast8_t;
    exp -= shiftDist as i64;
    if 4 as i32 <= shiftDist as i32
        && (exp as u32) < 0x1d as i32 as u32
    {
        uZ
            .ui = ((((sign as uint16_t as i32) << 15 as i32)
            + (((if sig != 0 { exp } else { 0 as i32 as i64 })
                as uint16_t as i32) << 10 as i32)) as u64)
            .wrapping_add(sig << shiftDist as i32 - 4 as i32)
            as uint16_t;
        return uZ.f;
    } else {
        return softfloat_roundPackToF16(sign, exp, sig << shiftDist as i32)
    };
}
