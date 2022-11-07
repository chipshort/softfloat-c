use crate::*;

pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type int_fast8_t = i8;
pub type int_fast16_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui32_f32 {
    pub ui: uint32_t,
    pub f: float32_t,
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
pub unsafe fn ui64_to_f32(mut a: uint64_t) -> float32_t {
    let mut shiftDist: int_fast8_t = 0;
    let mut u: ui32_f32 = ui32_f32 { ui: 0 };
    let mut sig: uint_fast32_t = 0;
    shiftDist = (softfloat_countLeadingZeros64(a) as i32 - 40 as i32)
        as int_fast8_t;
    if 0 as i32 <= shiftDist as i32 {
        u
            .ui = (if a != 0 {
            (((0 as i32 as uint32_t) << 31 as i32)
                .wrapping_add(
                    ((0x95 as i32 - shiftDist as i32) as uint32_t)
                        << 23 as i32,
                ) as u64)
                .wrapping_add(a << shiftDist as i32)
        } else {
            0 as i32 as u64
        }) as uint32_t;
        return u.f;
    } else {
        shiftDist = (shiftDist as i32 + 7 as i32) as int_fast8_t;
        sig = if (shiftDist as i32) < 0 as i32 {
            softfloat_shortShiftRightJam64(
                a,
                -(shiftDist as i32) as uint_fast8_t,
            )
        } else {
            a << shiftDist as i32
        };
        return softfloat_roundPackToF32(
            0 as i32 != 0,
            (0x9c as i32 - shiftDist as i32) as int_fast16_t,
            sig,
        );
    };
}
