use crate::*;

pub type __uint32_t = u32;
pub type uint32_t = __uint32_t;
pub type int_fast16_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast32_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui32_f32 {
    pub ui: uint32_t,
    pub f: float32_t,
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
pub unsafe fn softfloat_roundPackToF32(
    mut sign: bool,
    mut exp: int_fast16_t,
    mut sig: uint_fast32_t,
) -> float32_t {
    let mut current_block: u64;
    let mut roundingMode: uint_fast8_t = 0;
    let mut roundNearEven: bool = false;
    let mut roundIncrement: uint_fast8_t = 0;
    let mut roundBits: uint_fast8_t = 0;
    let mut isTiny: bool = false;
    let mut uiZ: uint_fast32_t = 0;
    let mut uZ: ui32_f32 = ui32_f32 { ui: 0 };
    roundingMode = softfloat_roundingMode;
    roundNearEven = roundingMode as i32
        == softfloat_round_near_even as i32;
    roundIncrement = 0x40 as i32 as uint_fast8_t;
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
            0x7f as i32
        } else {
            0 as i32
        }) as uint_fast8_t;
    }
    roundBits = (sig & 0x7f as i32 as u64) as uint_fast8_t;
    if 0xfd as i32 as u32 <= exp as u32 {
        if exp < 0 as i32 as i64 {
            isTiny = softfloat_detectTininess as i32
                == softfloat_tininess_beforeRounding as i32
                || exp < -(1 as i32) as i64
                || sig.wrapping_add(roundIncrement as u64)
                    < 0x80000000 as u32 as u64;
            sig = softfloat_shiftRightJam32(sig as uint32_t, -exp as uint_fast16_t)
                as uint_fast32_t;
            exp = 0 as i32 as int_fast16_t;
            roundBits = (sig & 0x7f as i32 as u64) as uint_fast8_t;
            if isTiny as i32 != 0 && roundBits as i32 != 0 {
                softfloat_raiseFlags(
                    softfloat_flag_underflow as i32 as uint_fast8_t,
                );
            }
            current_block = 17833034027772472439;
        } else if (0xfd as i32 as i64) < exp
                || 0x80000000 as u32 as u64
                    <= sig.wrapping_add(roundIncrement as u64)
            {
            softfloat_raiseFlags(
                (softfloat_flag_overflow as i32
                    | softfloat_flag_inexact as i32) as uint_fast8_t,
            );
            uiZ = ((sign as uint32_t) << 31 as i32)
                .wrapping_add((0xff as i32 as uint32_t) << 23 as i32)
                .wrapping_add(0 as i32 as u32)
                .wrapping_sub((roundIncrement == 0) as i32 as u32)
                as uint_fast32_t;
            current_block = 2735656305138411644;
        } else {
            current_block = 17833034027772472439;
        }
    } else {
        current_block = 17833034027772472439;
    }
    match current_block {
        17833034027772472439 => {
            sig = sig.wrapping_add(roundIncrement as u64) >> 7 as i32;
            if roundBits != 0 {
                softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                    | softfloat_flag_inexact as i32) as uint_fast8_t;
                if roundingMode as i32 == softfloat_round_odd as i32 {
                    sig |= 1 as i32 as u64;
                    current_block = 9951706356771178788;
                } else {
                    current_block = 11042950489265723346;
                }
            } else {
                current_block = 11042950489265723346;
            }
            match current_block {
                11042950489265723346 => {
                    sig
                        &= !(((roundBits as i32 ^ 0x40 as i32 == 0)
                            as i32 & roundNearEven as i32)
                            as uint_fast32_t);
                    if sig == 0 {
                        exp = 0 as i32 as int_fast16_t;
                    }
                }
                _ => {}
            }
            uiZ = (((sign as uint32_t) << 31 as i32)
                .wrapping_add((exp as uint32_t) << 23 as i32) as u64)
                .wrapping_add(sig);
        }
        _ => {}
    }
    uZ.ui = uiZ as uint32_t;
    return uZ.f;
}
