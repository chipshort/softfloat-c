use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type uint_fast8_t = u8;
pub type uint_fast64_t = u64;

pub type C2RustUnnamed = u32;
pub const softfloat_flag_invalid: C2RustUnnamed = 16;
pub const softfloat_flag_infinite: C2RustUnnamed = 8;
pub const softfloat_flag_overflow: C2RustUnnamed = 4;
pub const softfloat_flag_underflow: C2RustUnnamed = 2;
pub const softfloat_flag_inexact: C2RustUnnamed = 1;

#[inline]
unsafe fn softfloat_shortShiftLeft128(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut dist: uint_fast8_t,
) -> uint128 {
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    z
        .v64 = a64 << dist as i32
        | a0 >> (-(dist as i32) & 63 as i32);
    z.v0 = a0 << dist as i32;
    return z;
}
pub unsafe fn softfloat_f128UIToCommonNaN(
    mut uiA64: uint_fast64_t,
    mut uiA0: uint_fast64_t,
    mut zPtr: *mut commonNaN,
) {
    let mut NaNSig: uint128 = uint128 { v0: 0, v64: 0 };
    if uiA64 & 0x7fff800000000000 as u64 == 0x7fff000000000000 as u64
        && (uiA0 != 0 || uiA64 & 0x7fffffffffff as u64 != 0)
    {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
    }
    NaNSig = softfloat_shortShiftLeft128(uiA64, uiA0, 16 as i32 as uint_fast8_t);
    (*zPtr).sign = uiA64 >> 63 as i32 != 0;
    (*zPtr).v64 = NaNSig.v64;
    (*zPtr).v0 = NaNSig.v0;
}
