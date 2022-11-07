use crate::*;

pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type int_fast16_t = i64;
pub type uint_fast32_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui32_f32 {
    pub ui: uint32_t,
    pub f: float32_t,
}
pub unsafe fn i32_to_f32(mut a: int32_t) -> float32_t {
    let mut sign: bool = false;
    let mut uZ: ui32_f32 = ui32_f32 { ui: 0 };
    let mut absA: uint_fast32_t = 0;
    sign = a < 0 as i32;
    if a & 0x7fffffff as i32 == 0 {
        uZ
            .ui = if sign as i32 != 0 {
            ((1 as i32 as uint32_t) << 31 as i32)
                .wrapping_add((0x9e as i32 as uint32_t) << 23 as i32)
                .wrapping_add(0 as i32 as u32)
        } else {
            0 as i32 as u32
        };
        return uZ.f;
    }
    absA = if sign as i32 != 0 {
        (a as uint_fast32_t).wrapping_neg()
    } else {
        a as uint_fast32_t
    };
    return softfloat_normRoundPackToF32(sign, 0x9c as i32 as int_fast16_t, absA);
}
