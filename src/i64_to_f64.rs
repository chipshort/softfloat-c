use crate::*;

pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int64_t = __int64_t;
pub type uint64_t = __uint64_t;
pub type int_fast16_t = i64;
pub type uint_fast64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui64_f64 {
    pub ui: uint64_t,
    pub f: float64_t,
}
pub unsafe fn i64_to_f64(mut a: int64_t) -> float64_t {
    let mut sign: bool = false;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    let mut absA: uint_fast64_t = 0;
    sign = a < 0 as i32 as i64;
    if a as u64 & 0x7fffffffffffffff as u64 == 0 {
        uZ
            .ui = if sign as i32 != 0 {
            ((1 as i32 as uint_fast64_t) << 63 as i32)
                .wrapping_add(
                    (0x43e as i32 as uint_fast64_t) << 52 as i32,
                )
                .wrapping_add(0 as i32 as u64)
        } else {
            0 as i32 as u64
        };
        return uZ.f;
    }
    absA = if sign as i32 != 0 {
        (a as uint_fast64_t).wrapping_neg()
    } else {
        a as uint_fast64_t
    };
    return softfloat_normRoundPackToF64(
        sign,
        0x43c as i32 as int_fast16_t,
        absA,
    );
}
