use crate::*;

pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type int_fast16_t = i64;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;


#[derive(Copy, Clone)]
#[repr(C)]
pub union ui32_f32 {
    pub ui: uint32_t,
    pub f: float32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union ui64_f64 {
    pub ui: uint64_t,
    pub f: float64_t,
}


pub unsafe fn f32_to_f64(mut a: float32_t) -> float64_t {
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
    let mut uiZ: uint_fast64_t = 0;
    let mut normExpSig: exp16_sig32 = exp16_sig32 { exp: 0, sig: 0 };
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    uA.f = a;
    uiA = uA.ui as uint_fast32_t;
    sign = uiA as uint32_t >> 31 as i32 != 0;
    exp = (uiA >> 23 as i32) as int_fast16_t
        & 0xff as i32 as i64;
    frac = uiA & 0x7fffff as i32 as u64;
    if exp == 0xff as i32 as i64 {
        if frac != 0 {
            softfloat_f32UIToCommonNaN(uiA, &mut commonNaN);
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
                current_block = 3998581016991163533;
            } else {
                normExpSig = softfloat_normSubnormalF32Sig(frac);
                exp = normExpSig.exp - 1 as i32 as i64;
                frac = normExpSig.sig;
                current_block = 17833034027772472439;
            }
        } else {
            current_block = 17833034027772472439;
        }
        match current_block {
            3998581016991163533 => {}
            _ => {
                uiZ = ((sign as uint_fast64_t) << 63 as i32)
                    .wrapping_add(
                        ((exp + 0x380 as i32 as i64) as uint_fast64_t)
                            << 52 as i32,
                    )
                    .wrapping_add(frac << 29 as i32);
            }
        }
    }
    uZ.ui = uiZ;
    return uZ.f;
}
