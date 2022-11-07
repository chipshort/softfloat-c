use crate::*;

pub type uint_fast8_t = u8;
pub type uint_fast64_t = u64;
pub type C2RustUnnamed = u32;
pub const softfloat_flag_invalid: C2RustUnnamed = 16;
pub const softfloat_flag_infinite: C2RustUnnamed = 8;
pub const softfloat_flag_overflow: C2RustUnnamed = 4;
pub const softfloat_flag_underflow: C2RustUnnamed = 2;
pub const softfloat_flag_inexact: C2RustUnnamed = 1;
pub unsafe fn softfloat_propagateNaNF64UI(
    mut uiA: uint_fast64_t,
    mut uiB: uint_fast64_t,
) -> uint_fast64_t {
    let mut isSigNaNA: bool = false;
    let mut isSigNaNB: bool = false;
    let mut uiNonsigA: uint_fast64_t = 0;
    let mut uiNonsigB: uint_fast64_t = 0;
    let mut uiMagA: uint_fast64_t = 0;
    let mut uiMagB: uint_fast64_t = 0;
    isSigNaNA = uiA & 0x7ff8000000000000 as u64
        == 0x7ff0000000000000 as u64
        && uiA & 0x7ffffffffffff as u64 != 0;
    isSigNaNB = uiB & 0x7ff8000000000000 as u64
        == 0x7ff0000000000000 as u64
        && uiB & 0x7ffffffffffff as u64 != 0;
    uiNonsigA = uiA | 0x8000000000000 as u64;
    uiNonsigB = uiB | 0x8000000000000 as u64;
    if isSigNaNA as i32 | isSigNaNB as i32 != 0 {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
        if isSigNaNA {
            if !isSigNaNB {
                return if !uiB & 0x7ff0000000000000 as u64
                    == 0 as i32 as u64
                    && uiB & 0xfffffffffffff as u64 != 0
                {
                    uiNonsigB
                } else {
                    uiNonsigA
                };
            }
        } else {
            return if !uiA & 0x7ff0000000000000 as u64
                == 0 as i32 as u64
                && uiA & 0xfffffffffffff as u64 != 0
            {
                uiNonsigA
            } else {
                uiNonsigB
            }
        }
    }
    uiMagA = uiA & 0x7fffffffffffffff as u64;
    uiMagB = uiB & 0x7fffffffffffffff as u64;
    if uiMagA < uiMagB {
        return uiNonsigB;
    }
    if uiMagB < uiMagA {
        return uiNonsigA;
    }
    return if uiNonsigA < uiNonsigB { uiNonsigA } else { uiNonsigB };
}
