use crate::*;

pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
pub type int_fast16_t = i64;
pub type uint_fast32_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui32_f32 {
    pub ui: uint32_t,
    pub f: float32_t,
}
pub unsafe fn ui32_to_f32(mut a: uint32_t) -> float32_t {
    let mut uZ: ui32_f32 = ui32_f32 { ui: 0 };
    if a == 0 {
        uZ.ui = 0 as i32 as uint32_t;
        return uZ.f;
    }
    if a & 0x80000000 as u32 != 0 {
        return softfloat_roundPackToF32(
            0 as i32 != 0,
            0x9d as i32 as int_fast16_t,
            (a >> 1 as i32 | a & 1 as i32 as u32)
                as uint_fast32_t,
        )
    } else {
        return softfloat_normRoundPackToF32(
            0 as i32 != 0,
            0x9c as i32 as int_fast16_t,
            a as uint_fast32_t,
        )
    };
}
