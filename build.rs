extern crate gcc;

fn main() {
  gcc::compile_library("libfastpkbdf2.a", &["fastpbkdf2/fastpbkdf2.c"]);
}
