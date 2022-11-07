use crate::*;

pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type int_fast32_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uint64_extra {
    pub extra: uint64_t,
    pub v: uint64_t,
}

pub type extFloat80_t = extFloat80M;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub s: extFloat80M,
    pub f: extFloat80_t,
}
pub const softfloat_round_odd: C2RustUnnamed_1 = 6;
pub const softfloat_flag_inexact: C2RustUnnamed_2 = 1;
pub const softfloat_round_max: C2RustUnnamed_1 = 3;
pub const softfloat_round_min: C2RustUnnamed_1 = 2;
pub const softfloat_round_near_maxMag: C2RustUnnamed_1 = 4;
pub const softfloat_flag_overflow: C2RustUnnamed_2 = 4;
pub const softfloat_flag_underflow: C2RustUnnamed_2 = 2;
pub const softfloat_tininess_beforeRounding: C2RustUnnamed_0 = 0;
pub const softfloat_round_near_even: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_0 = u32;
pub const softfloat_tininess_afterRounding: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = u32;
pub const softfloat_round_minMag: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_2 = u32;
pub const softfloat_flag_invalid: C2RustUnnamed_2 = 16;
pub const softfloat_flag_infinite: C2RustUnnamed_2 = 8;
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
#[inline]
unsafe fn softfloat_shiftRightJam64Extra(
    mut a: uint64_t,
    mut extra: uint64_t,
    mut dist: uint_fast32_t,
) -> uint64_extra {
    let mut z: uint64_extra = uint64_extra { extra: 0, v: 0 };
    if dist < 64 as i32 as u64 {
        z.v = a >> dist;
        z.extra = a << (dist.wrapping_neg() & 63 as i32 as u64);
    } else {
        z.v = 0 as i32 as uint64_t;
        z
            .extra = if dist == 64 as i32 as u64 {
            a
        } else {
            (a != 0 as i32 as u64) as i32 as u64
        };
    }
    z.extra
        |= (extra != 0 as i32 as u64) as i32 as u64;
    return z;
}
pub unsafe fn softfloat_roundPackToExtF80(
    mut sign: bool,
    mut exp: int_fast32_t,
    mut sig: uint_fast64_t,
    mut sigExtra: uint_fast64_t,
    mut roundingPrecision: uint_fast8_t,
) -> extFloat80_t {
    let mut current_block: u64;
    let mut roundingMode: uint_fast8_t = 0;
    let mut roundNearEven: bool = false;
    let mut roundIncrement: uint_fast64_t = 0;
    let mut roundMask: uint_fast64_t = 0;
    let mut roundBits: uint_fast64_t = 0;
    let mut isTiny: bool = false;
    let mut doIncrement: bool = false;
    let mut sig64Extra: uint64_extra = uint64_extra { extra: 0, v: 0 };
    let mut uZ: C2RustUnnamed = C2RustUnnamed {
        s: extFloat80M {
            signif: 0,
            signExp: 0,
        },
    };
    roundingMode = softfloat_roundingMode;
    roundNearEven = roundingMode as i32
        == softfloat_round_near_even as i32;
    if roundingPrecision as i32 == 80 as i32 {
        current_block = 13283091372498831498;
    } else {
        if roundingPrecision as i32 == 64 as i32 {
            roundIncrement = 0x400 as u64;
            roundMask = 0x7ff as u64;
            current_block = 13536709405535804910;
        } else if roundingPrecision as i32 == 32 as i32 {
            roundIncrement = 0x8000000000 as u64;
            roundMask = 0xffffffffff as u64;
            current_block = 13536709405535804910;
        } else {
            current_block = 13283091372498831498;
        }
        match current_block {
            13283091372498831498 => {}
            _ => {
                sig
                    |= (sigExtra != 0 as i32 as u64) as i32
                        as u64;
                if !roundNearEven
                    && roundingMode as i32
                        != softfloat_round_near_maxMag as i32
                {
                    roundIncrement = if roundingMode as i32
                        == (if sign as i32 != 0 {
                            softfloat_round_min as i32
                        } else {
                            softfloat_round_max as i32
                        })
                    {
                        roundMask
                    } else {
                        0 as i32 as u64
                    };
                }
                roundBits = sig & roundMask;
                if 0x7ffd as i32 as u32
                    <= (exp - 1 as i32 as i64) as uint32_t
                {
                    if exp <= 0 as i32 as i64 {
                        isTiny = softfloat_detectTininess as i32
                            == softfloat_tininess_beforeRounding as i32
                            || exp < 0 as i32 as i64
                            || sig <= sig.wrapping_add(roundIncrement);
                        sig = softfloat_shiftRightJam64(
                            sig,
                            (1 as i32 as i64 - exp) as uint_fast32_t,
                        );
                        roundBits = sig & roundMask;
                        if roundBits != 0 {
                            if isTiny {
                                softfloat_raiseFlags(
                                    softfloat_flag_underflow as i32 as uint_fast8_t,
                                );
                            }
                            softfloat_exceptionFlags = (softfloat_exceptionFlags
                                as i32 | softfloat_flag_inexact as i32)
                                as uint_fast8_t;
                            if roundingMode as i32
                                == softfloat_round_odd as i32
                            {
                                sig
                                    |= roundMask
                                        .wrapping_add(1 as i32 as u64);
                            }
                        }
                        sig = (sig as u64).wrapping_add(roundIncrement)
                            as uint_fast64_t as uint_fast64_t;
                        exp = (sig & 0x8000000000000000 as u64
                            != 0 as i32 as u64) as i32
                            as int_fast32_t;
                        roundIncrement = roundMask
                            .wrapping_add(1 as i32 as u64);
                        if roundNearEven as i32 != 0
                            && roundBits << 1 as i32 == roundIncrement
                        {
                            roundMask |= roundIncrement;
                        }
                        sig &= !roundMask;
                        current_block = 11154085646137466785;
                    } else if (0x7ffe as i32 as i64) < exp
                            || exp == 0x7ffe as i32 as i64
                                && sig.wrapping_add(roundIncrement) < sig
                        {
                        current_block = 809462055393894747;
                    } else {
                        current_block = 5494826135382683477;
                    }
                } else {
                    current_block = 5494826135382683477;
                }
                match current_block {
                    11154085646137466785 => {}
                    809462055393894747 => {}
                    _ => {
                        if roundBits != 0 {
                            softfloat_exceptionFlags = (softfloat_exceptionFlags
                                as i32 | softfloat_flag_inexact as i32)
                                as uint_fast8_t;
                            if roundingMode as i32
                                == softfloat_round_odd as i32
                            {
                                sig = sig & !roundMask
                                    | roundMask.wrapping_add(1 as i32 as u64);
                                current_block = 11154085646137466785;
                            } else {
                                current_block = 17184638872671510253;
                            }
                        } else {
                            current_block = 17184638872671510253;
                        }
                        match current_block {
                            11154085646137466785 => {}
                            _ => {
                                sig = sig.wrapping_add(roundIncrement);
                                if sig < roundIncrement {
                                    exp += 1;
                                    sig = 0x8000000000000000 as u64;
                                }
                                roundIncrement = roundMask
                                    .wrapping_add(1 as i32 as u64);
                                if roundNearEven as i32 != 0
                                    && roundBits << 1 as i32 == roundIncrement
                                {
                                    roundMask |= roundIncrement;
                                }
                                sig &= !roundMask;
                                current_block = 11154085646137466785;
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        13283091372498831498 => {
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
            if 0x7ffd as i32 as u32
                <= (exp - 1 as i32 as i64) as uint32_t
            {
                if exp <= 0 as i32 as i64 {
                    isTiny = softfloat_detectTininess as i32
                        == softfloat_tininess_beforeRounding as i32
                        || exp < 0 as i32 as i64 || !doIncrement
                        || sig < 0xffffffffffffffff as u64;
                    sig64Extra = softfloat_shiftRightJam64Extra(
                        sig,
                        sigExtra,
                        (1 as i32 as i64 - exp) as uint_fast32_t,
                    );
                    exp = 0 as i32 as int_fast32_t;
                    sig = sig64Extra.v;
                    sigExtra = sig64Extra.extra;
                    if sigExtra != 0 {
                        if isTiny {
                            softfloat_raiseFlags(
                                softfloat_flag_underflow as i32 as uint_fast8_t,
                            );
                        }
                        softfloat_exceptionFlags = (softfloat_exceptionFlags
                            as i32 | softfloat_flag_inexact as i32)
                            as uint_fast8_t;
                        if roundingMode as i32
                            == softfloat_round_odd as i32
                        {
                            sig |= 1 as i32 as u64;
                            current_block = 11154085646137466785;
                        } else {
                            current_block = 8545136480011357681;
                        }
                    } else {
                        current_block = 8545136480011357681;
                    }
                    match current_block {
                        11154085646137466785 => {}
                        _ => {
                            doIncrement = 0x8000000000000000 as u64
                                <= sigExtra;
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
                            if doIncrement {
                                sig = sig.wrapping_add(1);
                                sig
                                    &= !(((sigExtra & 0x7fffffffffffffff as u64 == 0)
                                        as i32 & roundNearEven as i32)
                                        as uint_fast64_t);
                                exp = (sig & 0x8000000000000000 as u64
                                    != 0 as i32 as u64) as i32
                                    as int_fast32_t;
                            }
                            current_block = 11154085646137466785;
                        }
                    }
                } else if (0x7ffe as i32 as i64) < exp
                        || exp == 0x7ffe as i32 as i64
                            && sig == 0xffffffffffffffff as u64
                            && doIncrement as i32 != 0
                    {
                    roundMask = 0 as i32 as uint_fast64_t;
                    current_block = 809462055393894747;
                } else {
                    current_block = 7158658067966855297;
                }
            } else {
                current_block = 7158658067966855297;
            }
            match current_block {
                809462055393894747 => {}
                11154085646137466785 => {}
                _ => {
                    if sigExtra != 0 {
                        softfloat_exceptionFlags = (softfloat_exceptionFlags
                            as i32 | softfloat_flag_inexact as i32)
                            as uint_fast8_t;
                        if roundingMode as i32
                            == softfloat_round_odd as i32
                        {
                            sig |= 1 as i32 as u64;
                            current_block = 11154085646137466785;
                        } else {
                            current_block = 11052029508375673978;
                        }
                    } else {
                        current_block = 11052029508375673978;
                    }
                    match current_block {
                        11154085646137466785 => {}
                        _ => {
                            if doIncrement {
                                sig = sig.wrapping_add(1);
                                if sig == 0 {
                                    exp += 1;
                                    sig = 0x8000000000000000 as u64;
                                } else {
                                    sig
                                        &= !(((sigExtra & 0x7fffffffffffffff as u64 == 0)
                                            as i32 & roundNearEven as i32)
                                            as uint_fast64_t);
                                }
                            }
                            current_block = 11154085646137466785;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        809462055393894747 => {
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
                exp = 0x7fff as i32 as int_fast32_t;
                sig = 0x8000000000000000 as u64;
            } else {
                exp = 0x7ffe as i32 as int_fast32_t;
                sig = !roundMask;
            }
        }
        _ => {}
    }
    uZ
        .s
        .signExp = ((sign as uint_fast16_t) << 15 as i32 | exp as u64)
        as uint16_t;
    uZ.s.signif = sig;
    return uZ.f;
}
