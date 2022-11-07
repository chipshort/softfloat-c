#!/usr/bin/env bash

# run nix-shell first

# build c2rust
pushd c2rust
cargo build --release -p c2rust
popd

# transpile SoftFloat
pushd build
make clean
bear -- make
../c2rust/target/release/c2rust transpile --emit-modules compile_commands.json > output.txt 2>&1
make clean
popd

# copy to src
mv berkeley-softfloat-3/source/*.rs src
mkdir -p src/_8086
# rust modules cannot start with a number, so adding an underscore
mv berkeley-softfloat-3/source/8086/*.rs src/_8086

# remove rust2c annotations (those only work on nightly)
sed -i '/#\[c2rust::.*\]$/d' src/*.rs
sed -i '/#\[c2rust::.*\]$/d' src/_8086/*.rs

# remove libc dependency
sed -i 's/libc::c_schar/i8/g' src/*.rs
sed -i 's/libc::c_schar/i8/g' src/_8086/*.rs
sed -i 's/libc::c_uchar/u8/g' src/*.rs
sed -i 's/libc::c_uchar/u8/g' src/_8086/*.rs
sed -i 's/libc::c_short/i16/g' src/*.rs
sed -i 's/libc::c_short/i16/g' src/_8086/*.rs
sed -i 's/libc::c_ushort/u16/g' src/*.rs
sed -i 's/libc::c_ushort/u16/g' src/_8086/*.rs
sed -i 's/libc::c_int/i32/g' src/*.rs
sed -i 's/libc::c_int/i32/g' src/_8086/*.rs
sed -i 's/libc::c_uint/u32/g' src/*.rs
sed -i 's/libc::c_uint/u32/g' src/_8086/*.rs
sed -i 's/libc::c_longlong/i64/g' src/*.rs
sed -i 's/libc::c_longlong/i64/g' src/_8086/*.rs
sed -i 's/libc::c_ulonglong/u64/g' src/*.rs
sed -i 's/libc::c_ulonglong/u64/g' src/_8086/*.rs
sed -i 's/libc::intmax_t/i64/g' src/*.rs
sed -i 's/libc::intmax_t/i64/g' src/_8086/*.rs
sed -i 's/libc::uintmax_t/u64/g' src/*.rs
sed -i 's/libc::uintmax_t/u64/g' src/_8086/*.rs
sed -i 's/libc::c_long/i64/g' src/*.rs
sed -i 's/libc::c_long/i64/g' src/_8086/*.rs
sed -i 's/libc::c_ulong/u64/g' src/*.rs
sed -i 's/libc::c_ulong/u64/g' src/_8086/*.rs

sed -i '/use ::libc;$/d' src/*.rs
sed -i '/use ::libc;$/d' src/_8086/*.rs

## Use Rust imports instead of extern "C"
# Remove duplicate struct definitions
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct float16_t \{\n    pub v: uint16_t,\n\}//g' src/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct float16_t \{\n    pub v: uint16_t,\n\}//g' src/_8086/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct float32_t \{\n    pub v: uint32_t,\n\}//g' src/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct float32_t \{\n    pub v: uint32_t,\n\}//g' src/_8086/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct float64_t \{\n    pub v: uint64_t,\n\}//g' src/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct float64_t \{\n    pub v: uint64_t,\n\}//g' src/_8086/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct float128_t \{\n    pub v: \[uint64_t; 2\],\n\}//g' src/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct float128_t \{\n    pub v: \[uint64_t; 2\],\n\}//g' src/_8086/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct extFloat80M \{\n    pub signif: uint64_t,\n    pub signExp: uint16_t,\n\}//g' src/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct extFloat80M \{\n    pub signif: uint64_t,\n    pub signExp: uint16_t,\n\}//g' src/_8086/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct uint128 \{\n    pub v0: uint64_t,\n    pub v64: uint64_t,\n\}//g' src/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct uint128 \{\n    pub v0: uint64_t,\n    pub v64: uint64_t,\n\}//g' src/_8086/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct exp8_sig16 \{\n    pub exp: int_fast8_t,\n    pub sig: uint_fast16_t,\n\}//g' src/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct exp8_sig16 \{\n    pub exp: int_fast8_t,\n    pub sig: uint_fast16_t,\n\}//g' src/_8086/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct commonNaN \{\n    pub sign: bool,\n    pub v0: uint64_t,\n    pub v64: uint64_t,\n\}//g' src/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct commonNaN \{\n    pub sign: bool,\n    pub v0: uint64_t,\n    pub v64: uint64_t,\n\}//g' src/_8086/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct exp16_sig32 {\n    pub exp: int_fast16_t,\n    pub sig: uint_fast32_t,\n}//g' src/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct exp16_sig32 {\n    pub exp: int_fast16_t,\n    pub sig: uint_fast32_t,\n}//g' src/_8086/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct exp16_sig64 {\n    pub exp: int_fast16_t,\n    pub sig: uint_fast64_t,\n}//g' src/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct exp16_sig64 {\n    pub exp: int_fast16_t,\n    pub sig: uint_fast64_t,\n}//g' src/_8086/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct exp32_sig128 {\n    pub exp: int_fast32_t,\n    pub sig: uint128,\n}//g' src/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct exp32_sig128 {\n    pub exp: int_fast32_t,\n    pub sig: uint128,\n}//g' src/_8086/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct uint128_extra {\n    pub extra: uint64_t,\n    pub v: uint128,\n}//g' src/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct uint128_extra {\n    pub extra: uint64_t,\n    pub v: uint128,\n}//g' src/_8086/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct exp32_sig64 {\n    pub exp: int_fast32_t,\n    pub sig: uint64_t,\n}//g' src/*.rs
perl -i -p0e 's/#\[derive\(Copy, Clone\)\]\n#\[repr\(C\)\]\npub struct exp32_sig64 {\n    pub exp: int_fast32_t,\n    pub sig: uint64_t,\n}//g' src/_8086/*.rs
# Insert import at the top of all files
shopt -s extglob
sed -i '1 s/^/use crate::*;\n\n/' src/!(lib.rs|types.rs|!(*.rs))
sed -i '1 s/^/use crate::*;\n\n/' src/_8086/!(mod.rs|!(*.rs))
# Remove extern "C" blocks
perl -i -p0e 's/extern "C" \{(.|\n)*?\}\n//g' src/*.rs
perl -i -p0e 's/extern "C" \{(.|\n)*?\}\n//g' src/_8086/*.rs
# Remove extern declarations from functions
sed -i 's/extern "C" //g' src/*.rs
sed -i 's/extern "C" //g' src/_8086/*.rs
sed -i '/#\[no_mangle\]$/d' src/*.rs
sed -i '/#\[no_mangle\]$/d' src/_8086/*.rs
