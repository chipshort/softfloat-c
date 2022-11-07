use crate::*;

pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;

pub type extFloat80_t = extFloat80M;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s: extFloat80M,
    pub f: extFloat80_t,
}
pub unsafe fn extF80_isSignalingNaN(mut a: extFloat80_t) -> bool {
    let mut uA: C2RustUnnamed = C2RustUnnamed {
        s: extFloat80_t {
            signif: 0,
            signExp: 0,
        },
    };
    uA.f = a;
    return uA.s.signExp as i32 & 0x7fff as i32 == 0x7fff as i32
        && uA.s.signif & 0x4000000000000000 as u64 == 0
        && uA.s.signif & 0x3fffffffffffffff as u64 != 0;
}
