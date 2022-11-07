use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast16_t = i64;
pub type int_fast32_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast64_t = u64;



#[derive(Copy, Clone)]
#[repr(C)]
pub union ui64_f64 {
    pub ui: uint64_t,
    pub f: float64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union ui128_f128 {
    pub ui: uint128,
    pub f: float128_t,
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
pub unsafe fn f128_to_f64(mut a: float128_t) -> float64_t {
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiA64: uint_fast64_t = 0;
    let mut uiA0: uint_fast64_t = 0;
    let mut sign: bool = false;
    let mut exp: int_fast32_t = 0;
    let mut frac64: uint_fast64_t = 0;
    let mut frac0: uint_fast64_t = 0;
    let mut commonNaN: commonNaN = commonNaN {
        sign: false,
        v0: 0,
        v64: 0,
    };
    let mut uiZ: uint_fast64_t = 0;
    let mut frac128: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    sign = uiA64 >> 63 as i32 != 0;
    exp = (uiA64 >> 48 as i32) as int_fast32_t
        & 0x7fff as i32 as i64;
    frac64 = uiA64 & 0xffffffffffff as u64;
    frac0 = uiA0;
    if exp == 0x7fff as i32 as i64 {
        if frac64 | frac0 != 0 {
            softfloat_f128UIToCommonNaN(uiA64, uiA0, &mut commonNaN);
            uiZ = softfloat_commonNaNToF64UI(&mut commonNaN);
        } else {
            uiZ = ((sign as uint_fast64_t) << 63 as i32)
                .wrapping_add(
                    (0x7ff as i32 as uint_fast64_t) << 52 as i32,
                )
                .wrapping_add(0 as i32 as u64);
        }
    } else {
        frac128 = softfloat_shortShiftLeft128(
            frac64,
            frac0,
            14 as i32 as uint_fast8_t,
        );
        frac64 = frac128.v64
            | (frac128.v0 != 0 as i32 as u64) as i32
                as u64;
        if exp as u64 | frac64 == 0 {
            uiZ = ((sign as uint_fast64_t) << 63 as i32)
                .wrapping_add((0 as i32 as uint_fast64_t) << 52 as i32)
                .wrapping_add(0 as i32 as u64);
        } else {
            exp -= 0x3c01 as i32 as i64;
            if (::core::mem::size_of::<int_fast16_t>() as u64)
                < ::core::mem::size_of::<int_fast32_t>() as u64
            {
                if exp < -(0x1000 as i32) as i64 {
                    exp = -(0x1000 as i32) as int_fast32_t;
                }
            }
            return softfloat_roundPackToF64(
                sign,
                exp,
                frac64 | 0x4000000000000000 as u64,
            );
        }
    }
    uZ.ui = uiZ;
    return uZ.f;
}
