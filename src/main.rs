use std::io;
use std::io::Write;
use std::fs::{OpenOptions};
use std::io::{Seek};

/*fn write_oob(vector : &Vec<i32> , index : usize , element : i32 ){
	
 let buffer_ptr = vector.as_ptr();
  let ind = buffer_ptr.wrapping_add(index); // get to the address at which we need to make the insertion
  
   // Open the /proc/self/mem file for writing
   let mut file = OpenOptions::new().write(true).open("/proc/self/mem").expect("Failed to access process memory");

	// Seek to the desired memory address
	 file.seek(std::io::SeekFrom::Start(ind as u64)).expect("Failed to find the index for insertion");

    // Write the data to the specified memory address
	file.write_all(&element.to_ne_bytes()).expect("Failed to write at index");
	    
   //This part is resizing the vector to demonstrate the insertion was successful.   	
   // A vector has a pointer to it's first index and than two variables storing capacity and length respectively. Bound checks are performed by comparing these values so if we can update these numbers, we can not just write out of bounds, we can read as well. 
    let vec_ptr: *const usize = vector as *const Vec<i32> as *const usize; 
    let capacity_ptr: *const usize = vec_ptr.wrapping_add(1);
    let len_ptr: *const usize = vec_ptr.wrapping_add(2);
    
     file.seek(std::io::SeekFrom::Start(len_ptr as u64)).expect("Failed to find length");
     let num = index+1;
     file.write_all(&num.to_ne_bytes()).expect("Failed to update length"); // update the length to index+1
     

     println!("I have {:?}" , vector[index]); //this would have caused a panic without updating the length of the vector
     
}*/

fn write_oob(vector: &Vec<i32>, index: usize, element: i32)
{
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
    // A vector has a pointer to its first index and then two variables storing capacity and length respectively.
    // Bound checks are performed by comparing these values, so if we can update these numbers,
    // we can not just write out of bounds, but read as well.
    let vec_ptr: *const usize = vector as *const Vec<i32> as *const usize;
    let capacity_ptr: *const usize = vec_ptr.wrapping_add(1) ;
    let len_ptr: *const usize =  vec_ptr.wrapping_add(2) ;

    file.seek(std::io::SeekFrom::Start(len_ptr as u64))
        .expect("Failed to find length");
    let num = index + 1;
    file.write_all(&num.to_ne_bytes())
        .expect("Failed to update length"); // update the length to index+1

    // Print the updated vector element
    println!("I have {:?}", vector[index]);
}


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
          write_oob(&my_vector, index, element);
          
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
 
