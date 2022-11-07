use crate::*;

pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
pub type int_fast32_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;
pub const softfloat_flag_invalid: C2RustUnnamed_1 = 16;
pub const softfloat_flag_inexact: C2RustUnnamed_1 = 1;
pub const softfloat_round_odd: C2RustUnnamed_0 = 6;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ui: uint32_t,
    pub i: int32_t,
}
pub const softfloat_round_near_even: C2RustUnnamed_0 = 0;
pub const softfloat_round_max: C2RustUnnamed_0 = 3;
pub const softfloat_round_min: C2RustUnnamed_0 = 2;
pub const softfloat_round_near_maxMag: C2RustUnnamed_0 = 4;
pub type C2RustUnnamed_0 = u32;
pub const softfloat_round_minMag: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = u32;
pub const softfloat_flag_infinite: C2RustUnnamed_1 = 8;
pub const softfloat_flag_overflow: C2RustUnnamed_1 = 4;
pub const softfloat_flag_underflow: C2RustUnnamed_1 = 2;
pub unsafe fn softfloat_roundToI32(
    mut sign: bool,
    mut sig: uint_fast64_t,
    mut roundingMode: uint_fast8_t,
    mut exact: bool,
) -> int_fast32_t {
    let mut roundIncrement: uint_fast16_t = 0;
    let mut roundBits: uint_fast16_t = 0;
    let mut sig32: uint_fast32_t = 0;
    let mut uZ: C2RustUnnamed = C2RustUnnamed { ui: 0 };
    let mut z: int_fast32_t = 0;
    roundIncrement = 0x800 as i32 as uint_fast16_t;
    if roundingMode as i32 != softfloat_round_near_maxMag as i32
        && roundingMode as i32 != softfloat_round_near_even as i32
    {
        roundIncrement = 0 as i32 as uint_fast16_t;
        if if sign as i32 != 0 {
            (roundingMode as i32 == softfloat_round_min as i32
                || roundingMode as i32 == softfloat_round_odd as i32)
                as i32
        } else {
            (roundingMode as i32 == softfloat_round_max as i32)
                as i32
        } != 0
        {
            roundIncrement = 0xfff as i32 as uint_fast16_t;
        }
    }
    roundBits = sig & 0xfff as i32 as u64;
    sig = (sig as u64).wrapping_add(roundIncrement) as uint_fast64_t
        as uint_fast64_t;
    if !(sig & 0xfffff00000000000 as u64 != 0) {
        sig32 = sig >> 12 as i32;
        if roundBits == 0x800 as i32 as u64
            && roundingMode as i32 == softfloat_round_near_even as i32
        {
            sig32 &= !(1 as i32 as uint_fast32_t);
        }
        uZ
            .ui = (if sign as i32 != 0 { sig32.wrapping_neg() } else { sig32 })
            as uint32_t;
        z = uZ.i as int_fast32_t;
        if !(z != 0
            && (z < 0 as i32 as i64) as i32
                ^ sign as i32 != 0)
        {
            if roundBits != 0 {
                if roundingMode as i32 == softfloat_round_odd as i32 {
                    z |= 1 as i32 as i64;
                }
                if exact {
                    softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                        | softfloat_flag_inexact as i32) as uint_fast8_t;
                }
            }
            return z;
        }
    }
    softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
    return (if sign as i32 != 0 {
        -(0x7fffffff as i32) - 1 as i32
    } else {
        -(0x7fffffff as i32) - 1 as i32
    }) as int_fast32_t;
}
