use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type uint_fast16_t = u64;


pub unsafe fn softfloat_commonNaNToExtF80UI(
    mut aPtr: *const commonNaN,
) -> uint128 {
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    uiZ
        .v64 = ((*aPtr).sign as uint_fast16_t) << 15 as i32
        | 0x7fff as i32 as u64;
    uiZ.v0 = 0xc000000000000000 as u64 | (*aPtr).v64 >> 1 as i32;
    return uiZ;
}
