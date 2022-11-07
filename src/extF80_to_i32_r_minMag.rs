use crate::*;

pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type int_fast32_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast32_t = u64;
pub type uint_fast64_t = u64;

pub type extFloat80_t = extFloat80M;
pub type C2RustUnnamed = u32;
pub const softfloat_flag_invalid: C2RustUnnamed = 16;
pub const softfloat_flag_infinite: C2RustUnnamed = 8;
pub const softfloat_flag_overflow: C2RustUnnamed = 4;
pub const softfloat_flag_underflow: C2RustUnnamed = 2;
pub const softfloat_flag_inexact: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub s: extFloat80M,
    pub f: extFloat80_t,
}
pub unsafe fn extF80_to_i32_r_minMag(
    mut a: extFloat80_t,
    mut exact: bool,
) -> int_fast32_t {
    let mut uA: C2RustUnnamed_0 = C2RustUnnamed_0 {
        s: extFloat80_t {
            signif: 0,
            signExp: 0,
        },
    };
    let mut uiA64: uint_fast16_t = 0;
    let mut exp: int_fast32_t = 0;
    let mut sig: uint_fast64_t = 0;
    let mut shiftDist: int_fast32_t = 0;
    let mut sign: bool = false;
    let mut absZ: int_fast32_t = 0;
    uA.f = a;
    uiA64 = uA.s.signExp as uint_fast16_t;
    exp = (uiA64 & 0x7fff as i32 as u64) as int_fast32_t;
    sig = uA.s.signif;
    shiftDist = 0x403e as i32 as i64 - exp;
    if 64 as i32 as i64 <= shiftDist {
        if exact as i32 != 0 && exp as u64 | sig != 0 {
            softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                | softfloat_flag_inexact as i32) as uint_fast8_t;
        }
        return 0 as i32 as int_fast32_t;
    }
    sign = uiA64 as uint16_t as i32 >> 15 as i32 != 0;
    if shiftDist < 33 as i32 as i64 {
        if uiA64
            == (1 as i32 as uint_fast16_t) << 15 as i32
                | 0x401e as i32 as u64
            && sig < 0x8000000100000000 as u64
        {
            if exact as i32 != 0 && sig & 0xffffffff as u64 != 0 {
                softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
                    | softfloat_flag_inexact as i32) as uint_fast8_t;
            }
            return (-(0x7fffffff as i32) - 1 as i32) as int_fast32_t;
        }
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
        return (if exp == 0x7fff as i32 as i64
            && sig & 0x7fffffffffffffff as u64 != 0
        {
            -(0x7fffffff as i32) - 1 as i32
        } else if sign as i32 != 0 {
            -(0x7fffffff as i32) - 1 as i32
        } else {
            -(0x7fffffff as i32) - 1 as i32
        }) as int_fast32_t;
    }
    absZ = (sig >> shiftDist) as int_fast32_t;
    if exact as i32 != 0 && (absZ as uint_fast32_t) << shiftDist != sig {
        softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
            | softfloat_flag_inexact as i32) as uint_fast8_t;
    }
    return if sign as i32 != 0 { -absZ } else { absZ };
}
