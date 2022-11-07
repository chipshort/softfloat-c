use crate::*;

pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;

pub type extFloat80_t = extFloat80M;
pub unsafe fn extF80M_isSignalingNaN(mut aPtr: *const extFloat80_t) -> bool {
    let mut aSPtr: *const extFloat80M = 0 as *const extFloat80M;
    let mut uiA0: uint64_t = 0;
    aSPtr = aPtr as *const extFloat80M;
    if (*aSPtr).signExp as i32 & 0x7fff as i32 != 0x7fff as i32 {
        return 0 as i32 != 0;
    }
    uiA0 = (*aSPtr).signif;
    return uiA0 & 0x4000000000000000 as u64 == 0
        && uiA0 & 0x3fffffffffffffff as u64 != 0;
}
