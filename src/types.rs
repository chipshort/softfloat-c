#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct float16_t {
    pub v: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct float32_t {
    pub v: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct float64_t {
    pub v: u64,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct float128_t {
    pub v: [u64; 2],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct extFloat80M {
    pub signif: u64,
    pub signExp: u16,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct uint128 {
    pub v0: u64,
    pub v64: u64,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct exp8_sig16 {
    pub exp: i8,
    pub sig: u64,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct commonNaN {
    pub sign: bool,
    pub v0: u64,
    pub v64: u64,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct exp16_sig32 {
    pub exp: i64,
    pub sig: u64,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct exp16_sig64 {
    pub exp: i64,
    pub sig: u64,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct exp32_sig128 {
    pub exp: i64,
    pub sig: uint128,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct exp32_sig64 {
    pub exp: i64,
    pub sig: u64,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct uint128_extra {
    pub extra: u64,
    pub v: uint128,
}

impl float16_t {
    pub fn from_bits(v: u16) -> Self {
        Self { v }
    }
    pub fn to_bits(self) -> u16 {
        self.v
    }
}
impl float32_t {
    pub fn from_bits(v: u32) -> Self {
        Self { v }
    }
    pub fn to_bits(self) -> u32 {
        self.v
    }
}
impl float64_t {
    pub fn from_bits(v: u64) -> Self {
        Self { v }
    }
    pub fn to_bits(self) -> u64 {
        self.v
    }
}
