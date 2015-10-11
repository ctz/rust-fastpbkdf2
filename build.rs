extern crate gcc;

fn main() {
  gcc::Config::new()
    .file("fastpbkdf2/fastpbkdf2.c")
    .include("fastpbkdf2/")
    .flag("-std=c99")
    .opt_level(3)
    .compile("libfastpbkdf2.a");
}
