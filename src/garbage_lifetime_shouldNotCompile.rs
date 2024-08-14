'''
This was patched on March 06, 2024 and is no longer present in Rust 1.78.0
//https://github.com/rust-lang/rust/issues/114728
'''

type Static<'a> = &'static &'a ();
trait Extend<'a> {
    fn extend(self, _: &'a str) -> &'static str;
}
impl<'a> Extend<'a> for Static<'a> {
    fn extend(self, s: &'a str) -> &'static str {
        s
    }
}
fn boom<'a>(arg: Static<'_>) -> impl Extend<'a> {
    arg
}
fn main() {
    let y = boom(&&()).extend(&String::from("temporary"));
    println!("{}", y);
}
