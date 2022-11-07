use crate::*;

pub type uint_fast8_t = u8;
pub unsafe fn softfloat_raiseFlags(mut flags: uint_fast8_t) {
    softfloat_exceptionFlags = (softfloat_exceptionFlags as i32
        | flags as i32) as uint_fast8_t;
}
