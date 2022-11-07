use crate::*;

pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type int_fast32_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ui: u128,
    pub s: uint128,
}

pub type extFloat80_t = extFloat80M;

pub type C2RustUnnamed_0 = u32;
pub const softfloat_flag_invalid: C2RustUnnamed_0 = 16;
pub const softfloat_flag_infinite: C2RustUnnamed_0 = 8;
pub const softfloat_flag_overflow: C2RustUnnamed_0 = 4;
pub const softfloat_flag_underflow: C2RustUnnamed_0 = 2;
pub const softfloat_flag_inexact: C2RustUnnamed_0 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub s: extFloat80M,
    pub f: extFloat80_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub s: extFloat80M,
    pub f: extFloat80_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub s: extFloat80M,
    pub f: extFloat80_t,
}
#[inline]
unsafe fn softfloat_mul64To128(mut a: uint64_t, mut b: uint64_t) -> uint128 {
    let mut uZ: C2RustUnnamed = C2RustUnnamed { ui: 0 };
    uZ.ui = (a as u128).wrapping_mul(b as u128);
    return uZ.s;
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
pub unsafe fn extF80_mul(
    mut a: extFloat80_t,
    mut b: extFloat80_t,
) -> extFloat80_t {
    let mut current_block: u64;
    let mut uA: C2RustUnnamed_3 = C2RustUnnamed_3 {
        s: extFloat80_t {
            signif: 0,
            signExp: 0,
        },
    };
    let mut uiA64: uint_fast16_t = 0;
    let mut uiA0: uint_fast64_t = 0;
    let mut signA: bool = false;
    let mut expA: int_fast32_t = 0;
    let mut sigA: uint_fast64_t = 0;
    let mut uB: C2RustUnnamed_2 = C2RustUnnamed_2 {
        s: extFloat80_t {
            signif: 0,
            signExp: 0,
        },
    };
    let mut uiB64: uint_fast16_t = 0;
    let mut uiB0: uint_fast64_t = 0;
    let mut signB: bool = false;
    let mut expB: int_fast32_t = 0;
    let mut sigB: uint_fast64_t = 0;
    let mut signZ: bool = false;
    let mut magBits: uint_fast64_t = 0;
    let mut normExpSig: exp32_sig64 = exp32_sig64 { exp: 0, sig: 0 };
    let mut expZ: int_fast32_t = 0;
    let mut sig128Z: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uiZ64: uint_fast16_t = 0;
    let mut uiZ0: uint_fast64_t = 0;
    let mut uZ: C2RustUnnamed_1 = C2RustUnnamed_1 {
        s: extFloat80_t {
            signif: 0,
            signExp: 0,
        },
    };
    uA.f = a;
    uiA64 = uA.s.signExp as uint_fast16_t;
    uiA0 = uA.s.signif;
    signA = uiA64 as uint16_t as i32 >> 15 as i32 != 0;
    expA = (uiA64 & 0x7fff as i32 as u64) as int_fast32_t;
    sigA = uiA0;
    uB.f = b;
    uiB64 = uB.s.signExp as uint_fast16_t;
    uiB0 = uB.s.signif;
    signB = uiB64 as uint16_t as i32 >> 15 as i32 != 0;
    expB = (uiB64 & 0x7fff as i32 as u64) as int_fast32_t;
    sigB = uiB0;
    signZ = signA as i32 ^ signB as i32 != 0;
    if expA == 0x7fff as i32 as i64 {
        if sigA & 0x7fffffffffffffff as u64 != 0
            || expB == 0x7fff as i32 as i64
                && sigB & 0x7fffffffffffffff as u64 != 0
        {
            current_block = 6106652275726582626;
        } else {
            magBits = expB as u64 | sigB;
            current_block = 2536789419620059479;
        }
    } else if expB == 0x7fff as i32 as i64 {
        if sigB & 0x7fffffffffffffff as u64 != 0 {
            current_block = 6106652275726582626;
        } else {
            magBits = expA as u64 | sigA;
            current_block = 2536789419620059479;
        }
    } else {
        if expA == 0 {
            expA = 1 as i32 as int_fast32_t;
        }
        if sigA & 0x8000000000000000 as u64 == 0 {
            if sigA == 0 {
                current_block = 630912775473753207;
            } else {
                normExpSig = softfloat_normSubnormalExtF80Sig(sigA);
                expA += normExpSig.exp;
                sigA = normExpSig.sig;
                current_block = 13550086250199790493;
            }
        } else {
            current_block = 13550086250199790493;
        }
        match current_block {
            13550086250199790493 => {
                if expB == 0 {
                    expB = 1 as i32 as int_fast32_t;
                }
                if sigB & 0x8000000000000000 as u64 == 0 {
                    if sigB == 0 {
                        current_block = 630912775473753207;
                    } else {
                        normExpSig = softfloat_normSubnormalExtF80Sig(sigB);
                        expB += normExpSig.exp;
                        sigB = normExpSig.sig;
                        current_block = 14136749492126903395;
                    }
                } else {
                    current_block = 14136749492126903395;
                }
                match current_block {
                    630912775473753207 => {}
                    _ => {
                        expZ = expA + expB - 0x3ffe as i32 as i64;
                        sig128Z = softfloat_mul64To128(sigA, sigB);
                        if sig128Z.v64 < 0x8000000000000000 as u64 {
                            expZ -= 1;
                            sig128Z = softfloat_add128(
                                sig128Z.v64,
                                sig128Z.v0,
                                sig128Z.v64,
                                sig128Z.v0,
                            );
                        }
                        return softfloat_roundPackToExtF80(
                            signZ,
                            expZ,
                            sig128Z.v64,
                            sig128Z.v0,
                            extF80_roundingPrecision,
                        );
                    }
                }
            }
            _ => {}
        }
        uiZ64 = (signZ as uint_fast16_t) << 15 as i32
            | 0 as i32 as u64;
        uiZ0 = 0 as i32 as uint_fast64_t;
        current_block = 15151965201224081921;
    }
    match current_block {
        6106652275726582626 => {
            uiZ = softfloat_propagateNaNExtF80UI(uiA64, uiA0, uiB64, uiB0);
            uiZ64 = uiZ.v64;
            uiZ0 = uiZ.v0;
        }
        2536789419620059479 => {
            if magBits == 0 {
                softfloat_raiseFlags(
                    softfloat_flag_invalid as i32 as uint_fast8_t,
                );
                uiZ64 = 0xffff as i32 as uint_fast16_t;
                uiZ0 = 0xc000000000000000 as u64;
            } else {
                uiZ64 = (signZ as uint_fast16_t) << 15 as i32
                    | 0x7fff as i32 as u64;
                uiZ0 = 0x8000000000000000 as u64;
            }
        }
        _ => {}
    }
    uZ.s.signExp = uiZ64 as uint16_t;
    uZ.s.signif = uiZ0;
    return uZ.f;
}
