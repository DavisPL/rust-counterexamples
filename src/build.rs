// /build.rs
fn main() {
    cc::Build::new()
        .compiler("/usr/bin/clang-16")
        .file("./src/abc.c")
        .opt_level(0)
        .compile("abc");
}
