use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type uint_fast8_t = u8;
pub type uint_fast32_t = u64;
unsafe fn softfloat_shortShiftRightJamM(
    mut size_words: uint_fast8_t,
    mut aPtr: *const uint64_t,
    mut dist: uint_fast8_t,
    mut zPtr: *mut uint64_t,
) {
    let mut uNegDist: uint_fast8_t = 0;
    let mut index: u32 = 0;
    let mut lastIndex: u32 = 0;
    let mut partWordZ: uint64_t = 0;
    let mut wordA: uint64_t = 0;
    uNegDist = -(dist as i32) as uint_fast8_t;
    index = 0 as i32 as u32;
    lastIndex = (size_words as i32 - 1 as i32) as u32;
    wordA = *aPtr.offset(index as isize);
    partWordZ = wordA >> dist as i32;
    if partWordZ << dist as i32 != wordA {
        partWordZ |= 1 as i32 as u64;
    }
    while index != lastIndex {
        wordA = *aPtr
            .offset(index.wrapping_add(1 as i32 as u32) as isize);
        *zPtr
            .offset(
                index as isize,
            ) = wordA << (uNegDist as i32 & 63 as i32) | partWordZ;
        index = index.wrapping_add(1 as i32 as u32);
        partWordZ = wordA >> dist as i32;
    }
    *zPtr.offset(index as isize) = partWordZ;
}
pub unsafe fn softfloat_shiftRightJam256M(
    mut aPtr: *const uint64_t,
    mut dist: uint_fast32_t,
    mut zPtr: *mut uint64_t,
) {
    let mut current_block: u64;
    let mut wordJam: uint64_t = 0;
    let mut wordDist: uint_fast32_t = 0;
    let mut ptr: *mut uint64_t = 0 as *mut uint64_t;
    let mut i: uint_fast8_t = 0;
    let mut innerDist: uint_fast8_t = 0;
    wordJam = 0 as i32 as uint64_t;
    wordDist = dist >> 6 as i32;
    if wordDist != 0 {
        if (4 as i32 as u64) < wordDist {
            wordDist = 4 as i32 as uint_fast32_t;
        }
        ptr = aPtr.offset(0 as i32 as isize) as *mut uint64_t;
        i = wordDist as uint_fast8_t;
        loop {
            let fresh0 = ptr;
            ptr = ptr.offset(1);
            wordJam = *fresh0;
            if wordJam != 0 {
                break;
            }
            i = i.wrapping_sub(1);
            if !(i != 0) {
                break;
            }
        }
        ptr = zPtr;
    }
    if wordDist < 4 as i32 as u64 {
        aPtr = aPtr.offset(wordDist as isize);
        innerDist = (dist & 63 as i32 as u64) as uint_fast8_t;
        if innerDist != 0 {
            softfloat_shortShiftRightJamM(
                (4 as i32 as u64).wrapping_sub(wordDist)
                    as uint_fast8_t,
                aPtr,
                innerDist,
                zPtr.offset(0 as i32 as isize),
            );
            if wordDist == 0 {
                current_block = 5731960936215835141;
            } else {
                current_block = 11298138898191919651;
            }
        } else {
            aPtr = aPtr.offset(0 as i32 as isize);
            ptr = zPtr.offset(0 as i32 as isize);
            i = (4 as i32 as u64).wrapping_sub(wordDist)
                as uint_fast8_t;
            while i != 0 {
                *ptr = *aPtr;
                aPtr = aPtr.offset(1 as i32 as isize);
                ptr = ptr.offset(1 as i32 as isize);
                i = i.wrapping_sub(1);
            }
            current_block = 11298138898191919651;
        }
        match current_block {
            5731960936215835141 => {}
            _ => {
                ptr = zPtr
                    .offset(
                        (4 as i32 as u64).wrapping_sub(wordDist)
                            as isize,
                    );
                current_block = 15768484401365413375;
            }
        }
    } else {
        current_block = 15768484401365413375;
    }
    match current_block {
        15768484401365413375 => {
            loop {
                let fresh1 = ptr;
                ptr = ptr.offset(1);
                *fresh1 = 0 as i32 as uint64_t;
                wordDist = wordDist.wrapping_sub(1);
                if !(wordDist != 0) {
                    break;
                }
            }
        }
        _ => {}
    }
    if wordJam != 0 {
        let ref mut fresh2 = *zPtr.offset(0 as i32 as isize);
        *fresh2 |= 1 as i32 as u64;
    }
}
