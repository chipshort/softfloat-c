use crate::*;

pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast64_t = u64;



pub type extFloat80_t = extFloat80M;
#[derive(Copy, Clone)]
#[repr(C)]
pub union ui128_f128 {
    pub ui: uint128,
    pub f: float128_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s: extFloat80M,
    pub f: extFloat80_t,
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
pub unsafe fn extF80_to_f128(mut a: extFloat80_t) -> float128_t {
    let mut uA: C2RustUnnamed = C2RustUnnamed {
        s: extFloat80_t {
            signif: 0,
            signExp: 0,
        },
    };
    let mut uiA64: uint_fast16_t = 0;
    let mut uiA0: uint_fast64_t = 0;
    let mut exp: uint_fast16_t = 0;
    let mut frac: uint_fast64_t = 0;
    let mut commonNaN: commonNaN = commonNaN {
        sign: false,
        v0: 0,
        v64: 0,
    };
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut sign: bool = false;
    let mut frac128: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    uA.f = a;
    uiA64 = uA.s.signExp as uint_fast16_t;
    uiA0 = uA.s.signif;
    exp = uiA64 & 0x7fff as i32 as u64;
    frac = uiA0 & 0x7fffffffffffffff as u64;
    if exp == 0x7fff as i32 as u64 && frac != 0 {
        softfloat_extF80UIToCommonNaN(uiA64, uiA0, &mut commonNaN);
        uiZ = softfloat_commonNaNToF128UI(&mut commonNaN);
    } else {
        sign = uiA64 as uint16_t as i32 >> 15 as i32 != 0;
        frac128 = softfloat_shortShiftLeft128(
            0 as i32 as uint64_t,
            frac,
            49 as i32 as uint_fast8_t,
        );
        uiZ
            .v64 = ((sign as uint_fast64_t) << 63 as i32)
            .wrapping_add(exp << 48 as i32)
            .wrapping_add(frac128.v64);
        uiZ.v0 = frac128.v0;
    }
    uZ.ui = uiZ;
    return uZ.f;
}
