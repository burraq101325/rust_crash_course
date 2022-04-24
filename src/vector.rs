// Fixed list where elements are of same data types
use std::mem::size_of_val;
pub fn run(){
    let mut numbers:Vec<i32> = vec![1,2,3,4];
    
    println!("{:?}", numbers);
    numbers.push(10);
    numbers.push(20);
    println!("{:?}", numbers);

    println!("Length of Vector: {}", numbers.len());

    println!("Vector occupies {} bytes", size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice {:?}", slice);

    // Loop through vector values
    for x in numbers.iter(){
        println!("Number: {}", x);
    }
    // Loop and mutate values
    for x in numbers.iter_mut(){
        *x *= 2;
    }
    println!("Number: {:?}", numbers);

}