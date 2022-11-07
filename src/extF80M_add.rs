use crate::*;

pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type uint_fast16_t = u64;
pub type uint_fast64_t = u64;

pub type extFloat80_t = extFloat80M;
pub unsafe fn extF80M_add(
    mut aPtr: *const extFloat80_t,
    mut bPtr: *const extFloat80_t,
    mut zPtr: *mut extFloat80_t,
) {
    let mut aSPtr: *const extFloat80M = 0 as *const extFloat80M;
    let mut bSPtr: *const extFloat80M = 0 as *const extFloat80M;
    let mut uiA64: uint_fast16_t = 0;
    let mut uiA0: uint_fast64_t = 0;
    let mut signA: bool = false;
    let mut uiB64: uint_fast16_t = 0;
    let mut uiB0: uint_fast64_t = 0;
    let mut signB: bool = false;
    aSPtr = aPtr as *const extFloat80M;
    bSPtr = bPtr as *const extFloat80M;
    uiA64 = (*aSPtr).signExp as uint_fast16_t;
    uiA0 = (*aSPtr).signif;
    signA = uiA64 as uint16_t as i32 >> 15 as i32 != 0;
    uiB64 = (*bSPtr).signExp as uint_fast16_t;
    uiB0 = (*bSPtr).signif;
    signB = uiB64 as uint16_t as i32 >> 15 as i32 != 0;
    if signA as i32 == signB as i32 {
        *zPtr = softfloat_addMagsExtF80(uiA64, uiA0, uiB64, uiB0, signA);
    } else {
        *zPtr = softfloat_subMagsExtF80(uiA64, uiA0, uiB64, uiB0, signA);
    };
}
