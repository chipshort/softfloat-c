use crate::*;

pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type int_fast32_t = i64;
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
pub unsafe fn f128_to_extF80(mut a: float128_t) -> extFloat80_t {
    let mut current_block: u64;
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
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uiZ64: uint_fast16_t = 0;
    let mut uiZ0: uint_fast64_t = 0;
    let mut normExpSig: exp32_sig128 = exp32_sig128 {
        exp: 0,
        sig: uint128 { v0: 0, v64: 0 },
    };
    let mut sig128: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uZ: C2RustUnnamed = C2RustUnnamed {
        s: extFloat80M {
            signif: 0,
            signExp: 0,
        },
    };
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
            uiZ = softfloat_commonNaNToExtF80UI(&mut commonNaN);
            uiZ64 = uiZ.v64;
            uiZ0 = uiZ.v0;
        } else {
            uiZ64 = (sign as uint_fast16_t) << 15 as i32
                | 0x7fff as i32 as u64;
            uiZ0 = 0x8000000000000000 as u64;
        }
    } else {
        if exp == 0 {
            if frac64 | frac0 == 0 {
                uiZ64 = (sign as uint_fast16_t) << 15 as i32
                    | 0 as i32 as u64;
                uiZ0 = 0 as i32 as uint_fast64_t;
                current_block = 13869584887439955522;
            } else {
                normExpSig = softfloat_normSubnormalF128Sig(frac64, frac0);
                exp = normExpSig.exp;
                frac64 = normExpSig.sig.v64;
                frac0 = normExpSig.sig.v0;
                current_block = 14401909646449704462;
            }
        } else {
            current_block = 14401909646449704462;
        }
        match current_block {
            13869584887439955522 => {}
            _ => {
                sig128 = softfloat_shortShiftLeft128(
                    frac64 | 0x1000000000000 as u64,
                    frac0,
                    15 as i32 as uint_fast8_t,
                );
                return softfloat_roundPackToExtF80(
                    sign,
                    exp,
                    sig128.v64,
                    sig128.v0,
                    80 as i32 as uint_fast8_t,
                );
            }
        }
    }
    uZ.s.signExp = uiZ64 as uint16_t;
    uZ.s.signif = uiZ0;
    return uZ.f;
}
