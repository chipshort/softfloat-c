use crate::*;

pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;
pub const softfloat_flag_invalid: C2RustUnnamed_0 = 16;
pub const softfloat_flag_inexact: C2RustUnnamed_0 = 1;
pub const softfloat_round_odd: C2RustUnnamed = 6;
pub const softfloat_round_near_even: C2RustUnnamed = 0;
pub const softfloat_round_max: C2RustUnnamed = 3;
pub const softfloat_round_min: C2RustUnnamed = 2;
pub const softfloat_round_near_maxMag: C2RustUnnamed = 4;
pub type C2RustUnnamed = u32;
pub const softfloat_round_minMag: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = u32;
pub const softfloat_flag_infinite: C2RustUnnamed_0 = 8;
pub const softfloat_flag_overflow: C2RustUnnamed_0 = 4;
pub const softfloat_flag_underflow: C2RustUnnamed_0 = 2;
pub unsafe fn softfloat_roundToUI32(
    mut sign: bool,
    mut sig: uint_fast64_t,
    mut roundingMode: uint_fast8_t,
    mut exact: bool,
) -> uint_fast32_t {
    let mut current_block: u64;
    let mut roundIncrement: uint_fast16_t = 0;
    let mut roundBits: uint_fast16_t = 0;
    let mut z: uint_fast32_t = 0;
    roundIncrement = 0x800 as i32 as uint_fast16_t;
    if roundingMode as i32 != softfloat_round_near_maxMag as i32
        && roundingMode as i32 != softfloat_round_near_even as i32
    {
        roundIncrement = 0 as i32 as uint_fast16_t;
        if sign {
            if sig == 0 {
                return 0 as i32 as uint_fast32_t;
            }
            if roundingMode as i32 == softfloat_round_min as i32 {
                current_block = 1274484373872403519;
            } else if roundingMode as i32 == softfloat_round_odd as i32 {
                current_block = 1274484373872403519;
            } else {
                current_block = 2868539653012386629;
            }
        } else {
            if roundingMode as i32 == softfloat_round_max as i32 {
                roundIncrement = 0xfff as i32 as uint_fast16_t;
            }
            current_block = 2868539653012386629;
        }
    } else {
        current_block = 2868539653012386629;
    }
    match current_block {
        2868539653012386629 => {
            roundBits = sig & 0xfff as i32 as u64;
            sig = (sig as u64).wrapping_add(roundIncrement) as uint_fast64_t
                as uint_fast64_t;
            if !(sig & 0xfffff00000000000 as u64 != 0) {
                z = sig >> 12 as i32;
                if roundBits == 0x800 as i32 as u64
                    && roundingMode as i32
                        == softfloat_round_near_even as i32
                {
                    z &= !(1 as i32 as uint_fast32_t);
                }
                if !(sign as i32 != 0 && z != 0) {
                    if roundBits != 0 {
                        if roundingMode as i32
                            == softfloat_round_odd as i32
                        {
                            z |= 1 as i32 as u64;
                        }
                        if exact {
                            softfloat_exceptionFlags = (softfloat_exceptionFlags
                                as i32 | softfloat_flag_inexact as i32)
                                as uint_fast8_t;
                        }
                    }
                    return z;
                }
            }
        }
        _ => {}
    }
    softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
    return (if sign as i32 != 0 {
        0xffffffff as u32
    } else {
        0xffffffff as u32
    }) as uint_fast32_t;
}
