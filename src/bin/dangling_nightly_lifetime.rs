/*
Requires nightly Rust to run

The bug was patched on January 22, 2024 and is no longer
present as of Rust 1.82.0
https://github.com/rust-lang/rust/issues/120217
*/

// #![feature(arbitrary_self_types)]

// trait Static<'a> {
//     fn proof(self: *const Self, s: &'a str) -> &'static str;
// }

// fn bad_cast<'a>(x: *const dyn Static<'static>) -> *const dyn Static<'a> {
//     x as _
// }

// impl Static<'static> for () {
//     fn proof(self: *const Self, s: &'static str) -> &'static str {
//         s
//     }
// }

// fn extend_lifetime(s: &str) -> &'static str {
//     bad_cast(&()).proof(s)
// }

// fn main() {
//     let s = String::from("Hello World");
//     let slice = extend_lifetime(&s);
//     println!("Now it exists: {slice}");
//     drop(s);
//     println!("Now it’s gone: {slice}");
// }

fn main() {}
