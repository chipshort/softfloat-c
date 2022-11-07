use crate::*;

pub type uint_fast8_t = u8;
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
pub unsafe fn softfloat_roundToUI64(
    mut sign: bool,
    mut sig: uint_fast64_t,
    mut sigExtra: uint_fast64_t,
    mut roundingMode: uint_fast8_t,
    mut exact: bool,
) -> uint_fast64_t {
    let mut current_block: u64;
    if roundingMode as i32 == softfloat_round_near_maxMag as i32
        || roundingMode as i32 == softfloat_round_near_even as i32
    {
        if 0x8000000000000000 as u64 <= sigExtra {
            current_block = 17530804174226228756;
        } else {
            current_block = 12349973810996921269;
        }
    } else if sign {
        if sig | sigExtra == 0 {
            return 0 as i32 as uint_fast64_t;
        }
        if roundingMode as i32 == softfloat_round_min as i32 {
            current_block = 2767619977013357497;
        } else if roundingMode as i32 == softfloat_round_odd as i32 {
            current_block = 2767619977013357497;
        } else {
            current_block = 12349973810996921269;
        }
    } else if roundingMode as i32 == softfloat_round_max as i32
            && sigExtra != 0
        {
        current_block = 17530804174226228756;
    } else {
        current_block = 12349973810996921269;
    }
    match current_block {
        17530804174226228756 => {
            sig = sig.wrapping_add(1);
            if sig == 0 {
                current_block = 2767619977013357497;
            } else {
                if sigExtra == 0x8000000000000000 as u64
                    && roundingMode as i32
                        == softfloat_round_near_even as i32
                {
                    sig &= !(1 as i32 as uint_fast64_t);
                }
                current_block = 12349973810996921269;
            }
        }
        _ => {}
    }
    match current_block {
        12349973810996921269 => {
            if !(sign as i32 != 0 && sig != 0) {
                if sigExtra != 0 {
                    if roundingMode as i32 == softfloat_round_odd as i32
                    {
                        sig |= 1 as i32 as u64;
                    }
                    if exact {
                        softfloat_exceptionFlags = (softfloat_exceptionFlags
                            as i32 | softfloat_flag_inexact as i32)
                            as uint_fast8_t;
                    }
                }
                return sig;
            }
        }
        _ => {}
    }
    softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
    return if sign as i32 != 0 {
        0xffffffffffffffff as u64
    } else {
        0xffffffffffffffff as u64
    };
}
