use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast8_t = i8;
pub type int_fast16_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui64_f64 {
    pub ui: uint64_t,
    pub f: float64_t,
}
#[inline]
unsafe fn softfloat_countLeadingZeros64(mut a: uint64_t) -> uint_fast8_t {
    return (if a != 0 {
        (a as u64).leading_zeros() as i32
    } else {
        64 as i32
    }) as uint_fast8_t;
}
pub unsafe fn softfloat_normRoundPackToF64(
    mut sign: bool,
    mut exp: int_fast16_t,
    mut sig: uint_fast64_t,
) -> float64_t {
    let mut shiftDist: int_fast8_t = 0;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    shiftDist = (softfloat_countLeadingZeros64(sig) as i32 - 1 as i32)
        as int_fast8_t;
    exp -= shiftDist as i64;
    if 10 as i32 <= shiftDist as i32
        && (exp as u32) < 0x7fd as i32 as u32
    {
        uZ
            .ui = ((sign as uint_fast64_t) << 63 as i32)
            .wrapping_add(
                ((if sig != 0 { exp } else { 0 as i32 as i64 })
                    as uint_fast64_t) << 52 as i32,
            )
            .wrapping_add(sig << shiftDist as i32 - 10 as i32);
        return uZ.f;
    } else {
        return softfloat_roundPackToF64(sign, exp, sig << shiftDist as i32)
    };
}
