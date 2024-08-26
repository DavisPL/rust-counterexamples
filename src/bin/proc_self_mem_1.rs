use std::fs::OpenOptions;
use std::io::{Result,Seek,Write};

/*
Works on Linux
*/
pub fn write_to_memory(x: *const i32, value: usize) -> Result<()> {
	    let mut file = OpenOptions::new().write(true).open("/proc/self/mem")?;

	    // Seek to the desired memory address
	    file.seek(std::io::SeekFrom::Start(x as u64))?;

	    // Write the data to the specified memory address
	    file.write_all(&value.to_ne_bytes())?;

	Ok(())
}

fn main() -> Result<()> {
	let x: i32 = 10;
	let value : usize = 100;

	println!("Before calling funciton x is {:?}", x);

	write_to_memory(&x, value)?;

	println!("After calling funciton x is {:?}", x);
	Ok(())


}

