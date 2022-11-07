use crate::*;

pub type uint_fast8_t = u8;
pub const softfloat_tininess_afterRounding: C2RustUnnamed = 1;
pub type C2RustUnnamed = u32;
pub const softfloat_tininess_beforeRounding: C2RustUnnamed = 0;
pub const softfloat_round_near_even: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = u32;
pub const softfloat_round_odd: C2RustUnnamed_0 = 6;
pub const softfloat_round_near_maxMag: C2RustUnnamed_0 = 4;
pub const softfloat_round_max: C2RustUnnamed_0 = 3;
pub const softfloat_round_min: C2RustUnnamed_0 = 2;
pub const softfloat_round_minMag: C2RustUnnamed_0 = 1;
pub static mut softfloat_roundingMode: uint_fast8_t = softfloat_round_near_even
    as i32 as uint_fast8_t;
pub static mut softfloat_detectTininess: uint_fast8_t = softfloat_tininess_afterRounding
    as i32 as uint_fast8_t;
pub static mut softfloat_exceptionFlags: uint_fast8_t = 0 as i32 as uint_fast8_t;
pub static mut extF80_roundingPrecision: uint_fast8_t = 80 as i32
    as uint_fast8_t;
