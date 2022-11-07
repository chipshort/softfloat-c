use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type uint_fast8_t = u8;
pub unsafe fn softfloat_add256M(
    mut aPtr: *const uint64_t,
    mut bPtr: *const uint64_t,
    mut zPtr: *mut uint64_t,
) {
    let mut index: u32 = 0;
    let mut carry: uint_fast8_t = 0;
    let mut wordA: uint64_t = 0;
    let mut wordZ: uint64_t = 0;
    index = 0 as i32 as u32;
    carry = 0 as i32 as uint_fast8_t;
    loop {
        wordA = *aPtr.offset(index as isize);
        wordZ = wordA
            .wrapping_add(*bPtr.offset(index as isize))
            .wrapping_add(carry as u64);
        *zPtr.offset(index as isize) = wordZ;
        if index == (4 as i32 - 1 as i32) as u32 {
            break;
        }
        if wordZ != wordA {
            carry = (wordZ < wordA) as i32 as uint_fast8_t;
        }
        index = index.wrapping_add(1 as i32 as u32);
    };
}
