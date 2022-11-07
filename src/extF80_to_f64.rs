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


pub type extFloat80_t = extFloat80M;
#[derive(Copy, Clone)]
#[repr(C)]
pub union ui64_f64 {
    pub ui: uint64_t,
    pub f: float64_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s: extFloat80M,
    pub f: extFloat80_t,
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
pub unsafe fn extF80_to_f64(mut a: extFloat80_t) -> float64_t {
    let mut uA: C2RustUnnamed = C2RustUnnamed {
        s: extFloat80_t {
            signif: 0,
            signExp: 0,
        },
    };
    let mut uiA64: uint_fast16_t = 0;
    let mut uiA0: uint_fast64_t = 0;
    let mut sign: bool = false;
    let mut exp: int_fast32_t = 0;
    let mut sig: uint_fast64_t = 0;
    let mut commonNaN: commonNaN = commonNaN {
        sign: false,
        v0: 0,
        v64: 0,
    };
    let mut uiZ: uint_fast64_t = 0;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    uA.f = a;
    uiA64 = uA.s.signExp as uint_fast16_t;
    uiA0 = uA.s.signif;
    sign = uiA64 as uint16_t as i32 >> 15 as i32 != 0;
    exp = (uiA64 & 0x7fff as i32 as u64) as int_fast32_t;
    sig = uiA0;
    if exp as u64 | sig == 0 {
        uiZ = ((sign as uint_fast64_t) << 63 as i32)
            .wrapping_add((0 as i32 as uint_fast64_t) << 52 as i32)
            .wrapping_add(0 as i32 as u64);
    } else if exp == 0x7fff as i32 as i64 {
        if sig & 0x7fffffffffffffff as u64 != 0 {
            softfloat_extF80UIToCommonNaN(uiA64, uiA0, &mut commonNaN);
            uiZ = softfloat_commonNaNToF64UI(&mut commonNaN);
        } else {
            uiZ = ((sign as uint_fast64_t) << 63 as i32)
                .wrapping_add(
                    (0x7ff as i32 as uint_fast64_t) << 52 as i32,
                )
                .wrapping_add(0 as i32 as u64);
        }
    } else {
        sig = softfloat_shortShiftRightJam64(sig, 1 as i32 as uint_fast8_t);
        exp -= 0x3c01 as i32 as i64;
        if (::core::mem::size_of::<int_fast16_t>() as u64)
            < ::core::mem::size_of::<int_fast32_t>() as u64
        {
            if exp < -(0x1000 as i32) as i64 {
                exp = -(0x1000 as i32) as int_fast32_t;
            }
        }
        return softfloat_roundPackToF64(sign, exp, sig);
    }
    uZ.ui = uiZ;
    return uZ.f;
}
