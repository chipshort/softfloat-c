use crate::*;

pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type int_fast8_t = i8;
pub type int_fast32_t = i64;
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
#[inline]
unsafe fn softfloat_shortShiftRightJam128Extra(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut extra: uint64_t,
    mut dist: uint_fast8_t,
) -> uint128_extra {
    let mut negDist: uint_fast8_t = -(dist as i32) as uint_fast8_t;
    let mut z: uint128_extra = uint128_extra {
        extra: 0,
        v: uint128 { v0: 0, v64: 0 },
    };
    z.v.v64 = a64 >> dist as i32;
    z
        .v
        .v0 = a64 << (negDist as i32 & 63 as i32)
        | a0 >> dist as i32;
    z
        .extra = a0 << (negDist as i32 & 63 as i32)
        | (extra != 0 as i32 as u64) as i32 as u64;
    return z;
}
pub unsafe fn softfloat_normRoundPackToF128(
    mut sign: bool,
    mut exp: int_fast32_t,
    mut sig64: uint_fast64_t,
    mut sig0: uint_fast64_t,
) -> float128_t {
    let mut shiftDist: int_fast8_t = 0;
    let mut sig128: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut sigExtra: uint_fast64_t = 0;
    let mut sig128Extra: uint128_extra = uint128_extra {
        extra: 0,
        v: uint128 { v0: 0, v64: 0 },
    };
    if sig64 == 0 {
        exp -= 64 as i32 as i64;
        sig64 = sig0;
        sig0 = 0 as i32 as uint_fast64_t;
    }
    shiftDist = (softfloat_countLeadingZeros64(sig64) as i32 - 15 as i32)
        as int_fast8_t;
    exp -= shiftDist as i64;
    if 0 as i32 <= shiftDist as i32 {
        if shiftDist != 0 {
            sig128 = softfloat_shortShiftLeft128(sig64, sig0, shiftDist as uint_fast8_t);
            sig64 = sig128.v64;
            sig0 = sig128.v0;
        }
        if (exp as uint32_t) < 0x7ffd as i32 as u32 {
            uZ
                .ui
                .v64 = ((sign as uint_fast64_t) << 63 as i32)
                .wrapping_add(
                    ((if sig64 | sig0 != 0 {
                        exp
                    } else {
                        0 as i32 as i64
                    }) as uint_fast64_t) << 48 as i32,
                )
                .wrapping_add(sig64);
            uZ.ui.v0 = sig0;
            return uZ.f;
        }
        sigExtra = 0 as i32 as uint_fast64_t;
    } else {
        sig128Extra = softfloat_shortShiftRightJam128Extra(
            sig64,
            sig0,
            0 as i32 as uint64_t,
            -(shiftDist as i32) as uint_fast8_t,
        );
        sig64 = sig128Extra.v.v64;
        sig0 = sig128Extra.v.v0;
        sigExtra = sig128Extra.extra;
    }
    return softfloat_roundPackToF128(sign, exp, sig64, sig0, sigExtra);
}
