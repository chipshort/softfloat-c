use crate::*;

pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type uint_fast16_t = u64;
pub type uint_fast64_t = u64;

pub type extFloat80_t = extFloat80M;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s: extFloat80M,
    pub f: extFloat80_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub s: extFloat80M,
    pub f: extFloat80_t,
}
pub unsafe fn extF80_sub(
    mut a: extFloat80_t,
    mut b: extFloat80_t,
) -> extFloat80_t {
    let mut uA: C2RustUnnamed_0 = C2RustUnnamed_0 {
        s: extFloat80_t {
            signif: 0,
            signExp: 0,
        },
    };
    let mut uiA64: uint_fast16_t = 0;
    let mut uiA0: uint_fast64_t = 0;
    let mut signA: bool = false;
    let mut uB: C2RustUnnamed = C2RustUnnamed {
        s: extFloat80_t {
            signif: 0,
            signExp: 0,
        },
    };
    let mut uiB64: uint_fast16_t = 0;
    let mut uiB0: uint_fast64_t = 0;
    let mut signB: bool = false;
    uA.f = a;
    uiA64 = uA.s.signExp as uint_fast16_t;
    uiA0 = uA.s.signif;
    signA = uiA64 as uint16_t as i32 >> 15 as i32 != 0;
    uB.f = b;
    uiB64 = uB.s.signExp as uint_fast16_t;
    uiB0 = uB.s.signif;
    signB = uiB64 as uint16_t as i32 >> 15 as i32 != 0;
    if signA as i32 == signB as i32 {
        return softfloat_subMagsExtF80(uiA64, uiA0, uiB64, uiB0, signA)
    } else {
        return softfloat_addMagsExtF80(uiA64, uiA0, uiB64, uiB0, signA)
    };
}
