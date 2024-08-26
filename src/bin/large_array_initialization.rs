/*
https://github.com/rust-lang/rust/issues/121868
*/
static VALS: [u8; 1024 * 1024 * 1024 * 4] = [0u8; 1024 * 1024 * 1024 * 4];

fn main() {
    for i in 0..2048 {
        println!("{}", VALS[i]);
        if VALS[i] != 0 {
            println!("NON-ZERO: {}", VALS[i]);
            return;
        }
    }
}
