use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type int_fast32_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;


#[derive(Copy, Clone)]
#[repr(C)]
pub union ui128_f128 {
    pub ui: uint128,
    pub f: float128_t,
}
pub type C2RustUnnamed = u32;
pub const softfloat_mulAdd_subProd: C2RustUnnamed = 2;
pub const softfloat_mulAdd_subC: C2RustUnnamed = 1;

pub const softfloat_round_min: C2RustUnnamed_0 = 2;
pub const softfloat_flag_invalid: C2RustUnnamed_1 = 16;
pub type C2RustUnnamed_0 = u32;
pub const softfloat_round_odd: C2RustUnnamed_0 = 6;
pub const softfloat_round_near_maxMag: C2RustUnnamed_0 = 4;
pub const softfloat_round_max: C2RustUnnamed_0 = 3;
pub const softfloat_round_minMag: C2RustUnnamed_0 = 1;
pub const softfloat_round_near_even: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = u32;
pub const softfloat_flag_infinite: C2RustUnnamed_1 = 8;
pub const softfloat_flag_overflow: C2RustUnnamed_1 = 4;
pub const softfloat_flag_underflow: C2RustUnnamed_1 = 2;
pub const softfloat_flag_inexact: C2RustUnnamed_1 = 1;
#[inline]
unsafe fn softfloat_countLeadingZeros64(mut a: uint64_t) -> uint_fast8_t {
    return (if a != 0 {
        (a as u64).leading_zeros() as i32
    } else {
        64 as i32
    }) as uint_fast8_t;
}
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
unsafe fn softfloat_shortShiftRight128(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut dist: uint_fast8_t,
) -> uint128 {
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    z.v64 = a64 >> dist as i32;
    z
        .v0 = a64 << (-(dist as i32) & 63 as i32)
        | a0 >> dist as i32;
    return z;
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
#[inline]
unsafe fn softfloat_sub128(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut b64: uint64_t,
    mut b0: uint64_t,
) -> uint128 {
    let mut z: uint128 = uint128 { v0: 0, v64: 0 };
    z.v0 = a0.wrapping_sub(b0);
    z.v64 = a64.wrapping_sub(b64);
    z
        .v64 = (z.v64 as u64)
        .wrapping_sub((a0 < b0) as i32 as u64) as uint64_t as uint64_t;
    return z;
}
pub unsafe fn softfloat_mulAddF128(
    mut uiA64: uint_fast64_t,
    mut uiA0: uint_fast64_t,
    mut uiB64: uint_fast64_t,
    mut uiB0: uint_fast64_t,
    mut uiC64: uint_fast64_t,
    mut uiC0: uint_fast64_t,
    mut op: uint_fast8_t,
) -> float128_t {
    let mut current_block: u64;
    let mut signA: bool = false;
    let mut expA: int_fast32_t = 0;
    let mut sigA: uint128 = uint128 { v0: 0, v64: 0 };
    let mut signB: bool = false;
    let mut expB: int_fast32_t = 0;
    let mut sigB: uint128 = uint128 { v0: 0, v64: 0 };
    let mut signC: bool = false;
    let mut expC: int_fast32_t = 0;
    let mut sigC: uint128 = uint128 { v0: 0, v64: 0 };
    let mut signZ: bool = false;
    let mut magBits: uint_fast64_t = 0;
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut normExpSig: exp32_sig128 = exp32_sig128 {
        exp: 0,
        sig: uint128 { v0: 0, v64: 0 },
    };
    let mut expZ: int_fast32_t = 0;
    let mut sig256Z: [uint64_t; 4] = [0; 4];
    let mut sigZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut shiftDist: int_fast32_t = 0;
    let mut expDiff: int_fast32_t = 0;
    let mut x128: uint128 = uint128 { v0: 0, v64: 0 };
    let mut sig256C: [uint64_t; 4] = [0; 4];
    static mut zero256: [uint64_t; 4] = [
        0 as i32 as uint64_t,
        0 as i32 as uint64_t,
        0 as i32 as uint64_t,
        0 as i32 as uint64_t,
    ];
    let mut sigZExtra: uint_fast64_t = 0;
    let mut sig256Z0: uint_fast64_t = 0;
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    signA = uiA64 >> 63 as i32 != 0;
    expA = (uiA64 >> 48 as i32) as int_fast32_t
        & 0x7fff as i32 as i64;
    sigA.v64 = uiA64 & 0xffffffffffff as u64;
    sigA.v0 = uiA0;
    signB = uiB64 >> 63 as i32 != 0;
    expB = (uiB64 >> 48 as i32) as int_fast32_t
        & 0x7fff as i32 as i64;
    sigB.v64 = uiB64 & 0xffffffffffff as u64;
    sigB.v0 = uiB0;
    signC = (uiC64 >> 63 as i32 != 0) as i32
        ^ (op as i32 == softfloat_mulAdd_subC as i32) as i32
        != 0;
    expC = (uiC64 >> 48 as i32) as int_fast32_t
        & 0x7fff as i32 as i64;
    sigC.v64 = uiC64 & 0xffffffffffff as u64;
    sigC.v0 = uiC0;
    signZ = signA as i32 ^ signB as i32
        ^ (op as i32 == softfloat_mulAdd_subProd as i32) as i32
        != 0;
    if expA == 0x7fff as i32 as i64 {
        if sigA.v64 | sigA.v0 != 0
            || expB == 0x7fff as i32 as i64 && sigB.v64 | sigB.v0 != 0
        {
            current_block = 12035207565905621996;
        } else {
            magBits = expB as u64 | sigB.v64 | sigB.v0;
            current_block = 7554844609580121914;
        }
    } else if expB == 0x7fff as i32 as i64 {
        if sigB.v64 | sigB.v0 != 0 {
            current_block = 12035207565905621996;
        } else {
            magBits = expA as u64 | sigA.v64 | sigA.v0;
            current_block = 7554844609580121914;
        }
    } else if expC == 0x7fff as i32 as i64 {
        if sigC.v64 | sigC.v0 != 0 {
            uiZ.v64 = 0 as i32 as uint64_t;
            uiZ.v0 = 0 as i32 as uint64_t;
            current_block = 742596986223782323;
        } else {
            uiZ.v64 = uiC64;
            uiZ.v0 = uiC0;
            current_block = 7422456313092555880;
        }
    } else {
        if expA == 0 {
            if sigA.v64 | sigA.v0 == 0 {
                current_block = 14269454698284583605;
            } else {
                normExpSig = softfloat_normSubnormalF128Sig(sigA.v64, sigA.v0);
                expA = normExpSig.exp;
                sigA = normExpSig.sig;
                current_block = 652864300344834934;
            }
        } else {
            current_block = 652864300344834934;
        }
        match current_block {
            652864300344834934 => {
                if expB == 0 {
                    if sigB.v64 | sigB.v0 == 0 {
                        current_block = 14269454698284583605;
                    } else {
                        normExpSig = softfloat_normSubnormalF128Sig(sigB.v64, sigB.v0);
                        expB = normExpSig.exp;
                        sigB = normExpSig.sig;
                        current_block = 1836292691772056875;
                    }
                } else {
                    current_block = 1836292691772056875;
                }
                match current_block {
                    14269454698284583605 => {}
                    _ => {
                        expZ = expA + expB - 0x3ffe as i32 as i64;
                        sigA.v64 |= 0x1000000000000 as u64;
                        sigB.v64 |= 0x1000000000000 as u64;
                        sigA = softfloat_shortShiftLeft128(
                            sigA.v64,
                            sigA.v0,
                            8 as i32 as uint_fast8_t,
                        );
                        sigB = softfloat_shortShiftLeft128(
                            sigB.v64,
                            sigB.v0,
                            15 as i32 as uint_fast8_t,
                        );
                        softfloat_mul128To256M(
                            sigA.v64,
                            sigA.v0,
                            sigB.v64,
                            sigB.v0,
                            sig256Z.as_mut_ptr(),
                        );
                        sigZ.v64 = sig256Z[3 as i32 as usize];
                        sigZ.v0 = sig256Z[2 as i32 as usize];
                        shiftDist = 0 as i32 as int_fast32_t;
                        if sigZ.v64 & 0x100000000000000 as u64 == 0 {
                            expZ -= 1;
                            shiftDist = -(1 as i32) as int_fast32_t;
                        }
                        if expC == 0 {
                            if sigC.v64 | sigC.v0 == 0 {
                                shiftDist += 8 as i32 as i64;
                                current_block = 2955676227351570811;
                            } else {
                                normExpSig = softfloat_normSubnormalF128Sig(
                                    sigC.v64,
                                    sigC.v0,
                                );
                                expC = normExpSig.exp;
                                sigC = normExpSig.sig;
                                current_block = 13678349939556791712;
                            }
                        } else {
                            current_block = 13678349939556791712;
                        }
                        match current_block {
                            13678349939556791712 => {
                                sigC.v64 |= 0x1000000000000 as u64;
                                sigC = softfloat_shortShiftLeft128(
                                    sigC.v64,
                                    sigC.v0,
                                    8 as i32 as uint_fast8_t,
                                );
                                expDiff = expZ - expC;
                                if expDiff < 0 as i32 as i64 {
                                    expZ = expC;
                                    if signZ as i32 == signC as i32
                                        || expDiff < -(1 as i32) as i64
                                    {
                                        shiftDist -= expDiff;
                                        if shiftDist != 0 {
                                            sigZ = softfloat_shiftRightJam128(
                                                sigZ.v64,
                                                sigZ.v0,
                                                shiftDist as uint_fast32_t,
                                            );
                                        }
                                    } else if shiftDist == 0 {
                                        x128 = softfloat_shortShiftRight128(
                                            sig256Z[1 as i32 as usize],
                                            sig256Z[0 as i32 as usize],
                                            1 as i32 as uint_fast8_t,
                                        );
                                        sig256Z[1 as i32
                                            as usize] = sigZ.v0 << 63 as i32 | x128.v64;
                                        sig256Z[0 as i32 as usize] = x128.v0;
                                        sigZ = softfloat_shortShiftRight128(
                                            sigZ.v64,
                                            sigZ.v0,
                                            1 as i32 as uint_fast8_t,
                                        );
                                        sig256Z[3 as i32 as usize] = sigZ.v64;
                                        sig256Z[2 as i32 as usize] = sigZ.v0;
                                    }
                                } else {
                                    if shiftDist != 0 {
                                        softfloat_add256M(
                                            sig256Z.as_mut_ptr(),
                                            sig256Z.as_mut_ptr(),
                                            sig256Z.as_mut_ptr(),
                                        );
                                    }
                                    if expDiff == 0 {
                                        sigZ.v64 = sig256Z[3 as i32 as usize];
                                        sigZ.v0 = sig256Z[2 as i32 as usize];
                                    } else {
                                        sig256C[3 as i32 as usize] = sigC.v64;
                                        sig256C[2 as i32 as usize] = sigC.v0;
                                        sig256C[1 as i32
                                            as usize] = 0 as i32 as uint64_t;
                                        sig256C[0 as i32
                                            as usize] = 0 as i32 as uint64_t;
                                        softfloat_shiftRightJam256M(
                                            sig256C.as_mut_ptr(),
                                            expDiff as uint_fast32_t,
                                            sig256C.as_mut_ptr(),
                                        );
                                    }
                                }
                                shiftDist = 8 as i32 as int_fast32_t;
                                if signZ as i32 == signC as i32 {
                                    if expDiff <= 0 as i32 as i64 {
                                        sigZ = softfloat_add128(
                                            sigC.v64,
                                            sigC.v0,
                                            sigZ.v64,
                                            sigZ.v0,
                                        );
                                    } else {
                                        softfloat_add256M(
                                            sig256Z.as_mut_ptr(),
                                            sig256C.as_mut_ptr(),
                                            sig256Z.as_mut_ptr(),
                                        );
                                        sigZ.v64 = sig256Z[3 as i32 as usize];
                                        sigZ.v0 = sig256Z[2 as i32 as usize];
                                    }
                                    if sigZ.v64 & 0x200000000000000 as u64 != 0 {
                                        expZ += 1;
                                        shiftDist = 9 as i32 as int_fast32_t;
                                    }
                                    current_block = 2955676227351570811;
                                } else {
                                    if expDiff < 0 as i32 as i64 {
                                        signZ = signC;
                                        if expDiff < -(1 as i32) as i64 {
                                            sigZ = softfloat_sub128(
                                                sigC.v64,
                                                sigC.v0,
                                                sigZ.v64,
                                                sigZ.v0,
                                            );
                                            sigZExtra = sig256Z[1 as i32 as usize]
                                                | sig256Z[0 as i32 as usize];
                                            if sigZExtra != 0 {
                                                sigZ = softfloat_sub128(
                                                    sigZ.v64,
                                                    sigZ.v0,
                                                    0 as i32 as uint64_t,
                                                    1 as i32 as uint64_t,
                                                );
                                            }
                                            if sigZ.v64 & 0x100000000000000 as u64 == 0 {
                                                expZ -= 1;
                                                shiftDist = 7 as i32 as int_fast32_t;
                                            }
                                            current_block = 10179294621876634893;
                                        } else {
                                            sig256C[3 as i32 as usize] = sigC.v64;
                                            sig256C[2 as i32 as usize] = sigC.v0;
                                            sig256C[1 as i32
                                                as usize] = 0 as i32 as uint64_t;
                                            sig256C[0 as i32
                                                as usize] = 0 as i32 as uint64_t;
                                            softfloat_sub256M(
                                                sig256C.as_mut_ptr(),
                                                sig256Z.as_mut_ptr(),
                                                sig256Z.as_mut_ptr(),
                                            );
                                            current_block = 6478348674394853609;
                                        }
                                    } else if expDiff == 0 {
                                        sigZ = softfloat_sub128(
                                            sigZ.v64,
                                            sigZ.v0,
                                            sigC.v64,
                                            sigC.v0,
                                        );
                                        if sigZ.v64 | sigZ.v0 == 0
                                            && sig256Z[1 as i32 as usize] == 0
                                            && sig256Z[0 as i32 as usize] == 0
                                        {
                                            current_block = 11587282327478882474;
                                        } else {
                                            sig256Z[3 as i32 as usize] = sigZ.v64;
                                            sig256Z[2 as i32 as usize] = sigZ.v0;
                                            if sigZ.v64 & 0x8000000000000000 as u64 != 0 {
                                                signZ = !signZ;
                                                softfloat_sub256M(
                                                    zero256.as_mut_ptr(),
                                                    sig256Z.as_mut_ptr(),
                                                    sig256Z.as_mut_ptr(),
                                                );
                                            }
                                            current_block = 6478348674394853609;
                                        }
                                    } else {
                                        softfloat_sub256M(
                                            sig256Z.as_mut_ptr(),
                                            sig256C.as_mut_ptr(),
                                            sig256Z.as_mut_ptr(),
                                        );
                                        if (1 as i32 as i64) < expDiff {
                                            sigZ.v64 = sig256Z[3 as i32 as usize];
                                            sigZ.v0 = sig256Z[2 as i32 as usize];
                                            if sigZ.v64 & 0x100000000000000 as u64 == 0 {
                                                expZ -= 1;
                                                shiftDist = 7 as i32 as int_fast32_t;
                                            }
                                            current_block = 2955676227351570811;
                                        } else {
                                            current_block = 6478348674394853609;
                                        }
                                    }
                                    match current_block {
                                        10179294621876634893 => {}
                                        2955676227351570811 => {}
                                        11587282327478882474 => {}
                                        _ => {
                                            sigZ.v64 = sig256Z[3 as i32 as usize];
                                            sigZ.v0 = sig256Z[2 as i32 as usize];
                                            sigZExtra = sig256Z[1 as i32 as usize];
                                            sig256Z0 = sig256Z[0 as i32 as usize];
                                            if sigZ.v64 != 0 {
                                                if sig256Z0 != 0 {
                                                    sigZExtra |= 1 as i32 as u64;
                                                }
                                            } else {
                                                expZ -= 64 as i32 as i64;
                                                sigZ.v64 = sigZ.v0;
                                                sigZ.v0 = sigZExtra;
                                                sigZExtra = sig256Z0;
                                                if sigZ.v64 == 0 {
                                                    expZ -= 64 as i32 as i64;
                                                    sigZ.v64 = sigZ.v0;
                                                    sigZ.v0 = sigZExtra;
                                                    sigZExtra = 0 as i32 as uint_fast64_t;
                                                    if sigZ.v64 == 0 {
                                                        expZ -= 64 as i32 as i64;
                                                        sigZ.v64 = sigZ.v0;
                                                        sigZ.v0 = 0 as i32 as uint64_t;
                                                    }
                                                }
                                            }
                                            shiftDist = softfloat_countLeadingZeros64(sigZ.v64)
                                                as int_fast32_t;
                                            expZ += 7 as i32 as i64 - shiftDist;
                                            shiftDist = 15 as i32 as i64 - shiftDist;
                                            if (0 as i32 as i64) < shiftDist {
                                                current_block = 10179294621876634893;
                                            } else {
                                                if shiftDist != 0 {
                                                    shiftDist = -shiftDist;
                                                    sigZ = softfloat_shortShiftLeft128(
                                                        sigZ.v64,
                                                        sigZ.v0,
                                                        shiftDist as uint_fast8_t,
                                                    );
                                                    x128 = softfloat_shortShiftLeft128(
                                                        0 as i32 as uint64_t,
                                                        sigZExtra,
                                                        shiftDist as uint_fast8_t,
                                                    );
                                                    sigZ.v0 |= x128.v64;
                                                    sigZExtra = x128.v0;
                                                }
                                                current_block = 5980475890928805417;
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                        match current_block {
                            11587282327478882474 => {}
                            _ => {
                                match current_block {
                                    2955676227351570811 => {
                                        sigZExtra = sig256Z[1 as i32 as usize]
                                            | sig256Z[0 as i32 as usize];
                                        current_block = 10179294621876634893;
                                    }
                                    _ => {}
                                }
                                match current_block {
                                    10179294621876634893 => {
                                        sigZExtra = sigZ.v0
                                            << 64 as i32 as i64 - shiftDist
                                            | (sigZExtra != 0 as i32 as u64)
                                                as i32 as u64;
                                        sigZ = softfloat_shortShiftRight128(
                                            sigZ.v64,
                                            sigZ.v0,
                                            shiftDist as uint_fast8_t,
                                        );
                                    }
                                    _ => {}
                                }
                                return softfloat_roundPackToF128(
                                    signZ,
                                    expZ - 1 as i32 as i64,
                                    sigZ.v64,
                                    sigZ.v0,
                                    sigZExtra,
                                );
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        match current_block {
            14269454698284583605 => {
                uiZ.v64 = uiC64;
                uiZ.v0 = uiC0;
                if expC as u64 | sigC.v64 | sigC.v0 == 0
                    && signZ as i32 != signC as i32
                {
                    current_block = 11587282327478882474;
                } else {
                    current_block = 7422456313092555880;
                }
            }
            _ => {}
        }
        match current_block {
            7422456313092555880 => {}
            _ => {
                uiZ
                    .v64 = (((softfloat_roundingMode as i32
                    == softfloat_round_min as i32) as i32
                    as uint_fast64_t) << 63 as i32)
                    .wrapping_add(
                        (0 as i32 as uint_fast64_t) << 48 as i32,
                    )
                    .wrapping_add(0 as i32 as u64);
                uiZ.v0 = 0 as i32 as uint64_t;
                current_block = 7422456313092555880;
            }
        }
    }
    match current_block {
        7554844609580121914 => {
            if magBits != 0 {
                uiZ
                    .v64 = ((signZ as uint_fast64_t) << 63 as i32)
                    .wrapping_add(
                        (0x7fff as i32 as uint_fast64_t) << 48 as i32,
                    )
                    .wrapping_add(0 as i32 as u64);
                uiZ.v0 = 0 as i32 as uint64_t;
                if expC != 0x7fff as i32 as i64 {
                    current_block = 7422456313092555880;
                } else if sigC.v64 | sigC.v0 != 0 {
                    current_block = 742596986223782323;
                } else if signZ as i32 == signC as i32 {
                    current_block = 7422456313092555880;
                } else {
                    current_block = 4330759529560430365;
                }
            } else {
                current_block = 4330759529560430365;
            }
            match current_block {
                7422456313092555880 => {}
                742596986223782323 => {}
                _ => {
                    softfloat_raiseFlags(
                        softfloat_flag_invalid as i32 as uint_fast8_t,
                    );
                    uiZ.v64 = 0xffff800000000000 as u64;
                    uiZ.v0 = 0 as u64;
                    current_block = 742596986223782323;
                }
            }
        }
        12035207565905621996 => {
            uiZ = softfloat_propagateNaNF128UI(uiA64, uiA0, uiB64, uiB0);
            current_block = 742596986223782323;
        }
        _ => {}
    }
    match current_block {
        742596986223782323 => {
            uiZ = softfloat_propagateNaNF128UI(uiZ.v64, uiZ.v0, uiC64, uiC0);
        }
        _ => {}
    }
    uZ.ui = uiZ;
    return uZ.f;
}
