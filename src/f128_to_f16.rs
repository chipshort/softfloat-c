use crate::*;

pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type int_fast16_t = i64;
pub type int_fast32_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast64_t = u64;



#[derive(Copy, Clone)]
#[repr(C)]
pub union ui16_f16 {
    pub ui: uint16_t,
    pub f: float16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union ui128_f128 {
    pub ui: uint128,
    pub f: float128_t,
}

#[inline]
unsafe fn softfloat_shortShiftRightJam64(
    mut a: uint64_t,
    mut dist: uint_fast8_t,
) -> uint64_t {
    return a >> dist as i32
        | (a
            & ((1 as i32 as uint_fast64_t) << dist as i32)
                .wrapping_sub(1 as i32 as u64)
            != 0 as i32 as u64) as i32 as u64;
}
pub unsafe fn f128_to_f16(mut a: float128_t) -> float16_t {
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiA64: uint_fast64_t = 0;
    let mut uiA0: uint_fast64_t = 0;
    let mut sign: bool = false;
    let mut exp: int_fast32_t = 0;
    let mut frac64: uint_fast64_t = 0;
    let mut commonNaN: commonNaN = commonNaN {
        sign: false,
        v0: 0,
        v64: 0,
    };
    let mut uiZ: uint_fast16_t = 0;
    let mut frac16: uint_fast16_t = 0;
    let mut uZ: ui16_f16 = ui16_f16 { ui: 0 };
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    sign = uiA64 >> 63 as i32 != 0;
    exp = (uiA64 >> 48 as i32) as int_fast32_t
        & 0x7fff as i32 as i64;
    frac64 = uiA64 & 0xffffffffffff as u64
        | (uiA0 != 0 as i32 as u64) as i32 as u64;
    if exp == 0x7fff as i32 as i64 {
        if frac64 != 0 {
            softfloat_f128UIToCommonNaN(uiA64, uiA0, &mut commonNaN);
            uiZ = softfloat_commonNaNToF16UI(&mut commonNaN);
        } else {
            uiZ = (((sign as uint16_t as i32) << 15 as i32)
                + ((0x1f as i32 as uint16_t as i32) << 10 as i32)
                + 0 as i32) as uint_fast16_t;
        }
    } else {
        frac16 = softfloat_shortShiftRightJam64(
            frac64,
            34 as i32 as uint_fast8_t,
        );
        if exp as u64 | frac16 == 0 {
            uiZ = (((sign as uint16_t as i32) << 15 as i32)
                + ((0 as i32 as uint16_t as i32) << 10 as i32)
                + 0 as i32) as uint_fast16_t;
        } else {
            exp -= 0x3ff1 as i32 as i64;
            if (::core::mem::size_of::<int_fast16_t>() as u64)
                < ::core::mem::size_of::<int_fast32_t>() as u64
            {
                if exp < -(0x40 as i32) as i64 {
                    exp = -(0x40 as i32) as int_fast32_t;
                }
            }
            return softfloat_roundPackToF16(
                sign,
                exp,
                frac16 | 0x4000 as i32 as u64,
            );
        }
    }
    uZ.ui = uiZ as uint16_t;
    return uZ.f;
}
