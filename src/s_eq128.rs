use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub unsafe fn softfloat_eq128(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut b64: uint64_t,
    mut b0: uint64_t,
) -> bool {
    return a64 == b64 && a0 == b0;
}
