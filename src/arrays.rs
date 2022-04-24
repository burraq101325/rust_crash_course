// Fixed list where elements are of same data types
use std::mem::size_of_val;
pub fn run(){
    let numbers:[i32; 4] = [1,2,3,4];
    
    println!("{:?}", numbers);

    println!("Length of array: {}", numbers.len());
    // Arrays are stack allocated
    println!("Array occupies {} bytes", size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice {:?}", slice);
}