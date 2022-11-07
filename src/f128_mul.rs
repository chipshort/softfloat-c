use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast32_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast64_t = u64;



#[derive(Copy, Clone)]
#[repr(C)]
pub union ui128_f128 {
    pub ui: uint128,
    pub f: float128_t,
}

pub type C2RustUnnamed = u32;
pub const softfloat_flag_invalid: C2RustUnnamed = 16;
pub const softfloat_flag_infinite: C2RustUnnamed = 8;
pub const softfloat_flag_overflow: C2RustUnnamed = 4;
pub const softfloat_flag_underflow: C2RustUnnamed = 2;
pub const softfloat_flag_inexact: C2RustUnnamed = 1;
#[inline]
unsafe fn softfloat_mul128To256M(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut b64: uint64_t,
    mut b0: uint64_t,
    mut zPtr: *mut uint64_t,
) {
    let mut z0: u128 = 0;
    let mut mid1: u128 = 0;
    let mut mid: u128 = 0;
    let mut z128: u128 = 0;
    z0 = (a0 as u128).wrapping_mul(b0 as u128);
    mid1 = (a64 as u128).wrapping_mul(b0 as u128);
    mid = mid1.wrapping_add((a0 as u128).wrapping_mul(b64 as u128));
    z128 = (a64 as u128).wrapping_mul(b64 as u128);
    z128 = z128
        .wrapping_add(
            ((mid < mid1) as i32 as u128) << 64 as i32
                | mid >> 64 as i32,
        );
    mid <<= 64 as i32;
    z0 = z0.wrapping_add(mid);
    z128 = z128.wrapping_add((z0 < mid) as i32 as u128);
    *zPtr.offset(0 as i32 as isize) = z0 as uint64_t;
    *zPtr.offset(1 as i32 as isize) = (z0 >> 64 as i32) as uint64_t;
    *zPtr.offset(2 as i32 as isize) = z128 as uint64_t;
    *zPtr.offset(3 as i32 as isize) = (z128 >> 64 as i32) as uint64_t;
}
#[inline]
unsafe fn softfloat_shortShiftLeft128(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut dist: uint_fast8_t,
) -> uint128 {
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    z
        .v64 = a64 << dist as i32
        | a0 >> (-(dist as i32) & 63 as i32);
    z.v0 = a0 << dist as i32;
    return z;
}
#[inline]
unsafe fn softfloat_shortShiftRightJam128Extra(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut extra: uint64_t,
    mut dist: uint_fast8_t,
) -> uint128_extra {
    let mut negDist: uint_fast8_t = -(dist as i32) as uint_fast8_t;
    let mut z: uint128_extra = uint128_extra {
        extra: 0,
        v: uint128 { v0: 0, v64: 0 },
    };
    z.v.v64 = a64 >> dist as i32;
    z
        .v
        .v0 = a64 << (negDist as i32 & 63 as i32)
        | a0 >> dist as i32;
    z
        .extra = a0 << (negDist as i32 & 63 as i32)
        | (extra != 0 as i32 as u64) as i32 as u64;
    return z;
}
#[inline]
unsafe fn softfloat_add128(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut b64: uint64_t,
    mut b0: uint64_t,
) -> uint128 {
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    z.v0 = a0.wrapping_add(b0);
    z
        .v64 = a64
        .wrapping_add(b64)
        .wrapping_add((z.v0 < a0) as i32 as u64);
    return z;
}
pub unsafe fn f128_mul(mut a: float128_t, mut b: float128_t) -> float128_t {
    let mut current_block: u64;
    let mut uA: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiA64: uint_fast64_t = 0;
    let mut uiA0: uint_fast64_t = 0;
    let mut signA: bool = false;
    let mut expA: int_fast32_t = 0;
    let mut sigA: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uB: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    let mut uiB64: uint_fast64_t = 0;
    let mut uiB0: uint_fast64_t = 0;
    let mut signB: bool = false;
    let mut expB: int_fast32_t = 0;
    let mut sigB: uint128 = uint128 { v0: 0, v64: 0 };
    let mut signZ: bool = false;
    let mut magBits: uint_fast64_t = 0;
    let mut normExpSig: exp32_sig128 = exp32_sig128 {
        exp: 0,
        sig: uint128 { v0: 0, v64: 0 },
    };
    let mut expZ: int_fast32_t = 0;
    let mut sig256Z: [uint64_t; 4] = [0; 4];
    let mut sigZExtra: uint_fast64_t = 0;
    let mut sigZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut sig128Extra: uint128_extra = uint128_extra {
        extra: 0,
        v: uint128 { v0: 0, v64: 0 },
    };
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    uA.f = a;
    uiA64 = uA.ui.v64;
    uiA0 = uA.ui.v0;
    signA = uiA64 >> 63 as i32 != 0;
    expA = (uiA64 >> 48 as i32) as int_fast32_t
        & 0x7fff as i32 as i64;
    sigA.v64 = uiA64 & 0xffffffffffff as u64;
    sigA.v0 = uiA0;
    uB.f = b;
    uiB64 = uB.ui.v64;
    uiB0 = uB.ui.v0;
    signB = uiB64 >> 63 as i32 != 0;
    expB = (uiB64 >> 48 as i32) as int_fast32_t
        & 0x7fff as i32 as i64;
    sigB.v64 = uiB64 & 0xffffffffffff as u64;
    sigB.v0 = uiB0;
    signZ = signA as i32 ^ signB as i32 != 0;
    if expA == 0x7fff as i32 as i64 {
        if sigA.v64 | sigA.v0 != 0
            || expB == 0x7fff as i32 as i64 && sigB.v64 | sigB.v0 != 0
        {
            current_block = 6807611678581382739;
        } else {
            magBits = expB as u64 | sigB.v64 | sigB.v0;
            current_block = 1668438309440545211;
        }
    } else if expB == 0x7fff as i32 as i64 {
        if sigB.v64 | sigB.v0 != 0 {
            current_block = 6807611678581382739;
        } else {
            magBits = expA as u64 | sigA.v64 | sigA.v0;
            current_block = 1668438309440545211;
        }
    } else {
        if expA == 0 {
            if sigA.v64 | sigA.v0 == 0 {
                current_block = 2229079202108382871;
            } else {
                normExpSig = softfloat_normSubnormalF128Sig(sigA.v64, sigA.v0);
                expA = normExpSig.exp;
                sigA = normExpSig.sig;
                current_block = 14763689060501151050;
            }
        } else {
            current_block = 14763689060501151050;
        }
        match current_block {
            14763689060501151050 => {
                if expB == 0 {
                    if sigB.v64 | sigB.v0 == 0 {
                        current_block = 2229079202108382871;
                    } else {
                        normExpSig = softfloat_normSubnormalF128Sig(sigB.v64, sigB.v0);
                        expB = normExpSig.exp;
                        sigB = normExpSig.sig;
                        current_block = 15897653523371991391;
                    }
                } else {
                    current_block = 15897653523371991391;
                }
                match current_block {
                    2229079202108382871 => {}
                    _ => {
                        expZ = expA + expB - 0x4000 as i32 as i64;
                        sigA.v64 |= 0x1000000000000 as u64;
                        sigB = softfloat_shortShiftLeft128(
                            sigB.v64,
                            sigB.v0,
                            16 as i32 as uint_fast8_t,
                        );
                        softfloat_mul128To256M(
                            sigA.v64,
                            sigA.v0,
                            sigB.v64,
                            sigB.v0,
                            sig256Z.as_mut_ptr(),
                        );
                        sigZExtra = sig256Z[1 as i32 as usize]
                            | (sig256Z[0 as i32 as usize]
                                != 0 as i32 as u64) as i32
                                as u64;
                        sigZ = softfloat_add128(
                            sig256Z[3 as i32 as usize],
                            sig256Z[2 as i32 as usize],
                            sigA.v64,
                            sigA.v0,
                        );
                        if 0x2000000000000 as u64 <= sigZ.v64 {
                            expZ += 1;
                            sig128Extra = softfloat_shortShiftRightJam128Extra(
                                sigZ.v64,
                                sigZ.v0,
                                sigZExtra,
                                1 as i32 as uint_fast8_t,
                            );
                            sigZ = sig128Extra.v;
                            sigZExtra = sig128Extra.extra;
                        }
                        return softfloat_roundPackToF128(
                            signZ,
                            expZ,
                            sigZ.v64,
                            sigZ.v0,
                            sigZExtra,
                        );
                    }
                }
            }
            _ => {}
        }
        uiZ
            .v64 = ((signZ as uint_fast64_t) << 63 as i32)
            .wrapping_add((0 as i32 as uint_fast64_t) << 48 as i32)
            .wrapping_add(0 as i32 as u64);
        current_block = 13449254665990547442;
    }
    match current_block {
        1668438309440545211 => {
            if magBits == 0 {
                softfloat_raiseFlags(
                    softfloat_flag_invalid as i32 as uint_fast8_t,
                );
                uiZ.v64 = 0xffff800000000000 as u64;
                uiZ.v0 = 0 as u64;
                current_block = 476932901643527941;
            } else {
                uiZ
                    .v64 = ((signZ as uint_fast64_t) << 63 as i32)
                    .wrapping_add(
                        (0x7fff as i32 as uint_fast64_t) << 48 as i32,
                    )
                    .wrapping_add(0 as i32 as u64);
                current_block = 13449254665990547442;
            }
        }
        6807611678581382739 => {
            uiZ = softfloat_propagateNaNF128UI(uiA64, uiA0, uiB64, uiB0);
            current_block = 476932901643527941;
        }
        _ => {}
    }
    match current_block {
        13449254665990547442 => {
            uiZ.v0 = 0 as i32 as uint64_t;
        }
        _ => {}
    }
    uZ.ui = uiZ;
    return uZ.f;
}
