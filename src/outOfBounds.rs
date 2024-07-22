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
    // A vector has a pointer to its first index and then two variables storing length and capacity respectively.
    // Bound checks are performed by comparing these values, so if we can update these numbers,
    // we can not just write out of bounds, but read as well.
    let vec_ptr: *const usize = vector as *const Vec<i32> as *const usize;
    let len_ptr: *const usize = vec_ptr.wrapping_add(1) ;
    let capacity_ptr: *const usize =  vec_ptr.wrapping_add(2) ;

    file.seek(std::io::SeekFrom::Start(capacity_ptr as u64))
        .expect("Failed to find length");
    let num = index + 1;
    file.write_all(&num.to_ne_bytes())
        .expect("Failed to update length"); // update the capacity to index+1

    // Print the updated vector element
    println!("I have {:?}", vector[index]);
}

