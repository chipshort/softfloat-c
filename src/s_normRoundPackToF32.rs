use crate::*;

pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
pub type int_fast8_t = i8;
pub type int_fast16_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast32_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui32_f32 {
    pub ui: uint32_t,
    pub f: float32_t,
}
#[inline]
unsafe fn softfloat_countLeadingZeros32(mut a: uint32_t) -> uint_fast8_t {
    return (if a != 0 { a.leading_zeros() as i32 } else { 32 as i32 })
        as uint_fast8_t;
}
pub unsafe fn softfloat_normRoundPackToF32(
    mut sign: bool,
    mut exp: int_fast16_t,
    mut sig: uint_fast32_t,
) -> float32_t {
    let mut shiftDist: int_fast8_t = 0;
    let mut uZ: ui32_f32 = ui32_f32 { ui: 0 };
    shiftDist = (softfloat_countLeadingZeros32(sig as uint32_t) as i32
        - 1 as i32) as int_fast8_t;
    exp -= shiftDist as i64;
    if 7 as i32 <= shiftDist as i32
        && (exp as u32) < 0xfd as i32 as u32
    {
        uZ
            .ui = (((sign as uint32_t) << 31 as i32)
            .wrapping_add(
                ((if sig != 0 { exp } else { 0 as i32 as i64 })
                    as uint32_t) << 23 as i32,
            ) as u64)
            .wrapping_add(sig << shiftDist as i32 - 7 as i32)
            as uint32_t;
        return uZ.f;
    } else {
        return softfloat_roundPackToF32(sign, exp, sig << shiftDist as i32)
    };
}
