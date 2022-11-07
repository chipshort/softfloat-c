use crate::*;

pub type __uint16_t = u16;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int64_t = __int64_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type int_fast8_t = i8;
pub type int_fast16_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui16_f16 {
    pub ui: uint16_t,
    pub f: float16_t,
}
#[inline]
unsafe fn softfloat_countLeadingZeros64(mut a: uint64_t) -> uint_fast8_t {
    return (if a != 0 {
        (a as u64).leading_zeros() as i32
    } else {
        64 as i32
    }) as uint_fast8_t;
}
#[inline]
unsafe fn softfloat_shortShiftRightJam64(
    mut a: uint64_t,
    mut dist: uint_fast8_t,
) -> uint64_t {
    return a >> dist as i32
        | (a
            & ((1 as i32 as uint_fast64_t) << dist as i32)
                .wrapping_sub(1 as i32 as u64)
            != 0 as i32 as u64) as i32 as u64;
}
pub unsafe fn i64_to_f16(mut a: int64_t) -> float16_t {
    let mut sign: bool = false;
    let mut absA: uint_fast64_t = 0;
    let mut shiftDist: int_fast8_t = 0;
    let mut u: ui16_f16 = ui16_f16 { ui: 0 };
    let mut sig: uint_fast16_t = 0;
    sign = a < 0 as i32 as i64;
    absA = if sign as i32 != 0 {
        (a as uint_fast64_t).wrapping_neg()
    } else {
        a as uint_fast64_t
    };
    shiftDist = (softfloat_countLeadingZeros64(absA) as i32 - 53 as i32)
        as int_fast8_t;
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
            softfloat_shortShiftRightJam64(
                absA,
                -(shiftDist as i32) as uint_fast8_t,
            )
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
