use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast16_t = i64;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui64_f64 {
    pub ui: uint64_t,
    pub f: float64_t,
}
#[inline]
unsafe fn softfloat_shiftRightJam64(
    mut a: uint64_t,
    mut dist: uint_fast32_t,
) -> uint64_t {
    return if dist < 63 as i32 as u64 {
        a >> dist
            | (a << (dist.wrapping_neg() & 63 as i32 as u64)
                != 0 as i32 as u64) as i32 as u64
    } else {
        (a != 0 as i32 as u64) as i32 as u64
    };
}
pub unsafe fn softfloat_addMagsF64(
    mut uiA: uint_fast64_t,
    mut uiB: uint_fast64_t,
    mut signZ: bool,
) -> float64_t {
    let mut current_block: u64;
    let mut expA: int_fast16_t = 0;
    let mut sigA: uint_fast64_t = 0;
    let mut expB: int_fast16_t = 0;
    let mut sigB: uint_fast64_t = 0;
    let mut expDiff: int_fast16_t = 0;
    let mut uiZ: uint_fast64_t = 0;
    let mut expZ: int_fast16_t = 0;
    let mut sigZ: uint_fast64_t = 0;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    expA = (uiA >> 52 as i32) as int_fast16_t
        & 0x7ff as i32 as i64;
    sigA = uiA & 0xfffffffffffff as u64;
    expB = (uiB >> 52 as i32) as int_fast16_t
        & 0x7ff as i32 as i64;
    sigB = uiB & 0xfffffffffffff as u64;
    expDiff = expA - expB;
    if expDiff == 0 {
        if expA == 0 {
            uiZ = uiA.wrapping_add(sigB);
            current_block = 17292490315990239299;
        } else if expA == 0x7ff as i32 as i64 {
            if sigA | sigB != 0 {
                current_block = 6317110829061826235;
            } else {
                uiZ = uiA;
                current_block = 17292490315990239299;
            }
        } else {
            expZ = expA;
            sigZ = (0x20000000000000 as u64)
                .wrapping_add(sigA)
                .wrapping_add(sigB);
            sigZ <<= 9 as i32;
            current_block = 7746103178988627676;
        }
    } else {
        sigA <<= 9 as i32;
        sigB <<= 9 as i32;
        if expDiff < 0 as i32 as i64 {
            if expB == 0x7ff as i32 as i64 {
                if sigB != 0 {
                    current_block = 6317110829061826235;
                } else {
                    uiZ = ((signZ as uint_fast64_t) << 63 as i32)
                        .wrapping_add(
                            (0x7ff as i32 as uint_fast64_t) << 52 as i32,
                        )
                        .wrapping_add(0 as i32 as u64);
                    current_block = 17292490315990239299;
                }
            } else {
                expZ = expB;
                if expA != 0 {
                    sigA = (sigA as u64)
                        .wrapping_add(0x2000000000000000 as u64)
                        as uint_fast64_t as uint_fast64_t;
                } else {
                    sigA <<= 1 as i32;
                }
                sigA = softfloat_shiftRightJam64(sigA, -expDiff as uint_fast32_t);
                current_block = 9853141518545631134;
            }
        } else if expA == 0x7ff as i32 as i64 {
            if sigA != 0 {
                current_block = 6317110829061826235;
            } else {
                uiZ = uiA;
                current_block = 17292490315990239299;
            }
        } else {
            expZ = expA;
            if expB != 0 {
                sigB = (sigB as u64)
                    .wrapping_add(0x2000000000000000 as u64) as uint_fast64_t
                    as uint_fast64_t;
            } else {
                sigB <<= 1 as i32;
            }
            sigB = softfloat_shiftRightJam64(sigB, expDiff as uint_fast32_t);
            current_block = 9853141518545631134;
        }
        match current_block {
            17292490315990239299 => {}
            6317110829061826235 => {}
            _ => {
                sigZ = (0x2000000000000000 as u64)
                    .wrapping_add(sigA)
                    .wrapping_add(sigB);
                if sigZ < 0x4000000000000000 as u64 {
                    expZ -= 1;
                    sigZ <<= 1 as i32;
                }
                current_block = 7746103178988627676;
            }
        }
    }
    match current_block {
        7746103178988627676 => return softfloat_roundPackToF64(signZ, expZ, sigZ),
        6317110829061826235 => {
            uiZ = softfloat_propagateNaNF64UI(uiA, uiB);
        }
        _ => {}
    }
    uZ.ui = uiZ;
    return uZ.f;
}
