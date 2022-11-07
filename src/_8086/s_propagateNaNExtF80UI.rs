use crate::*;

pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast64_t = u64;

pub type C2RustUnnamed = u32;
pub const softfloat_flag_invalid: C2RustUnnamed = 16;
pub const softfloat_flag_infinite: C2RustUnnamed = 8;
pub const softfloat_flag_overflow: C2RustUnnamed = 4;
pub const softfloat_flag_underflow: C2RustUnnamed = 2;
pub const softfloat_flag_inexact: C2RustUnnamed = 1;
pub unsafe fn softfloat_propagateNaNExtF80UI(
    mut uiA64: uint_fast16_t,
    mut uiA0: uint_fast64_t,
    mut uiB64: uint_fast16_t,
    mut uiB0: uint_fast64_t,
) -> uint128 {
    let mut current_block: u64;
    let mut isSigNaNA: bool = false;
    let mut isSigNaNB: bool = false;
    let mut uiNonsigA0: uint_fast64_t = 0;
    let mut uiNonsigB0: uint_fast64_t = 0;
    let mut uiMagA64: uint_fast16_t = 0;
    let mut uiMagB64: uint_fast16_t = 0;
    let mut uiZ: uint128 = uint128 { v0: 0, v64: 0 };
    isSigNaNA = uiA64 & 0x7fff as i32 as u64
        == 0x7fff as i32 as u64
        && uiA0 & 0x4000000000000000 as u64 == 0
        && uiA0 & 0x3fffffffffffffff as u64 != 0;
    isSigNaNB = uiB64 & 0x7fff as i32 as u64
        == 0x7fff as i32 as u64
        && uiB0 & 0x4000000000000000 as u64 == 0
        && uiB0 & 0x3fffffffffffffff as u64 != 0;
    uiNonsigA0 = uiA0 | 0xc000000000000000 as u64;
    uiNonsigB0 = uiB0 | 0xc000000000000000 as u64;
    if isSigNaNA as i32 | isSigNaNB as i32 != 0 {
        softfloat_raiseFlags(softfloat_flag_invalid as i32 as uint_fast8_t);
        if isSigNaNA {
            if isSigNaNB {
                current_block = 7313909626704344692;
            } else if uiB64 & 0x7fff as i32 as u64
                    == 0x7fff as i32 as u64
                    && uiB0 & 0x7fffffffffffffff as u64 != 0
                {
                current_block = 10221791597126235408;
            } else {
                current_block = 462722342508368721;
            }
        } else if uiA64 & 0x7fff as i32 as u64
                == 0x7fff as i32 as u64
                && uiA0 & 0x7fffffffffffffff as u64 != 0
            {
            current_block = 462722342508368721;
        } else {
            current_block = 10221791597126235408;
        }
    } else {
        current_block = 7313909626704344692;
    }
    match current_block {
        7313909626704344692 => {
            uiMagA64 = uiA64 & 0x7fff as i32 as u64;
            uiMagB64 = uiB64 & 0x7fff as i32 as u64;
            if uiMagA64 < uiMagB64 {
                current_block = 10221791597126235408;
            } else if uiMagB64 < uiMagA64 {
                current_block = 462722342508368721;
            } else if uiA0 < uiB0 {
                current_block = 10221791597126235408;
            } else if uiB0 < uiA0 {
                current_block = 462722342508368721;
            } else if uiA64 < uiB64 {
                current_block = 462722342508368721;
            } else {
                current_block = 10221791597126235408;
            }
        }
        _ => {}
    }
    match current_block {
        462722342508368721 => {
            uiZ.v64 = uiA64;
            uiZ.v0 = uiNonsigA0;
            return uiZ;
        }
        _ => {
            uiZ.v64 = uiB64;
            uiZ.v0 = uiNonsigB0;
            return uiZ;
        }
    };
}
