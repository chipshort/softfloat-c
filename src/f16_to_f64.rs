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
pub union ui64_f64 {
    pub ui: uint64_t,
    pub f: float64_t,
}


pub unsafe fn f16_to_f64(mut a: float16_t) -> float64_t {
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
    let mut uiZ: uint_fast64_t = 0;
    let mut normExpSig: exp8_sig16 = exp8_sig16 { exp: 0, sig: 0 };
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    uA.f = a;
    uiA = uA.ui as uint_fast16_t;
    sign = uiA as uint16_t as i32 >> 15 as i32 != 0;
    exp = ((uiA >> 10 as i32) as int_fast8_t as i32
        & 0x1f as i32) as int_fast8_t;
    frac = uiA & 0x3ff as i32 as u64;
    if exp as i32 == 0x1f as i32 {
        if frac != 0 {
            softfloat_f16UIToCommonNaN(uiA, &mut commonNaN);
            uiZ = softfloat_commonNaNToF64UI(&mut commonNaN);
        } else {
            uiZ = ((sign as uint_fast64_t) << 63 as i32)
                .wrapping_add(
                    (0x7ff as i32 as uint_fast64_t) << 52 as i32,
                )
                .wrapping_add(0 as i32 as u64);
        }
    } else {
        if exp == 0 {
            if frac == 0 {
                uiZ = ((sign as uint_fast64_t) << 63 as i32)
                    .wrapping_add(
                        (0 as i32 as uint_fast64_t) << 52 as i32,
                    )
                    .wrapping_add(0 as i32 as u64);
                current_block = 12135327198841022002;
            } else {
                normExpSig = softfloat_normSubnormalF16Sig(frac);
                exp = (normExpSig.exp as i32 - 1 as i32) as int_fast8_t;
                frac = normExpSig.sig;
                current_block = 17833034027772472439;
            }
        } else {
            current_block = 17833034027772472439;
        }
        match current_block {
            12135327198841022002 => {}
            _ => {
                uiZ = ((sign as uint_fast64_t) << 63 as i32)
                    .wrapping_add(
                        ((exp as i32 + 0x3f0 as i32) as uint_fast64_t)
                            << 52 as i32,
                    )
                    .wrapping_add(frac << 42 as i32);
            }
        }
    }
    uZ.ui = uiZ;
    return uZ.f;
}
