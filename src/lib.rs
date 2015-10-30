extern crate libc;
use libc::size_t;

#[cfg(not(windows))]
#[link(name = "fastpbkdf2")]
#[link(name = "crypto")]
extern {}

#[cfg(windows)]
#[link(name = "fastpbkdf2")]
#[link(name = "fastpbkdf2/openssl/lib/libeay32")]
extern {}

extern {
  fn fastpbkdf2_hmac_sha1(pw: *const u8, npw: size_t,
                          salt: *const u8, nsalt: size_t,
                          iterations: u32,
                          out: *mut u8, nout: size_t);
  fn fastpbkdf2_hmac_sha256(pw: *const u8, npw: size_t,
                            salt: *const u8, nsalt: size_t,
                            iterations: u32,
                            out: *mut u8, nout: size_t);
  fn fastpbkdf2_hmac_sha512(pw: *const u8, npw: size_t,
                            salt: *const u8, nsalt: size_t,
                            iterations: u32,
                            out: *mut u8, nout: size_t);
}

pub fn pbkdf2_hmac_sha1(password: &[u8], salt: &[u8], iterations: u32, out: &mut[u8]) {
  assert!(iterations != 0);
  unsafe {
    fastpbkdf2_hmac_sha1(password.as_ptr(), password.len() as size_t,
                         salt.as_ptr(), salt.len() as size_t,
                         iterations,
                         out.as_mut_ptr(), out.len() as size_t);
  }
}

pub fn pbkdf2_hmac_sha256(password: &[u8], salt: &[u8], iterations: u32, out: &mut[u8]) {
  assert!(iterations != 0);
  unsafe {
    fastpbkdf2_hmac_sha256(password.as_ptr(), password.len() as size_t,
                           salt.as_ptr(), salt.len() as size_t,
                           iterations,
                           out.as_mut_ptr(), out.len() as size_t);
  }
}

pub fn pbkdf2_hmac_sha512(password: &[u8], salt: &[u8], iterations: u32, out: &mut[u8]) {
  assert!(iterations != 0);
  unsafe {
    fastpbkdf2_hmac_sha512(password.as_ptr(), password.len() as size_t,
                           salt.as_ptr(), salt.len() as size_t,
                           iterations,
                           out.as_mut_ptr(), out.len() as size_t);
  }
}
