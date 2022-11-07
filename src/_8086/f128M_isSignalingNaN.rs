use crate::*;

pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;

pub unsafe fn f128M_isSignalingNaN(mut aPtr: *const float128_t) -> bool {
    let mut aWPtr: *const uint32_t = 0 as *const uint32_t;
    let mut uiA96: uint32_t = 0;
    aWPtr = aPtr as *const uint32_t;
    uiA96 = *aWPtr.offset((4 as i32 - 1 as i32) as isize);
    if uiA96 & 0x7fff8000 as i32 as u32
        != 0x7fff0000 as i32 as u32
    {
        return 0 as i32 != 0;
    }
    return uiA96 & 0x7fff as i32 as u32
        != 0 as i32 as u32
        || *aWPtr.offset(2 as i32 as isize)
            | *aWPtr.offset(1 as i32 as isize)
            | *aWPtr.offset(0 as i32 as isize)
            != 0 as i32 as u32;
}
