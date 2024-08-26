/*
requires sudo access
uses gdb to update the value at the memory address
*/

use std::process::{Command,Stdio};
use std::io::{Result,Write};
use std::io::{BufWriter, ErrorKind};

pub fn write_to_memory<T : std::fmt::Display >(x: *const T, value: T) -> Result<()> {
    let pid = std::process::id();
    let address = x as usize;

    let mut child = Command::new("gdb")
        .arg("--pid").arg(pid.to_string())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    {
        // Scope for the mutable borrow of stdin
        let stdin = child.stdin.as_mut().ok_or_else(|| std::io::Error::new(ErrorKind::Other, "Failed to open GDB stdin"))?;
        let mut writer = BufWriter::new(stdin);

        // Send commands to GDB through stdin
        writeln!(writer, "set *((int *)0x{:x}) = {}", address, value)?;
        writeln!(writer, "detach")?;
        writeln!(writer, "quit")?;
        writer.flush()?;
    } // The BufWriter and hence the mutable borrow of stdin goes out of scope here

    // Wait for the child process to exit
    child.wait()?;
    
    println!("Value Updated Successfully") ;


    Ok(())
}

fn main() -> Result<()> {
	let x: i32 = 10;
	let value : i32 = 100;

	println!("Before calling funciton x is {:?}", x);

	write_to_memory(&x, value)?;

	println!("After calling funciton x is {:?}", x);
	Ok(())


}

