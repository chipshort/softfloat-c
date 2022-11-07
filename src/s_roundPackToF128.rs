use crate::*;

pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
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
pub const softfloat_round_odd: C2RustUnnamed_0 = 6;
pub const softfloat_flag_inexact: C2RustUnnamed_1 = 1;
pub const softfloat_round_max: C2RustUnnamed_0 = 3;
pub const softfloat_round_min: C2RustUnnamed_0 = 2;
pub const softfloat_round_near_maxMag: C2RustUnnamed_0 = 4;
pub const softfloat_flag_overflow: C2RustUnnamed_1 = 4;
pub const softfloat_flag_underflow: C2RustUnnamed_1 = 2;
pub const softfloat_tininess_beforeRounding: C2RustUnnamed = 0;
pub const softfloat_round_near_even: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed = u32;
pub const softfloat_tininess_afterRounding: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = u32;
pub const softfloat_round_minMag: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = u32;
pub const softfloat_flag_invalid: C2RustUnnamed_1 = 16;
pub const softfloat_flag_infinite: C2RustUnnamed_1 = 8;
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
unsafe fn softfloat_eq128(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut b64: uint64_t,
    mut b0: uint64_t,
) -> bool {
    return a64 == b64 && a0 == b0;
}
#[inline]
unsafe fn softfloat_lt128(
    mut a64: uint64_t,
    mut a0: uint64_t,
    mut b64: uint64_t,
    mut b0: uint64_t,
) -> bool {
    return a64 < b64 || a64 == b64 && a0 < b0;
}
pub unsafe fn softfloat_roundPackToF128(
    mut sign: bool,
    mut exp: int_fast32_t,
    mut sig64: uint_fast64_t,
    mut sig0: uint_fast64_t,
    mut sigExtra: uint_fast64_t,
) -> float128_t {
    let mut current_block: u64;
    let mut roundingMode: uint_fast8_t = 0;
    let mut roundNearEven: bool = false;
    let mut doIncrement: bool = false;
    let mut isTiny: bool = false;
    let mut sig128Extra: uint128_extra = uint128_extra {
        extra: 0,
        v: uint128 { v0: 0, v64: 0 },
    };
    let mut uiZ64: uint_fast64_t = 0;
    let mut uiZ0: uint_fast64_t = 0;
    let mut sig128: uint128 = uint128 { v0: 0, v64: 0 };
    let mut uZ: ui128_f128 = ui128_f128 {
        ui: uint128 { v0: 0, v64: 0 },
    };
    roundingMode = softfloat_roundingMode;
    roundNearEven = roundingMode as i32
        == softfloat_round_near_even as i32;
    doIncrement = 0x8000000000000000 as u64 <= sigExtra;
    if !roundNearEven
        && roundingMode as i32 != softfloat_round_near_maxMag as i32
    {
        doIncrement = roundingMode as i32
            == (if sign as i32 != 0 {
                softfloat_round_min as i32
            } else {
                softfloat_round_max as i32
            }) && sigExtra != 0;
    }
    if 0x7ffd as i32 as u32 <= exp as uint32_t {
        if exp < 0 as i32 as i64 {
            isTiny = softfloat_detectTininess as i32
                == softfloat_tininess_beforeRounding as i32
                || exp < -(1 as i32) as i64 || !doIncrement
                || softfloat_lt128(
                    sig64,
                    sig0,
                    0x1ffffffffffff as u64,
                    0xffffffffffffffff as u64,
                ) as i32 != 0;
            sig128Extra = softfloat_shiftRightJam128Extra(
                sig64,
                sig0,
                sigExtra,
                -exp as uint_fast32_t,
            );
            sig64 = sig128Extra.v.v64;
            sig0 = sig128Extra.v.v0;
            sigExtra = sig128Extra.extra;
            exp = 0 as i32 as int_fast32_t;
            if isTiny as i32 != 0 && sigExtra != 0 {
                softfloat_raiseFlags(
                    softfloat_flag_underflow as i32 as uint_fast8_t,
                );
            }
            doIncrement = 0x8000000000000000 as u64 <= sigExtra;
            if !roundNearEven
                && roundingMode as i32
                    != softfloat_round_near_maxMag as i32
            {
                doIncrement = roundingMode as i32
                    == (if sign as i32 != 0 {
                        softfloat_round_min as i32
                    } else {
                        softfloat_round_max as i32
                    }) && sigExtra != 0;
            }
            current_block = 18386322304582297246;
        } else if (0x7ffd as i32 as i64) < exp
                || exp == 0x7ffd as i32 as i64
                    && softfloat_eq128(
                        sig64,
                        sig0,
                        0x1ffffffffffff as u64,
                        0xffffffffffffffff as u64,
                    ) as i32 != 0 && doIncrement as i32 != 0
            {
            softfloat_raiseFlags(
                (softfloat_flag_overflow as i32
                    | softfloat_flag_inexact as i32) as uint_fast8_t,
            );
            if roundNearEven as i32 != 0
                || roundingMode as i32
                    == softfloat_round_near_maxMag as i32
                || roundingMode as i32
                    == (if sign as i32 != 0 {
                        softfloat_round_min as i32
                    } else {
                        softfloat_round_max as i32
                    })
            {
                uiZ64 = ((sign as uint_fast64_t) << 63 as i32)
                    .wrapping_add(
                        (0x7fff as i32 as uint_fast64_t) << 48 as i32,
                    )
                    .wrapping_add(0 as i32 as u64);
                uiZ0 = 0 as i32 as uint_fast64_t;
            } else {
                uiZ64 = ((sign as uint_fast64_t) << 63 as i32)
                    .wrapping_add(
                        (0x7ffe as i32 as uint_fast64_t) << 48 as i32,
                    )
                    .wrapping_add(0xffffffffffff as u64);
                uiZ0 = 0xffffffffffffffff as u64;
            }
            current_block = 8140749886351844285;
        } else {
            current_block = 18386322304582297246;
        }
    } else {
        current_block = 18386322304582297246;
    }
    match current_block {
        18386322304582297246 => {
            if sigExtra != 0 {
                softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                    | softfloat_flag_inexact as i32) as uint_fast8_t;
                if roundingMode as i32 == softfloat_round_odd as i32 {
                    sig0 |= 1 as i32 as u64;
                    current_block = 2515457604341827833;
                } else {
                    current_block = 5689316957504528238;
                }
            } else {
                current_block = 5689316957504528238;
            }
            match current_block {
                5689316957504528238 => {
                    if doIncrement {
                        sig128 = softfloat_add128(
                            sig64,
                            sig0,
                            0 as i32 as uint64_t,
                            1 as i32 as uint64_t,
                        );
                        sig64 = sig128.v64;
                        sig0 = sig128.v0
                            & !(((sigExtra & 0x7fffffffffffffff as u64 == 0)
                                as i32 & roundNearEven as i32) as uint64_t);
                    } else if sig64 | sig0 == 0 {
                        exp = 0 as i32 as int_fast32_t;
                    }
                }
                _ => {}
            }
            uiZ64 = ((sign as uint_fast64_t) << 63 as i32)
                .wrapping_add((exp as uint_fast64_t) << 48 as i32)
                .wrapping_add(sig64);
            uiZ0 = sig0;
        }
        _ => {}
    }
    uZ.ui.v64 = uiZ64;
    uZ.ui.v0 = uiZ0;
    return uZ.f;
}
