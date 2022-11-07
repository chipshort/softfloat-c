use crate::*;

pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;
pub unsafe fn softfloat_approxRecipSqrt32_1(
    mut oddExpA: u32,
    mut a: uint32_t,
) -> uint32_t {
    let mut index: i32 = 0;
    let mut eps: uint16_t = 0;
    let mut r0: uint16_t = 0;
    let mut ESqrR0: uint_fast32_t = 0;
    let mut sigma0: uint32_t = 0;
    let mut r: uint_fast32_t = 0;
    let mut sqrSigma0: uint32_t = 0;
    index = (a >> 27 as i32 & 0xe as i32 as u32)
        .wrapping_add(oddExpA) as i32;
    eps = (a >> 12 as i32) as uint16_t;
    r0 = (*softfloat_approxRecipSqrt_1k0s.as_ptr().offset(index as isize)
        as u64)
        .wrapping_sub(
            (*softfloat_approxRecipSqrt_1k1s.as_ptr().offset(index as isize)
                as u64)
                .wrapping_mul(eps as uint_fast32_t) >> 20 as i32,
        ) as uint16_t;
    ESqrR0 = (r0 as uint_fast32_t).wrapping_mul(r0 as u64);
    if oddExpA == 0 {
        ESqrR0 <<= 1 as i32;
    }
    sigma0 = !((ESqrR0 as uint32_t as u64).wrapping_mul(a as uint_fast64_t)
        >> 23 as i32) as uint32_t;
    r = ((r0 as uint_fast32_t) << 16 as i32)
        .wrapping_add(
            (r0 as u64).wrapping_mul(sigma0 as uint_fast64_t)
                >> 25 as i32,
        );
    sqrSigma0 = ((sigma0 as uint_fast64_t).wrapping_mul(sigma0 as u64)
        >> 32 as i32) as uint32_t;
    r = (r as u64)
        .wrapping_add(
            ((r >> 1 as i32)
                .wrapping_add(r >> 3 as i32)
                .wrapping_sub((r0 as uint_fast32_t) << 14 as i32) as uint32_t
                as u64)
                .wrapping_mul(sqrSigma0 as uint_fast64_t) >> 48 as i32,
        ) as uint_fast32_t as uint_fast32_t;
    if r & 0x80000000 as u32 as u64 == 0 {
        r = 0x80000000 as u32 as uint_fast32_t;
    }
    return r as uint32_t;
}
