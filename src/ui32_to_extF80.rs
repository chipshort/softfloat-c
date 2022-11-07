use crate::*;

pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type int_fast8_t = i8;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast64_t = u64;

pub type extFloat80_t = extFloat80M;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s: extFloat80M,
    pub f: extFloat80_t,
}
#[inline]
unsafe fn softfloat_countLeadingZeros32(mut a: uint32_t) -> uint_fast8_t {
    return (if a != 0 { a.leading_zeros() as i32 } else { 32 as i32 })
        as uint_fast8_t;
}
pub unsafe fn ui32_to_extF80(mut a: uint32_t) -> extFloat80_t {
    let mut uiZ64: uint_fast16_t = 0;
    let mut shiftDist: int_fast8_t = 0;
    let mut uZ: C2RustUnnamed = C2RustUnnamed {
        s: extFloat80M {
            signif: 0,
            signExp: 0,
        },
    };
    uiZ64 = 0 as i32 as uint_fast16_t;
    if a != 0 {
        shiftDist = softfloat_countLeadingZeros32(a) as int_fast8_t;
        uiZ64 = (0x401e as i32 - shiftDist as i32) as uint_fast16_t;
        a <<= shiftDist as i32;
    }
    uZ.s.signExp = uiZ64 as uint16_t;
    uZ.s.signif = (a as uint_fast64_t) << 32 as i32;
    return uZ.f;
}
