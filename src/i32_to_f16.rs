use crate::*;

pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type int32_t = __int32_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type int_fast8_t = i8;
pub type int_fast16_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast32_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui16_f16 {
    pub ui: uint16_t,
    pub f: float16_t,
}
#[inline]
unsafe fn softfloat_countLeadingZeros32(mut a: uint32_t) -> uint_fast8_t {
    return (if a != 0 { a.leading_zeros() as i32 } else { 32 as i32 })
        as uint_fast8_t;
}
pub unsafe fn i32_to_f16(mut a: int32_t) -> float16_t {
    let mut sign: bool = false;
    let mut absA: uint_fast32_t = 0;
    let mut shiftDist: int_fast8_t = 0;
    let mut u: ui16_f16 = ui16_f16 { ui: 0 };
    let mut sig: uint_fast16_t = 0;
    sign = a < 0 as i32;
    absA = if sign as i32 != 0 {
        (a as uint_fast32_t).wrapping_neg()
    } else {
        a as uint_fast32_t
    };
    shiftDist = (softfloat_countLeadingZeros32(absA as uint32_t) as i32
        - 21 as i32) as int_fast8_t;
    if 0 as i32 <= shiftDist as i32 {
        u
            .ui = (if a != 0 {
            ((((sign as uint16_t as i32) << 15 as i32)
                + (((0x18 as i32 - shiftDist as i32) as uint16_t
                    as i32) << 10 as i32)) as u64)
                .wrapping_add(absA << shiftDist as i32)
        } else {
            0 as i32 as u64
        }) as uint16_t;
        return u.f;
    } else {
        shiftDist = (shiftDist as i32 + 4 as i32) as int_fast8_t;
        sig = if (shiftDist as i32) < 0 as i32 {
            absA >> -(shiftDist as i32)
                | ((absA << (shiftDist as i32 & 31 as i32)) as uint32_t
                    != 0 as i32 as u32) as i32 as u64
        } else {
            absA << shiftDist as i32
        };
        return softfloat_roundPackToF16(
            sign,
            (0x1c as i32 - shiftDist as i32) as int_fast16_t,
            sig,
        );
    };
}
