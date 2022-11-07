use crate::*;

pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type int_fast16_t = i64;
pub type uint_fast16_t = u64;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;



pub type extFloat80_t = extFloat80M;
#[derive(Copy, Clone)]
#[repr(C)]
pub union ui32_f32 {
    pub ui: uint32_t,
    pub f: float32_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s: extFloat80M,
    pub f: extFloat80_t,
}

pub unsafe fn f32_to_extF80(mut a: float32_t) -> extFloat80_t {
    let mut current_block: u64;
    let mut uA: ui32_f32 = ui32_f32 { ui: 0 };
    let mut uiA: uint_fast32_t = 0;
    let mut sign: bool = false;
    let mut exp: int_fast16_t = 0;
    let mut frac: uint_fast32_t = 0;
    let mut commonNaN: commonNaN = commonNaN {
        sign: false,
        v0: 0,
        v64: 0,
    };
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uiZ64: uint_fast16_t = 0;
    let mut uiZ0: uint_fast64_t = 0;
    let mut normExpSig: exp16_sig32 = exp16_sig32 { exp: 0, sig: 0 };
    let mut uZ: C2RustUnnamed = C2RustUnnamed {
        s: extFloat80M {
            signif: 0,
            signExp: 0,
        },
    };
    uA.f = a;
    uiA = uA.ui as uint_fast32_t;
    sign = uiA as uint32_t >> 31 as i32 != 0;
    exp = (uiA >> 23 as i32) as int_fast16_t
        & 0xff as i32 as i64;
    frac = uiA & 0x7fffff as i32 as u64;
    if exp == 0xff as i32 as i64 {
        if frac != 0 {
            softfloat_f32UIToCommonNaN(uiA, &mut commonNaN);
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
                current_block = 11472363453916299741;
            } else {
                normExpSig = softfloat_normSubnormalF32Sig(frac);
                exp = normExpSig.exp;
                frac = normExpSig.sig;
                current_block = 12147880666119273379;
            }
        } else {
            current_block = 12147880666119273379;
        }
        match current_block {
            11472363453916299741 => {}
            _ => {
                uiZ64 = (sign as uint_fast16_t) << 15 as i32
                    | (exp + 0x3f80 as i32 as i64) as u64;
                uiZ0 = (frac | 0x800000 as i32 as u64)
                    << 40 as i32;
            }
        }
    }
    uZ.s.signExp = uiZ64 as uint16_t;
    uZ.s.signif = uiZ0;
    return uZ.f;
}
