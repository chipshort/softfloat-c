use crate::*;

pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int64_t = __int64_t;
pub type uint64_t = __uint64_t;
pub type int_fast64_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast64_t = u64;
pub const softfloat_flag_invalid: C2RustUnnamed_1 = 16;
pub const softfloat_flag_inexact: C2RustUnnamed_1 = 1;
pub const softfloat_round_odd: C2RustUnnamed_0 = 6;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ui: uint64_t,
    pub i: int64_t,
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
pub unsafe fn softfloat_roundToI64(
    mut sign: bool,
    mut sig: uint_fast64_t,
    mut sigExtra: uint_fast64_t,
    mut roundingMode: uint_fast8_t,
    mut exact: bool,
) -> int_fast64_t {
    let mut current_block: u64;
    let mut uZ: C2RustUnnamed = C2RustUnnamed { ui: 0 };
    let mut z: int_fast64_t = 0;
    if roundingMode as i32 == softfloat_round_near_maxMag as i32
        || roundingMode as i32 == softfloat_round_near_even as i32
    {
        if 0x8000000000000000 as u64 <= sigExtra {
            current_block = 3280480931206196221;
        } else {
            current_block = 1394248824506584008;
        }
    } else if sigExtra != 0
            && (if sign as i32 != 0 {
                (roundingMode as i32 == softfloat_round_min as i32
                    || roundingMode as i32 == softfloat_round_odd as i32)
                    as i32
            } else {
                (roundingMode as i32 == softfloat_round_max as i32)
                    as i32
            }) != 0
        {
        current_block = 3280480931206196221;
    } else {
        current_block = 1394248824506584008;
    }
    match current_block {
        3280480931206196221 => {
            sig = sig.wrapping_add(1);
            if sig == 0 {
                current_block = 2512296358467353240;
            } else {
                if sigExtra == 0x8000000000000000 as u64
                    && roundingMode as i32
                        == softfloat_round_near_even as i32
                {
                    sig &= !(1 as i32 as uint_fast64_t);
                }
                current_block = 1394248824506584008;
            }
        }
        _ => {}
    }
    match current_block {
        1394248824506584008 => {
            uZ.ui = if sign as i32 != 0 { sig.wrapping_neg() } else { sig };
            z = uZ.i;
            if !(z != 0
                && (z < 0 as i32 as i64) as i32
                    ^ sign as i32 != 0)
            {
                if sigExtra != 0 {
                    if roundingMode as i32 == softfloat_round_odd as i32
                    {
                        z |= 1 as i32 as i64;
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
        _ => {}
    }
    softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
    return if sign as i32 != 0 {
        -(0x7fffffffffffffff as i64) - 1 as i32 as i64
    } else {
        -(0x7fffffffffffffff as i64) - 1 as i32 as i64
    };
}
