use crate::*;

pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type int_fast8_t = i8;
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


pub unsafe fn f16_to_f128(mut a: float16_t) -> float128_t {
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
    let mut normExpSig: exp8_sig16 = exp8_sig16 { exp: 0, sig: 0 };
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
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
            uiZ = softfloat_commonNaNToF128UI(&mut commonNaN);
        } else {
            uiZ
                .v64 = ((sign as uint_fast64_t) << 63 as i32)
                .wrapping_add(
                    (0x7fff as i32 as uint_fast64_t) << 48 as i32,
                )
                .wrapping_add(0 as i32 as u64);
            uiZ.v0 = 0 as i32 as uint64_t;
        }
    } else {
        if exp == 0 {
            if frac == 0 {
                uiZ
                    .v64 = ((sign as uint_fast64_t) << 63 as i32)
                    .wrapping_add(
                        (0 as i32 as uint_fast64_t) << 48 as i32,
                    )
                    .wrapping_add(0 as i32 as u64);
                uiZ.v0 = 0 as i32 as uint64_t;
                current_block = 10514614826165598180;
            } else {
                normExpSig = softfloat_normSubnormalF16Sig(frac);
                exp = (normExpSig.exp as i32 - 1 as i32) as int_fast8_t;
                frac = normExpSig.sig;
                current_block = 6057473163062296781;
            }
        } else {
            current_block = 6057473163062296781;
        }
        match current_block {
            10514614826165598180 => {}
            _ => {
                uiZ
                    .v64 = ((sign as uint_fast64_t) << 63 as i32)
                    .wrapping_add(
                        ((exp as i32 + 0x3ff0 as i32) as uint_fast64_t)
                            << 48 as i32,
                    )
                    .wrapping_add(frac << 38 as i32);
                uiZ.v0 = 0 as i32 as uint64_t;
            }
        }
    }
    uZ.ui = uiZ;
    return uZ.f;
}
