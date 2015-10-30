extern crate gcc;

#[cfg(not(windows))]
fn main() {
  gcc::Config::new()
    .file("fastpbkdf2/fastpbkdf2.c")
    .include("fastpbkdf2/")
    .flag("-std=c99")
    .opt_level(3)
    .compile("libfastpbkdf2.a");
}

#[cfg(windows)]
fn main() {
  gcc::Config::new()
    .file("fastpbkdf2\\fastpbkdf2.c")
    .include("fastpbkdf2\\")
    .include("fastpbkdf2\\openssl\\include")
    .opt_level(3)
    .compile("libfastpbkdf2.a");
}
