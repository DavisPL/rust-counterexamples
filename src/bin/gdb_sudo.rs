/*
requires sudo access
uses gdb to update the value at the memory address
*/

use std::io::{BufWriter, ErrorKind};
use std::io::{Result, Write};
use std::process;
use std::process::{Command, Stdio};

const RUN_FLAG : bool = false;

pub fn write_to_memory<T: std::fmt::Display>(x: *const T, value: T) -> Result<()> {
    let pid = std::process::id();
    let address = x as usize;

    let mut child = Command::new("gdb")
        .arg("--pid")
        .arg(pid.to_string())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    {
        // Scope for the mutable borrow of stdin
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| std::io::Error::new(ErrorKind::Other, "Failed to open GDB stdin"))?;
        let mut writer = BufWriter::new(stdin);

        // Send commands to GDB through stdin
        writeln!(writer, "set *((int *)0x{:x}) = {}", address, value)?;
        writeln!(writer, "detach")?;
        writeln!(writer, "quit")?;
        writer.flush()?;
    } // The BufWriter and hence the mutable borrow of stdin goes out of scope here

    // Wait for the child process to exit
    child.wait()?;

    println!("Value Updated Successfully");

    Ok(())
}

fn main() -> Result<()> {

    if ! RUN_FLAG{
        println!("This code example requires gdb and sudo privileges.");
        println!("This code spawns a new process using gdb, attaches to the existing process, and updates memory at a given location");
        println!("The file has been disabled to prevent any accidental execution.");
        println!("You will need to manuallay change the RUN_FLAG in gdb_sudo.rs to true to execute it.");
        process::exit(1);
    }

    let x: i32 = 10;
    let value: i32 = 100;

    println!("Before calling funciton x is {:?}", x);

    write_to_memory(&x, value)?;

    println!("After calling funciton x is {:?}", x);
    Ok(())
}
