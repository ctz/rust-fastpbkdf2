// I'd like to have used test::Bencher and `cargo bench` here,
// but it's not usable in current stable rust :(
//
// I'd also like to use std::time::Duration::span, but that's also
// unstable.  FFS!

extern crate time;
use time::SteadyTime;

const ITERATIONS: u32 = 1 << 20;
const PASSWORD: &'static [u8] = b"password";
const SALT: &'static [u8] = b"salt";

fn bench<F>(name: &'static str, f: F)
where
    F: FnOnce(),
{
    let start = SteadyTime::now();
    f();
    let duration = SteadyTime::now() - start;
    println!("{} = {}ms", name, duration.num_milliseconds());
}

// fastpbkdf2 versions
extern crate fastpbkdf2;

fn fastpbkdf2_sha1() {
    let mut out = [0u8; 20];
    fastpbkdf2::pbkdf2_hmac_sha1(PASSWORD, SALT, ITERATIONS, &mut out);
}

fn fastpbkdf2_sha256() {
    let mut out = [0u8; 32];
    fastpbkdf2::pbkdf2_hmac_sha256(PASSWORD, SALT, ITERATIONS, &mut out);
}

fn fastpbkdf2_sha512() {
    let mut out = [0u8; 64];
    fastpbkdf2::pbkdf2_hmac_sha512(PASSWORD, SALT, ITERATIONS, &mut out);
}

// ring versions
extern crate ring;
use ring::pbkdf2 as ring_pbkfd2;
use std::num::NonZero;

fn ring_sha1() {
    let mut out = [0u8; 20];
    ring_pbkfd2::derive(
        ring::pbkdf2::PBKDF2_HMAC_SHA1,
        NonZero::new(ITERATIONS).unwrap(),
        PASSWORD,
        SALT,
        &mut out,
    );
}

fn ring_sha256() {
    let mut out = [0u8; 32];
    ring_pbkfd2::derive(
        ring::pbkdf2::PBKDF2_HMAC_SHA256,
        NonZero::new(ITERATIONS).unwrap(),
        PASSWORD,
        SALT,
        &mut out,
    );
}

fn ring_sha512() {
    let mut out = [0u8; 64];
    ring_pbkfd2::derive(
        ring::pbkdf2::PBKDF2_HMAC_SHA512,
        NonZero::new(ITERATIONS).unwrap(),
        PASSWORD,
        SALT,
        &mut out,
    );
}

// rust-crypto versions
extern crate hmac;
extern crate pbkdf2;
extern crate sha1;
extern crate sha2;
use hmac::Hmac;
use pbkdf2::pbkdf2;
use sha1::Sha1;
use sha2::{Sha256, Sha512};

fn rustcrypto_sha1() {
    let mut out = [0u8; 20];
    let _ = pbkdf2::<Hmac<Sha1>>(PASSWORD, SALT, ITERATIONS, &mut out);
}

fn rustcrypto_sha256() {
    let mut out = [0u8; 32];
    let _ = pbkdf2::<Hmac<Sha256>>(PASSWORD, SALT, ITERATIONS, &mut out);
}

fn rustcrypto_sha512() {
    let mut out = [0u8; 64];
    let _ = pbkdf2::<Hmac<Sha512>>(PASSWORD, SALT, ITERATIONS, &mut out);
}

fn main() {
    bench("fastpbkdf2-sha1", fastpbkdf2_sha1);
    bench("fastpbkdf2-sha256", fastpbkdf2_sha256);
    bench("fastpbkdf2-sha512", fastpbkdf2_sha512);

    bench("ring-sha1", ring_sha1);
    bench("ring-sha256", ring_sha256);
    bench("ring-sha512", ring_sha512);

    bench("rust-crypto-sha1", rustcrypto_sha1);
    bench("rust-crypto-sha256", rustcrypto_sha256);
    bench("rust-crypto-sha512", rustcrypto_sha512);
}
