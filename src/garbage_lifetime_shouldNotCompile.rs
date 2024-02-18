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
