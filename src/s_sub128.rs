use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;

pub unsafe fn softfloat_sub128(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut b64: uint64_t,
    mut b0: uint64_t,
) -> uint128 {
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    z.v0 = a0.wrapping_sub(b0);
    z
        .v64 = a64
        .wrapping_sub(b64)
        .wrapping_sub((a0 < b0) as i32 as u64);
    return z;
}
