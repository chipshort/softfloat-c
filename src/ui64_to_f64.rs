use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
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
pub unsafe fn ui64_to_f64(mut a: uint64_t) -> float64_t {
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    if a == 0 {
        uZ.ui = 0 as i32 as uint64_t;
        return uZ.f;
    }
    if a & 0x8000000000000000 as u64 != 0 {
        return softfloat_roundPackToF64(
            0 as i32 != 0,
            0x43d as i32 as int_fast16_t,
            softfloat_shortShiftRightJam64(a, 1 as i32 as uint_fast8_t),
        )
    } else {
        return softfloat_normRoundPackToF64(
            0 as i32 != 0,
            0x43c as i32 as int_fast16_t,
            a,
        )
    };
}
