use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type uint_fast64_t = u64;

pub unsafe fn f128M_sub(
    mut aPtr: *const float128_t,
    mut bPtr: *const float128_t,
    mut zPtr: *mut float128_t,
) {
    let mut aWPtr: *const uint64_t = 0 as *const uint64_t;
    let mut bWPtr: *const uint64_t = 0 as *const uint64_t;
    let mut uiA64: uint_fast64_t = 0;
    let mut uiA0: uint_fast64_t = 0;
    let mut signA: bool = false;
    let mut uiB64: uint_fast64_t = 0;
    let mut uiB0: uint_fast64_t = 0;
    let mut signB: bool = false;
    aWPtr = aPtr as *const uint64_t;
    bWPtr = bPtr as *const uint64_t;
    uiA64 = *aWPtr.offset(1 as i32 as isize);
    uiA0 = *aWPtr.offset(0 as i32 as isize);
    signA = uiA64 >> 63 as i32 != 0;
    uiB64 = *bWPtr.offset(1 as i32 as isize);
    uiB0 = *bWPtr.offset(0 as i32 as isize);
    signB = uiB64 >> 63 as i32 != 0;
    if signA as i32 == signB as i32 {
        *zPtr = softfloat_subMagsF128(uiA64, uiA0, uiB64, uiB0, signA);
    } else {
        *zPtr = softfloat_addMagsF128(uiA64, uiA0, uiB64, uiB0, signA);
    };
}
