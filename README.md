# rust-fastpbkdf2
This is a rust binding for [fastpbkdf2](https://github.com/ctz/fastpbkdf2).

[![Build Status](https://travis-ci.org/ctz/rust-fastpbkdf2.svg)](https://travis-ci.org/ctz/rust-fastpbkdf2)

## Interface

```rust
pub fn pbkdf2_hmac_sha1(password: &[u8], salt: &[u8], iterations: u32, out: &mut[u8])
pub fn pbkdf2_hmac_sha256(password: &[u8], salt: &[u8], iterations: u32, out: &mut[u8])
pub fn pbkdf2_hmac_sha512(password: &[u8], salt: &[u8], iterations: u32, out: &mut[u8])
```

## Performance

TODO

## Building and testing

You'll need OpenSSL for `fastpbkdf2`.  `cargo build` builds, `cargo test` runs tests.

## License
[CC0](https://creativecommons.org/publicdomain/zero/1.0/).

## Author
Joseph Birr-Pixton <jpixton@gmail.com>
