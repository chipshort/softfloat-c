# SoftFloat 3e

This is a Rust conversion of the [Berkeley SoftFloat Release 3e](https://github.com/ucb-bar/berkeley-softfloat-3) library.
It is a direct translation using [c2rust](https://github.com/immunant/c2rust) with a little automatic cleanup in `transpile.sh`.

This crate is mostly useful in contexts where you cannot link the C library directly, like in WebAssembly.
If this is not a concern for you, take a look at [softfloat-wrapper](https://crates.io/crates/softfloat-wrapper) or [softfloat-sys](https://crates.io/crates/softfloat-sys).

## Usage
Just add it as a dependency in your `Cargo.toml`:
```toml
[dependencies]
softfloat = { git = "https://github.com/chipshort/softfloat" }
```

## Code
No manual cleanup has been done. The only hand-written Rust code in this repository is in `src/lib.rs`, which just exports everything else,
and `src/types.rs`, which contains the structs from the C code (like `float32_t`), because `c2rust` generates those separately for every file and that's
very annoying to use.

### Generating latest version
To generate new code, first update the c submodule:
```sh
cd berkeley-softfloat-3
git pull
cd ..
```
For easier dependency management, this repo uses `nix-shell`, so to install all dependencies and transpile the code, just run: 
```shell
nix-shell --run ./transpile.sh
```
If you do not have `nix`, just check the dependencies inside `shell.nix`, make sure to install those and then run `./transpile.sh`

