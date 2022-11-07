use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type uint_fast8_t = u8;
pub type uint_fast64_t = u64;


#[inline]
unsafe fn softfloat_shortShiftRight128(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut dist: uint_fast8_t,
) -> uint128 {
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    z.v64 = a64 >> dist as i32;
    z
        .v0 = a64 << (-(dist as i32) & 63 as i32)
        | a0 >> dist as i32;
    return z;
}
pub unsafe fn softfloat_commonNaNToF128UI(
    mut aPtr: *const commonNaN,
) -> uint128 {
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    uiZ = softfloat_shortShiftRight128(
        (*aPtr).v64,
        (*aPtr).v0,
        16 as i32 as uint_fast8_t,
    );
    uiZ.v64
        |= ((*aPtr).sign as uint_fast64_t) << 63 as i32
            | 0x7fff800000000000 as u64;
    return uiZ;
}
