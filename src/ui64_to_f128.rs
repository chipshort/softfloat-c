use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast8_t = i8;
pub type uint_fast8_t = u8;
pub type uint_fast64_t = u64;


#[derive(Copy, Clone)]
#[repr(C)]
pub union ui128_f128 {
    pub ui: uint128,
    pub f: float128_t,
}
#[inline]
unsafe fn softfloat_countLeadingZeros64(mut a: uint64_t) -> uint_fast8_t {
    return (if a != 0 {
        (a as u64).leading_zeros() as i32
    } else {
        64 as i32
    }) as uint_fast8_t;
}
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
pub unsafe fn ui64_to_f128(mut a: uint64_t) -> float128_t {
    let mut uiZ64: uint_fast64_t = 0;
    let mut uiZ0: uint_fast64_t = 0;
    let mut shiftDist: int_fast8_t = 0;
    let mut zSig: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    if a == 0 {
        uiZ64 = 0 as i32 as uint_fast64_t;
        uiZ0 = 0 as i32 as uint_fast64_t;
    } else {
        shiftDist = (softfloat_countLeadingZeros64(a) as i32 + 49 as i32)
            as int_fast8_t;
        if 64 as i32 <= shiftDist as i32 {
            zSig.v64 = a << shiftDist as i32 - 64 as i32;
            zSig.v0 = 0 as i32 as uint64_t;
        } else {
            zSig = softfloat_shortShiftLeft128(
                0 as i32 as uint64_t,
                a,
                shiftDist as uint_fast8_t,
            );
        }
        uiZ64 = ((0 as i32 as uint_fast64_t) << 63 as i32)
            .wrapping_add(
                ((0x406e as i32 - shiftDist as i32) as uint_fast64_t)
                    << 48 as i32,
            )
            .wrapping_add(zSig.v64);
        uiZ0 = zSig.v0;
    }
    uZ.ui.v64 = uiZ64;
    uZ.ui.v0 = uiZ0;
    return uZ.f;
}
