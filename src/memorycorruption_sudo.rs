use std::alloc::{alloc, dealloc, Layout};
use std::env;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufWriter, Write, Result, Read, Error , Seek};
use std::process::{Command, Stdio};

// a fucnrtion that allocated mmemory in heap, and then returns a pointer to that memory
fn allocate() -> *mut i32 {
    let layout = Layout::new::<i32>();
    let ptr = unsafe {
        alloc(layout) as *mut i32
    };
    ptr
}

pub struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

pub fn write_to_memory(x: *const i32, value: i32) -> Result<()> {
    let pid = std::process::id();
    let address = x as usize;

    let mut child = Command::new("gdb")
        .arg("--pid").arg(pid.to_string())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    {
        // Scope for the mutable borrow of stdin
        let stdin = child.stdin.as_mut().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to open GDB stdin"))?;
        let mut writer = BufWriter::new(stdin);

        // Send commands to GDB through stdin
        writeln!(writer, "set *((int *)0x{:x}) = {}", address, value)?;
        writeln!(writer, "detach")?;
        writeln!(writer, "quit")?;
        writer.flush()?;
    } // The BufWriter and hence the mutable borrow of stdin goes out of scope here

     let mut gdb_output = String::new();
    if let Some(mut stdout) = child.stdout.take() {
        stdout.read_to_string(&mut gdb_output)?;
        println!("GDB Output: {}", gdb_output);
    }

    // Optionally, consume and print GDB's stderr as well
    let mut gdb_err_output = String::new();
    if let Some(mut stderr) = child.stderr.take() {
        stderr.read_to_string(&mut gdb_err_output)?;
        println!("GDB Error Output: {}", gdb_err_output);
    }

    // Wait for the child process to exit
    child.wait()?;


    Ok(())
}

pub struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn insert(&mut self, data: i32) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn display_address(&self){
     let mut current = &self.head;
     while let Some(node)=current{
	println!("{:p}" , &node.data);
	current = &node.next
	}
    }	

    pub fn display(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} ", node.data);
            current = &node.next;
        }
        println!();
    }

    pub fn get_length(&self) -> i32 {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }
        count
    }
    
    pub fn get_address(&self) -> &i32{
    	let mut current = &self.head;
   	if let Some(node) = current {
   		return &node.data
   	}else{
   		todo!{};
   	}
    }
}
/*
fn main() {
    // println!("Hello, world!");
    // easy();
    // normal();
    // bufferOverflow();

    // experiment();

    // A loop that keep allocating memory in heap and then taking input from user to store there, it stops, if user enters -1. 
    // LinkedList ptr; 
    let mut ptr = LinkedList::new();
    let mut input = String::new();
    loop {
        println!("Please choose an option:");
        println!("1. Enter a number to store in Linked List");
        println!("2. Print the elements stored in Linked List");
        println!("3. Exit");
        println!("4. Get Addreeses");
        println!("5. Modify the head");
        println!("6. Experiment");
        io::stdout().flush().expect("Failed to flush stdout");
        input.clear();
	io::stdin().read_line(&mut input).expect("Failed to read line");
        let choice: i32 = input.trim().parse().expect("Please type a number!");
        input.clear(); // Clear the input string
        match choice {
            1 => {
                println!("Please enter a number to store in Linked List:");
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let number: i32 = input.trim().parse().expect("Please type a number!");
                ptr.insert(number);
                input.clear(); // Clear the input string
            },
            2 => {
                ptr.display();
            },
            3 => {
                break;
            },
	    4 => {
		ptr.display_address();
		},
	    5=>{
	        println!("Please enter a number you want to add as replacement:");
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let number: i32 = input.trim().parse().expect("Please type a number!");
                let p : *const i32 = ptr.get_address();
                write_to_memory(p, number);
                ptr.display();
		},
	    6=>{
	    	let memory_address: *const i32 = ptr.get_address();
    		let data_to_write: usize = 42;

    		// Open the /proc/self/mem file for writing
    		let mut file = OpenOptions::new().write(true).open("/proc/self/mem");

    		// Seek to the desired memory address
    		file.seek(std::io::SeekFrom::Start(memory_address as u64));

    		// Write the data to the specified memory address
   		 file.write_all(&data_to_write.to_ne_bytes());

    		println!("Write to memory successful.");
	    
	    },
            _ => {
                println!("Invalid option");
            },
        }
    }


}
*/

fn main() -> io::Result<()> {

	let mut ptr = LinkedList::new();
    let mut input = String::new();
    loop {
        println!("Please choose an option:");
        println!("1. Enter a number to store in Linked List");
        println!("2. Print the elements stored in Linked List");
        println!("3. Get Addreeses");
        println!("4. Modify the head");
        println!("5. Exit");
        io::stdout().flush().expect("Failed to flush stdout");
        input.clear();
	io::stdin().read_line(&mut input).expect("Failed to read line");
        let choice: i32 = input.trim().parse().expect("Please type a number!");
        input.clear(); // Clear the input string
        match choice {
        1 => {
                println!("Please enter a number to store in Linked List:");
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let number: i32 = input.trim().parse().expect("Please type a number!");
                ptr.insert(number);
                input.clear(); // Clear the input string
            },
            2 => {
                ptr.display();
            },
            5 => {
                break;
            },
	    3 => {
		ptr.display_address();
		},
	    4=>{
	        println!("Please enter a number you want to add as replacement:");
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let number: i32 = input.trim().parse().expect("Please type a number!");
                let p : *const i32 = ptr.get_address();
                write_to_memory(p, number);
                ptr.display();
		},
    _=>{
	println!("Invalid Option")
	},
}}
    return Ok(());

}
