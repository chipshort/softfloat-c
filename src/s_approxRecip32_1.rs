use crate::*;

pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;
pub unsafe fn softfloat_approxRecip32_1(mut a: uint32_t) -> uint32_t {
    let mut index: i32 = 0;
    let mut eps: uint16_t = 0;
    let mut r0: uint16_t = 0;
    let mut sigma0: uint32_t = 0;
    let mut r: uint_fast32_t = 0;
    let mut sqrSigma0: uint32_t = 0;
    index = (a >> 27 as i32 & 0xf as i32 as u32) as i32;
    eps = (a >> 11 as i32) as uint16_t;
    r0 = (softfloat_approxRecip_1k0s[index as usize] as u64)
        .wrapping_sub(
            (softfloat_approxRecip_1k1s[index as usize] as u64)
                .wrapping_mul(eps as uint_fast32_t) >> 20 as i32,
        ) as uint16_t;
    sigma0 = !((r0 as u64).wrapping_mul(a as uint_fast64_t)
        >> 7 as i32) as uint32_t;
    r = ((r0 as uint_fast32_t) << 16 as i32)
        .wrapping_add(
            (r0 as u64).wrapping_mul(sigma0 as uint_fast64_t)
                >> 24 as i32,
        );
    sqrSigma0 = ((sigma0 as uint_fast64_t).wrapping_mul(sigma0 as u64)
        >> 32 as i32) as uint32_t;
    r = (r as u64)
        .wrapping_add(
            (r as uint32_t as u64).wrapping_mul(sqrSigma0 as uint_fast64_t)
                >> 48 as i32,
        ) as uint_fast32_t as uint_fast32_t;
    return r as uint32_t;
}
