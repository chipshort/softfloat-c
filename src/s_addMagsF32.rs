use crate::*;

pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
pub type int_fast16_t = i64;
pub type uint_fast16_t = u64;
pub type uint_fast32_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui32_f32 {
    pub ui: uint32_t,
    pub f: float32_t,
}
#[inline]
unsafe fn softfloat_shiftRightJam32(
    mut a: uint32_t,
    mut dist: uint_fast16_t,
) -> uint32_t {
    return if dist < 31 as i32 as u64 {
        a >> dist
            | (a << (dist.wrapping_neg() & 31 as i32 as u64)
                != 0 as i32 as u32) as i32 as u32
    } else {
        (a != 0 as i32 as u32) as i32 as u32
    };
}
pub unsafe fn softfloat_addMagsF32(
    mut uiA: uint_fast32_t,
    mut uiB: uint_fast32_t,
) -> float32_t {
    let mut current_block: u64;
    let mut expA: int_fast16_t = 0;
    let mut sigA: uint_fast32_t = 0;
    let mut expB: int_fast16_t = 0;
    let mut sigB: uint_fast32_t = 0;
    let mut expDiff: int_fast16_t = 0;
    let mut uiZ: uint_fast32_t = 0;
    let mut signZ: bool = false;
    let mut expZ: int_fast16_t = 0;
    let mut sigZ: uint_fast32_t = 0;
    let mut uZ: ui32_f32 = ui32_f32 { ui: 0 };
    expA = (uiA >> 23 as i32) as int_fast16_t
        & 0xff as i32 as i64;
    sigA = uiA & 0x7fffff as i32 as u64;
    expB = (uiB >> 23 as i32) as int_fast16_t
        & 0xff as i32 as i64;
    sigB = uiB & 0x7fffff as i32 as u64;
    expDiff = expA - expB;
    if expDiff == 0 {
        if expA == 0 {
            uiZ = uiA.wrapping_add(sigB);
            current_block = 12680572919044654648;
        } else if expA == 0xff as i32 as i64 {
            if sigA | sigB != 0 {
                current_block = 11501164521559807994;
            } else {
                uiZ = uiA;
                current_block = 12680572919044654648;
            }
        } else {
            signZ = uiA as uint32_t >> 31 as i32 != 0;
            expZ = expA;
            sigZ = (0x1000000 as i32 as u64)
                .wrapping_add(sigA)
                .wrapping_add(sigB);
            if sigZ & 1 as i32 as u64 == 0
                && expZ < 0xfe as i32 as i64
            {
                uiZ = (((signZ as uint32_t) << 31 as i32)
                    .wrapping_add((expZ as uint32_t) << 23 as i32)
                    as u64)
                    .wrapping_add(sigZ >> 1 as i32);
                current_block = 12680572919044654648;
            } else {
                sigZ <<= 6 as i32;
                current_block = 7828949454673616476;
            }
        }
    } else {
        signZ = uiA as uint32_t >> 31 as i32 != 0;
        sigA <<= 6 as i32;
        sigB <<= 6 as i32;
        if expDiff < 0 as i32 as i64 {
            if expB == 0xff as i32 as i64 {
                if sigB != 0 {
                    current_block = 11501164521559807994;
                } else {
                    uiZ = ((signZ as uint32_t) << 31 as i32)
                        .wrapping_add(
                            (0xff as i32 as uint32_t) << 23 as i32,
                        )
                        .wrapping_add(0 as i32 as u32) as uint_fast32_t;
                    current_block = 12680572919044654648;
                }
            } else {
                expZ = expB;
                sigA = (sigA as u64)
                    .wrapping_add(
                        if expA != 0 {
                            0x20000000 as i32 as u64
                        } else {
                            sigA
                        },
                    ) as uint_fast32_t as uint_fast32_t;
                sigA = softfloat_shiftRightJam32(
                    sigA as uint32_t,
                    -expDiff as uint_fast16_t,
                ) as uint_fast32_t;
                current_block = 2569451025026770673;
            }
        } else if expA == 0xff as i32 as i64 {
            if sigA != 0 {
                current_block = 11501164521559807994;
            } else {
                uiZ = uiA;
                current_block = 12680572919044654648;
            }
        } else {
            expZ = expA;
            sigB = (sigB as u64)
                .wrapping_add(
                    if expB != 0 {
                        0x20000000 as i32 as u64
                    } else {
                        sigB
                    },
                ) as uint_fast32_t as uint_fast32_t;
            sigB = softfloat_shiftRightJam32(sigB as uint32_t, expDiff as uint_fast16_t)
                as uint_fast32_t;
            current_block = 2569451025026770673;
        }
        match current_block {
            12680572919044654648 => {}
            11501164521559807994 => {}
            _ => {
                sigZ = (0x20000000 as i32 as u64)
                    .wrapping_add(sigA)
                    .wrapping_add(sigB);
                if sigZ < 0x40000000 as i32 as u64 {
                    expZ -= 1;
                    sigZ <<= 1 as i32;
                }
                current_block = 7828949454673616476;
            }
        }
    }
    match current_block {
        7828949454673616476 => return softfloat_roundPackToF32(signZ, expZ, sigZ),
        11501164521559807994 => {
            uiZ = softfloat_propagateNaNF32UI(uiA, uiB);
        }
        _ => {}
    }
    uZ.ui = uiZ as uint32_t;
    return uZ.f;
}
