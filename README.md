# rust-fastpbkdf2
This is a rust binding for [fastpbkdf2](https://github.com/ctz/fastpbkdf2).

[![Build Status](https://travis-ci.org/ctz/rust-fastpbkdf2.svg)](https://travis-ci.org/ctz/rust-fastpbkdf2)

## Interface

```rust
pub fn pbkdf2_hmac_sha1(password: &[u8], salt: &[u8], iterations: u32, out: &mut[u8]);
pub fn pbkdf2_hmac_sha256(password: &[u8], salt: &[u8], iterations: u32, out: &mut[u8]);
pub fn pbkdf2_hmac_sha512(password: &[u8], salt: &[u8], iterations: u32, out: &mut[u8]);
```

## Performance

Hash   | rust-fastpbkdf2   | [ring](https://github.com/briansmith/ring)  | [rust-crypto](https://github.com/DaGenix/rust-crypto)
-------|--------------|-----------------|----------------
SHA1   | 366ms        | 553ms (1.5x)    | 1077ms (2.9x)
SHA256 | 813ms        | 1021ms (1.25x)  | 2450ms (3.0x)
SHA512 | 1259ms       | 1579ms (1.25x)  | 3114ms (2.5x)

On Intel i3-2100T CPU @ 2.50GHz in 64-bit mode, 2<sup>20</sup> iterations, `--release` build.

## Building and testing

You'll need OpenSSL for `fastpbkdf2`.  `cargo build` builds, `cargo test` runs tests.

Don't forget checking out the `fastpbkdf2` git submodule before building.

Consider enabling CPU-specific optimizations such as [-C target-cpu=native](https://doc.rust-lang.org/rustc/codegen-options/index.html#target-cpu) if the compiled binaries are used locally or on a controlled machine type.

### Linux

OpenSSL libraries are typically available as either `openssl-dev`, `openssl-devel`, `libssl-dev` or similar package names.


### Windows

You'll [need to provide an OpenSSL build to fastpbkdf2](https://github.com/ctz/fastpbkdf2/blob/master/WINDOWS.md#OpenSSL) (inside its submodule).
This is unfortunately manual because it involves interacting with OpenSSL's build system,
or finding suitable binaries.

## License
[CC0](https://creativecommons.org/publicdomain/zero/1.0/).

## Author
Joseph Birr-Pixton <jpixton@gmail.com>
