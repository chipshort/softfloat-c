use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type uint_fast8_t = u8;
pub unsafe fn softfloat_sub256M(
    mut aPtr: *const uint64_t,
    mut bPtr: *const uint64_t,
    mut zPtr: *mut uint64_t,
) {
    let mut index: u32 = 0;
    let mut borrow: uint_fast8_t = 0;
    let mut wordA: uint64_t = 0;
    let mut wordB: uint64_t = 0;
    index = 0 as i32 as u32;
    borrow = 0 as i32 as uint_fast8_t;
    loop {
        wordA = *aPtr.offset(index as isize);
        wordB = *bPtr.offset(index as isize);
        *zPtr
            .offset(
                index as isize,
            ) = wordA.wrapping_sub(wordB).wrapping_sub(borrow as u64);
        if index == (4 as i32 - 1 as i32) as u32 {
            break;
        }
        borrow = (if borrow as i32 != 0 {
            (wordA <= wordB) as i32
        } else {
            (wordA < wordB) as i32
        }) as uint_fast8_t;
        index = index.wrapping_add(1 as i32 as u32);
    };
}
