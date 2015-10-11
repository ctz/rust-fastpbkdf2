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

Hash   | fastpbkdf2   | ring            | rust-crypto
-------|--------------|-----------------|----------------
SHA1   | 380ms        | 5299ms (13.9x)  | 41015ms (108x)
SHA256 | 855ms        | 8244ms (9.6x)  | 71521ms (84x)
SHA512 | 1329ms       | 15172ms (11.4x) | 81378ms (61x)

On Intel i3-2100T CPU @ 2.50GHz, 2<sup>20</sup> iterations, 64-bit mode.

## Building and testing

You'll need OpenSSL for `fastpbkdf2`.  `cargo build` builds, `cargo test` runs tests.

## License
[CC0](https://creativecommons.org/publicdomain/zero/1.0/).

## Author
Joseph Birr-Pixton <jpixton@gmail.com>
