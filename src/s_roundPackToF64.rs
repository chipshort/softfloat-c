use crate::*;

pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type int_fast16_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui64_f64 {
    pub ui: uint64_t,
    pub f: float64_t,
}
pub const softfloat_round_odd: C2RustUnnamed_0 = 6;
pub const softfloat_flag_inexact: C2RustUnnamed_1 = 1;
pub const softfloat_flag_overflow: C2RustUnnamed_1 = 4;
pub const softfloat_flag_underflow: C2RustUnnamed_1 = 2;
pub const softfloat_tininess_beforeRounding: C2RustUnnamed = 0;
pub const softfloat_round_max: C2RustUnnamed_0 = 3;
pub const softfloat_round_min: C2RustUnnamed_0 = 2;
pub const softfloat_round_near_maxMag: C2RustUnnamed_0 = 4;
pub const softfloat_round_near_even: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed = u32;
pub const softfloat_tininess_afterRounding: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = u32;
pub const softfloat_round_minMag: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = u32;
pub const softfloat_flag_invalid: C2RustUnnamed_1 = 16;
pub const softfloat_flag_infinite: C2RustUnnamed_1 = 8;
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
pub unsafe fn softfloat_roundPackToF64(
    mut sign: bool,
    mut exp: int_fast16_t,
    mut sig: uint_fast64_t,
) -> float64_t {
    let mut current_block: u64;
    let mut roundingMode: uint_fast8_t = 0;
    let mut roundNearEven: bool = false;
    let mut roundIncrement: uint_fast16_t = 0;
    let mut roundBits: uint_fast16_t = 0;
    let mut isTiny: bool = false;
    let mut uiZ: uint_fast64_t = 0;
    let mut uZ: ui64_f64 = ui64_f64 { ui: 0 };
    roundingMode = softfloat_roundingMode;
    roundNearEven = roundingMode as i32
        == softfloat_round_near_even as i32;
    roundIncrement = 0x200 as i32 as uint_fast16_t;
    if !roundNearEven
        && roundingMode as i32 != softfloat_round_near_maxMag as i32
    {
        roundIncrement = (if roundingMode as i32
            == (if sign as i32 != 0 {
                softfloat_round_min as i32
            } else {
                softfloat_round_max as i32
            })
        {
            0x3ff as i32
        } else {
            0 as i32
        }) as uint_fast16_t;
    }
    roundBits = sig & 0x3ff as i32 as u64;
    if 0x7fd as i32 <= exp as uint16_t as i32 {
        if exp < 0 as i32 as i64 {
            isTiny = softfloat_detectTininess as i32
                == softfloat_tininess_beforeRounding as i32
                || exp < -(1 as i32) as i64
                || sig.wrapping_add(roundIncrement)
                    < 0x8000000000000000 as u64;
            sig = softfloat_shiftRightJam64(sig, -exp as uint_fast32_t);
            exp = 0 as i32 as int_fast16_t;
            roundBits = sig & 0x3ff as i32 as u64;
            if isTiny as i32 != 0 && roundBits != 0 {
                softfloat_raiseFlags(
                    softfloat_flag_underflow as i32 as uint_fast8_t,
                );
            }
            current_block = 17833034027772472439;
        } else if (0x7fd as i32 as i64) < exp
                || 0x8000000000000000 as u64
                    <= sig.wrapping_add(roundIncrement)
            {
            softfloat_raiseFlags(
                (softfloat_flag_overflow as i32
                    | softfloat_flag_inexact as i32) as uint_fast8_t,
            );
            uiZ = ((sign as uint_fast64_t) << 63 as i32)
                .wrapping_add(
                    (0x7ff as i32 as uint_fast64_t) << 52 as i32,
                )
                .wrapping_add(0 as i32 as u64)
                .wrapping_sub((roundIncrement == 0) as i32 as u64);
            current_block = 15697564613478109721;
        } else {
            current_block = 17833034027772472439;
        }
    } else {
        current_block = 17833034027772472439;
    }
    match current_block {
        17833034027772472439 => {
            sig = sig.wrapping_add(roundIncrement) >> 10 as i32;
            if roundBits != 0 {
                softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                    | softfloat_flag_inexact as i32) as uint_fast8_t;
                if roundingMode as i32 == softfloat_round_odd as i32 {
                    sig |= 1 as i32 as u64;
                    current_block = 10545073652187581578;
                } else {
                    current_block = 11042950489265723346;
                }
            } else {
                current_block = 11042950489265723346;
            }
            match current_block {
                11042950489265723346 => {
                    sig
                        &= !(((roundBits ^ 0x200 as i32 as u64 == 0)
                            as i32 & roundNearEven as i32)
                            as uint_fast64_t);
                    if sig == 0 {
                        exp = 0 as i32 as int_fast16_t;
                    }
                }
                _ => {}
            }
            uiZ = ((sign as uint_fast64_t) << 63 as i32)
                .wrapping_add((exp as uint_fast64_t) << 52 as i32)
                .wrapping_add(sig);
        }
        _ => {}
    }
    uZ.ui = uiZ;
    return uZ.f;
}
