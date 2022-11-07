use crate::*;

pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type int_fast8_t = i8;
pub type uint_fast16_t = u64;
pub type uint_fast64_t = u64;



pub type extFloat80_t = extFloat80M;
#[derive(Copy, Clone)]
#[repr(C)]
pub union ui16_f16 {
    pub ui: uint16_t,
    pub f: float16_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s: extFloat80M,
    pub f: extFloat80_t,
}

pub unsafe fn f16_to_extF80(mut a: float16_t) -> extFloat80_t {
    let mut current_block: u64;
    let mut uA: ui16_f16 = ui16_f16 { ui: 0 };
    let mut uiA: uint_fast16_t = 0;
    let mut sign: bool = false;
    let mut exp: int_fast8_t = 0;
    let mut frac: uint_fast16_t = 0;
    let mut commonNaN: commonNaN = commonNaN {
        sign: false,
        v0: 0,
        v64: 0,
    };
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uiZ64: uint_fast16_t = 0;
    let mut uiZ0: uint_fast64_t = 0;
    let mut normExpSig: exp8_sig16 = exp8_sig16 { exp: 0, sig: 0 };
    let mut uZ: C2RustUnnamed = C2RustUnnamed {
        s: extFloat80M {
            signif: 0,
            signExp: 0,
        },
    };
    uA.f = a;
    uiA = uA.ui as uint_fast16_t;
    sign = uiA as uint16_t as i32 >> 15 as i32 != 0;
    exp = ((uiA >> 10 as i32) as int_fast8_t as i32
        & 0x1f as i32) as int_fast8_t;
    frac = uiA & 0x3ff as i32 as u64;
    if exp as i32 == 0x1f as i32 {
        if frac != 0 {
            softfloat_f16UIToCommonNaN(uiA, &mut commonNaN);
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
            if frac == 0 {
                uiZ64 = (sign as uint_fast16_t) << 15 as i32
                    | 0 as i32 as u64;
                uiZ0 = 0 as i32 as uint_fast64_t;
                current_block = 12163647228066880242;
            } else {
                normExpSig = softfloat_normSubnormalF16Sig(frac);
                exp = normExpSig.exp;
                frac = normExpSig.sig;
                current_block = 12147880666119273379;
            }
        } else {
            current_block = 12147880666119273379;
        }
        match current_block {
            12163647228066880242 => {}
            _ => {
                uiZ64 = (sign as uint_fast16_t) << 15 as i32
                    | (exp as i32 + 0x3ff0 as i32) as u64;
                uiZ0 = (frac | 0x400 as i32 as u64)
                    << 53 as i32;
            }
        }
    }
    uZ.s.signExp = uiZ64 as uint16_t;
    uZ.s.signif = uiZ0;
    return uZ.f;
}
