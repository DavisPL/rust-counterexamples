use std::io;
use std::io::Write;
use std::fs::{File, OpenOptions};
use std::io::{Seek, Read, SeekFrom};
use std::mem;

fn main() -> io::Result<()> {
    const VECTOR_SIZE: usize = 10;
    let mut my_vector: Vec<i32> = vec![0; VECTOR_SIZE];

    // Get the memory address of the vector's data
    let vector_ptr = my_vector.as_ptr();

    loop {
        println!("Menu:");
        println!("1. Insert an element at a specific index");
        println!("2. Print the current state of the vector");
        println!("3. Print the memory address of the vector");
        println!("4. Write out of bounds xD");
        println!("5. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
            insert_element(&mut my_vector)
            },
            2 => print_vector(&my_vector),
            3 => println!("Memory address of the vector: {:?}", vector_ptr),
            4 => {
            print!("Enter the index to insert at (0 to 9): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let index: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid index.");
            return Ok(());
        }
    };

        print!("Enter the element to insert: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let element: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid element.");
                return Ok(());
            }
        };
        
        let ind = vector_ptr.wrapping_add(index);
         println!("Memory address of the vector: {:?}", vector_ptr);
        println!("The required index is 0x{:?}" , ind);
        
        // Open the /proc/self/mem file for writing
	    let mut file = OpenOptions::new().write(true).open("/proc/self/mem")?;

	    // Seek to the desired memory address
	    file.seek(std::io::SeekFrom::Start(ind as u64))?;

	    // Write the data to the specified memory address
	    file.write_all(&element.to_ne_bytes())?;
	    
	    println!("Write Successfull, Now printing the vector");
            
            }
            5 => {
                println!("Exiting the program. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please enter a valid option."),
        }
    }
        return Ok(());
}

fn bytes_to_decimal(bytes: &[u8]) -> u32 {
    bytes.iter().fold(0, |acc, &byte| acc * 256 + u32::from(byte))
}

fn insert_element(vector: &mut Vec<i32>) {
    print!("Enter the index to insert at (0 to 9): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let index: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid index.");
            return;
        }
    };

        print!("Enter the element to insert: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let element: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid element.");
                return;
            }
        };

        vector[index] = element;
        println!("Element {} inserted at index {}", element, index);
  
}

fn print_vector(vector: &Vec<i32>) {
    println!("Current state of the vector: {:?}", vector);
}
 
