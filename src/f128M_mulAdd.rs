use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type uint_fast8_t = u8;
pub type uint_fast64_t = u64;

pub unsafe fn f128M_mulAdd(
    mut aPtr: *const float128_t,
    mut bPtr: *const float128_t,
    mut cPtr: *const float128_t,
    mut zPtr: *mut float128_t,
) {
    let mut aWPtr: *const uint64_t = 0 as *const uint64_t;
    let mut bWPtr: *const uint64_t = 0 as *const uint64_t;
    let mut cWPtr: *const uint64_t = 0 as *const uint64_t;
    let mut uiA64: uint_fast64_t = 0;
    let mut uiA0: uint_fast64_t = 0;
    let mut uiB64: uint_fast64_t = 0;
    let mut uiB0: uint_fast64_t = 0;
    let mut uiC64: uint_fast64_t = 0;
    let mut uiC0: uint_fast64_t = 0;
    aWPtr = aPtr as *const uint64_t;
    bWPtr = bPtr as *const uint64_t;
    cWPtr = cPtr as *const uint64_t;
    uiA64 = *aWPtr.offset(1 as i32 as isize);
    uiA0 = *aWPtr.offset(0 as i32 as isize);
    uiB64 = *bWPtr.offset(1 as i32 as isize);
    uiB0 = *bWPtr.offset(0 as i32 as isize);
    uiC64 = *cWPtr.offset(1 as i32 as isize);
    uiC0 = *cWPtr.offset(0 as i32 as isize);
    *zPtr = softfloat_mulAddF128(
        uiA64,
        uiA0,
        uiB64,
        uiB0,
        uiC64,
        uiC0,
        0 as i32 as uint_fast8_t,
    );
}
