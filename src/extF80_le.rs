use crate::*;

pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast64_t = u64;

pub type extFloat80_t = extFloat80M;
pub type C2RustUnnamed = u32;
pub const softfloat_flag_invalid: C2RustUnnamed = 16;
pub const softfloat_flag_infinite: C2RustUnnamed = 8;
pub const softfloat_flag_overflow: C2RustUnnamed = 4;
pub const softfloat_flag_underflow: C2RustUnnamed = 2;
pub const softfloat_flag_inexact: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub s: extFloat80M,
    pub f: extFloat80_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub s: extFloat80M,
    pub f: extFloat80_t,
}
#[inline]
unsafe fn softfloat_lt128(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut b64: uint64_t,
    mut b0: uint64_t,
) -> bool {
    return a64 < b64 || a64 == b64 && a0 < b0;
}
pub unsafe fn extF80_le(mut a: extFloat80_t, mut b: extFloat80_t) -> bool {
    let mut uA: C2RustUnnamed_1 = C2RustUnnamed_1 {
        s: extFloat80_t {
            signif: 0,
            signExp: 0,
        },
    };
    let mut uiA64: uint_fast16_t = 0;
    let mut uiA0: uint_fast64_t = 0;
    let mut uB: C2RustUnnamed_0 = C2RustUnnamed_0 {
        s: extFloat80_t {
            signif: 0,
            signExp: 0,
        },
    };
    let mut uiB64: uint_fast16_t = 0;
    let mut uiB0: uint_fast64_t = 0;
    let mut signA: bool = false;
    let mut signB: bool = false;
    uA.f = a;
    uiA64 = uA.s.signExp as uint_fast16_t;
    uiA0 = uA.s.signif;
    uB.f = b;
    uiB64 = uB.s.signExp as uint_fast16_t;
    uiB0 = uB.s.signif;
    if uiA64 & 0x7fff as i32 as u64
        == 0x7fff as i32 as u64
        && uiA0 & 0x7fffffffffffffff as u64 != 0
        || uiB64 & 0x7fff as i32 as u64
            == 0x7fff as i32 as u64
            && uiB0 & 0x7fffffffffffffff as u64 != 0
    {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
        return 0 as i32 != 0;
    }
    signA = uiA64 as uint16_t as i32 >> 15 as i32 != 0;
    signB = uiB64 as uint16_t as i32 >> 15 as i32 != 0;
    return if signA as i32 != signB as i32 {
        (signA as i32 != 0
            || (uiA64 | uiB64) & 0x7fff as i32 as u64 | uiA0 | uiB0
                == 0) as i32
    } else {
        (uiA64 == uiB64 && uiA0 == uiB0
            || signA as i32
                ^ softfloat_lt128(uiA64, uiA0, uiB64, uiB0) as i32 != 0)
            as i32
    } != 0;
}
