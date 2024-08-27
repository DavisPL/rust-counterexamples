use std::fs::OpenOptions;
use std::io::{Result, Seek, Write};
use std::process;

const RUN_FLAG : bool = false;

pub fn write_to_memory(x: *const i32, value: usize) -> Result<()> {
    let mut file = OpenOptions::new().write(true).open("/proc/self/mem")?;

    // Seek to the desired memory address
    file.seek(std::io::SeekFrom::Start(x as u64))?;

    // Write the data to the specified memory address
    file.write_all(&value.to_ne_bytes())?;

    Ok(())
}

fn main() -> Result<()> {

	if ! RUN_FLAG{
        println!("This code example works on Linux.");
        println!("It performs a memory safety violation in Safe Rust using /proc/self/mem.");
		println!("It updates the value at the given memory location");
        println!("This file has been disabled to prevent any accidental execution.");
        println!("You will need to manuallay change the RUN_FLAG in proc_self_mem_1.rs to true to execute it.");
        process::exit(1);
    }

    let x: i32 = 10;
    let value: usize = 100;

    println!("Before calling funciton x is {:?}", x);

    write_to_memory(&x, value)?;

    println!("After calling funciton x is {:?}", x);
    Ok(())
}
