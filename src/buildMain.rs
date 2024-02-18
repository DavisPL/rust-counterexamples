// /src/main.rs
extern "C" {
    fn test() -> f32;
}

fn main() {
    println!("{}", unsafe { test() });
}
