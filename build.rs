extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/jpleph.cpp")
        .include("src/")
        .compile("jpl");
}
