use std::fs::OpenOptions;
use std::io::{Seek, Write};
use std::process;

const RUN_FLAG: bool = false;

fn write_oob(vector: &Vec<i32>, index: usize, element: i32) {
    let buffer_ptr = vector.as_ptr();
    let ind = buffer_ptr.wrapping_add(index);

    // Open the /proc/self/mem file for writing
    let mut file = OpenOptions::new()
        .write(true)
        .open("/proc/self/mem")
        .expect("Failed to access process memory");

    // Seek to the desired memory address
    file.seek(std::io::SeekFrom::Start(ind as u64))
        .expect("Failed to find the index for insertion");

    // Write the data to the specified memory address
    file.write_all(&element.to_ne_bytes())
        .expect("Failed to write at index");

    // This part is resizing the vector to demonstrate the insertion was successful.
    // A vector has a pointer to its first index and then two variables storing length and capacity respectively.
    // Bound checks are performed by comparing these values, so if we can update these numbers,
    // we can not just write out of bounds, but read as well.
    let vec_ptr: *const usize = vector as *const Vec<i32> as *const usize;
    let capacity_ptr: *const usize = vec_ptr.wrapping_add(2);

    file.seek(std::io::SeekFrom::Start(capacity_ptr as u64))
        .expect("Failed to find length");
    let num = index + 1;
    file.write_all(&num.to_ne_bytes())
        .expect("Failed to update length"); // update the capacity to index+1

    // Print the updated vector element
    println!("I have {:?}", vector[index]);
}

fn main() {
    if !RUN_FLAG {
        println!("This code example works on Linux.");
        println!("It performs a memory safety violation in Safe Rust using /proc/self/mem.");
        println!("It performs and out of bounds read and write.");
        println!("This file has been disabled to prevent any accidental execution.");
        println!("You will need to manuallay change the RUN_FLAG in proc_self_mem_2.rs to true to execute it.");
        process::exit(1);
    }

    let v = vec![1, 2, 3];
    let element: i32 = 1000;

    let index: usize = 10;

    println!("Vector before calling the funciton {:?}", v);

    write_oob(&v, index, element);

    println!("Vector after calling the funciton {:?}", v);
}
