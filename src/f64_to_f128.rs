use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast16_t = i64;
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
pub unsafe fn f64_to_f128(mut a: float64_t) -> float128_t {
    let mut current_block: u64;
    let mut uA: ui64_f64 = ui64_f64 { ui: 0 };
    let mut uiA: uint_fast64_t = 0;
    let mut sign: bool = false;
    let mut exp: int_fast16_t = 0;
    let mut frac: uint_fast64_t = 0;
    let mut commonNaN: commonNaN = commonNaN {
        sign: false,
        v0: 0,
        v64: 0,
    };
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut normExpSig: exp16_sig64 = exp16_sig64 { exp: 0, sig: 0 };
    let mut frac128: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    uA.f = a;
    uiA = uA.ui;
    sign = uiA >> 63 as i32 != 0;
    exp = (uiA >> 52 as i32) as int_fast16_t
        & 0x7ff as i32 as i64;
    frac = uiA & 0xfffffffffffff as u64;
    if exp == 0x7ff as i32 as i64 {
        if frac != 0 {
            softfloat_f64UIToCommonNaN(uiA, &mut commonNaN);
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
                current_block = 10635749917740166916;
            } else {
                normExpSig = softfloat_normSubnormalF64Sig(frac);
                exp = normExpSig.exp - 1 as i32 as i64;
                frac = normExpSig.sig;
                current_block = 13242334135786603907;
            }
        } else {
            current_block = 13242334135786603907;
        }
        match current_block {
            10635749917740166916 => {}
            _ => {
                frac128 = softfloat_shortShiftLeft128(
                    0 as i32 as uint64_t,
                    frac,
                    60 as i32 as uint_fast8_t,
                );
                uiZ
                    .v64 = ((sign as uint_fast64_t) << 63 as i32)
                    .wrapping_add(
                        ((exp + 0x3c00 as i32 as i64) as uint_fast64_t)
                            << 48 as i32,
                    )
                    .wrapping_add(frac128.v64);
                uiZ.v0 = frac128.v0;
            }
        }
    }
    uZ.ui = uiZ;
    return uZ.f;
}
