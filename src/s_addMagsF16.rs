use crate::*;

pub type __uint16_t = u16;
pub type uint16_t = __uint16_t;
pub type int_fast8_t = i8;
pub type int_fast16_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u64;
pub type uint_fast32_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union ui16_f16 {
    pub ui: uint16_t,
    pub f: float16_t,
}
pub const softfloat_flag_inexact: C2RustUnnamed_0 = 1;
pub const softfloat_round_odd: C2RustUnnamed = 6;
pub const softfloat_flag_overflow: C2RustUnnamed_0 = 4;
pub const softfloat_round_max: C2RustUnnamed = 3;
pub const softfloat_round_min: C2RustUnnamed = 2;
pub const softfloat_round_near_even: C2RustUnnamed = 0;
pub type C2RustUnnamed = u32;
pub const softfloat_round_near_maxMag: C2RustUnnamed = 4;
pub const softfloat_round_minMag: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = u32;
pub const softfloat_flag_invalid: C2RustUnnamed_0 = 16;
pub const softfloat_flag_infinite: C2RustUnnamed_0 = 8;
pub const softfloat_flag_underflow: C2RustUnnamed_0 = 2;
pub unsafe fn softfloat_addMagsF16(
    mut uiA: uint_fast16_t,
    mut uiB: uint_fast16_t,
) -> float16_t {
    let mut current_block: u64;
    let mut expA: int_fast8_t = 0;
    let mut sigA: uint_fast16_t = 0;
    let mut expB: int_fast8_t = 0;
    let mut sigB: uint_fast16_t = 0;
    let mut expDiff: int_fast8_t = 0;
    let mut uiZ: uint_fast16_t = 0;
    let mut signZ: bool = false;
    let mut expZ: int_fast8_t = 0;
    let mut sigZ: uint_fast16_t = 0;
    let mut sigX: uint_fast16_t = 0;
    let mut sigY: uint_fast16_t = 0;
    let mut shiftDist: int_fast8_t = 0;
    let mut sig32Z: uint_fast32_t = 0;
    let mut roundingMode: int_fast8_t = 0;
    let mut uZ: ui16_f16 = ui16_f16 { ui: 0 };
    expA = ((uiA >> 10 as i32) as int_fast8_t as i32
        & 0x1f as i32) as int_fast8_t;
    sigA = uiA & 0x3ff as i32 as u64;
    expB = ((uiB >> 10 as i32) as int_fast8_t as i32
        & 0x1f as i32) as int_fast8_t;
    sigB = uiB & 0x3ff as i32 as u64;
    expDiff = (expA as i32 - expB as i32) as int_fast8_t;
    if expDiff == 0 {
        if expA == 0 {
            uiZ = uiA.wrapping_add(sigB);
            current_block = 9038991475839087817;
        } else if expA as i32 == 0x1f as i32 {
            if sigA | sigB != 0 {
                current_block = 11876232331036681714;
            } else {
                uiZ = uiA;
                current_block = 9038991475839087817;
            }
        } else {
            signZ = uiA as uint16_t as i32 >> 15 as i32 != 0;
            expZ = expA;
            sigZ = (0x800 as i32 as u64)
                .wrapping_add(sigA)
                .wrapping_add(sigB);
            if sigZ & 1 as i32 as u64 == 0
                && (expZ as i32) < 0x1e as i32
            {
                sigZ >>= 1 as i32;
                current_block = 18280529711645514348;
            } else {
                sigZ <<= 3 as i32;
                current_block = 11763295167351361500;
            }
        }
    } else {
        signZ = uiA as uint16_t as i32 >> 15 as i32 != 0;
        if (expDiff as i32) < 0 as i32 {
            if expB as i32 == 0x1f as i32 {
                if sigB != 0 {
                    current_block = 11876232331036681714;
                } else {
                    uiZ = (((signZ as uint16_t as i32) << 15 as i32)
                        + ((0x1f as i32 as uint16_t as i32)
                            << 10 as i32) + 0 as i32) as uint_fast16_t;
                    current_block = 9038991475839087817;
                }
            } else if expDiff as i32 <= -(13 as i32) {
                uiZ = ((((signZ as uint16_t as i32) << 15 as i32)
                    + ((expB as uint16_t as i32) << 10 as i32))
                    as u64)
                    .wrapping_add(sigB);
                if expA as u64 | sigA != 0 {
                    current_block = 11237535339883696275;
                } else {
                    current_block = 9038991475839087817;
                }
            } else {
                expZ = expB;
                sigX = sigB | 0x400 as i32 as u64;
                sigY = sigA
                    .wrapping_add(
                        (if expA as i32 != 0 {
                            0x400 as i32 as u64
                        } else {
                            sigA
                        }),
                    );
                shiftDist = (19 as i32 + expDiff as i32) as int_fast8_t;
                current_block = 5529461102203738653;
            }
        } else {
            uiZ = uiA;
            if expA as i32 == 0x1f as i32 {
                if sigA != 0 {
                    current_block = 11876232331036681714;
                } else {
                    current_block = 9038991475839087817;
                }
            } else if 13 as i32 <= expDiff as i32 {
                if expB as u64 | sigB != 0 {
                    current_block = 11237535339883696275;
                } else {
                    current_block = 9038991475839087817;
                }
            } else {
                expZ = expA;
                sigX = sigA | 0x400 as i32 as u64;
                sigY = sigB
                    .wrapping_add(
                        (if expB as i32 != 0 {
                            0x400 as i32 as u64
                        } else {
                            sigB
                        }),
                    );
                shiftDist = (19 as i32 - expDiff as i32) as int_fast8_t;
                current_block = 5529461102203738653;
            }
        }
        match current_block {
            9038991475839087817 => {}
            11876232331036681714 => {}
            _ => {
                match current_block {
                    11237535339883696275 => {
                        roundingMode = softfloat_roundingMode as int_fast8_t;
                        if roundingMode as i32
                            != softfloat_round_near_even as i32
                        {
                            if roundingMode as i32
                                == (if uiZ as uint16_t as i32 >> 15 as i32
                                    != 0
                                {
                                    softfloat_round_min as i32
                                } else {
                                    softfloat_round_max as i32
                                })
                            {
                                uiZ = uiZ.wrapping_add(1);
                                if (uiZ << 1 as i32) as uint16_t as i32
                                    == 0xf800 as i32
                                {
                                    softfloat_raiseFlags(
                                        (softfloat_flag_overflow as i32
                                            | softfloat_flag_inexact as i32) as uint_fast8_t,
                                    );
                                }
                            } else if roundingMode as i32
                                    == softfloat_round_odd as i32
                                {
                                uiZ |= 1 as i32 as u64;
                            }
                        }
                        softfloat_exceptionFlags = (softfloat_exceptionFlags
                            as i32 | softfloat_flag_inexact as i32)
                            as uint_fast8_t;
                        current_block = 9038991475839087817;
                    }
                    _ => {
                        sig32Z = (sigX << 19 as i32)
                            .wrapping_add(sigY << shiftDist as i32);
                        if sig32Z < 0x40000000 as i32 as u64 {
                            expZ -= 1;
                            sig32Z <<= 1 as i32;
                        }
                        sigZ = sig32Z >> 16 as i32;
                        if sig32Z & 0xffff as i32 as u64 != 0 {
                            sigZ |= 1 as i32 as u64;
                            current_block = 11763295167351361500;
                        } else if sigZ & 0xf as i32 as u64 == 0
                                && (expZ as i32) < 0x1e as i32
                            {
                            sigZ >>= 4 as i32;
                            current_block = 18280529711645514348;
                        } else {
                            current_block = 11763295167351361500;
                        }
                    }
                }
            }
        }
    }
    match current_block {
        18280529711645514348 => {
            uiZ = ((((signZ as uint16_t as i32) << 15 as i32)
                + ((expZ as uint16_t as i32) << 10 as i32))
                as u64)
                .wrapping_add(sigZ);
        }
        11763295167351361500 => {
            return softfloat_roundPackToF16(signZ, expZ as int_fast16_t, sigZ);
        }
        11876232331036681714 => {
            uiZ = softfloat_propagateNaNF16UI(uiA, uiB);
        }
        _ => {}
    }
    uZ.ui = uiZ as uint16_t;
    return uZ.f;
}
