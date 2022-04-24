// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run(){
    let mut hello = String::from("Hello");
    println!("{} {}", hello, hello.len());
    // Push for Chr
    hello.push('W');
    // Push for String
    hello.push_str("orld!");

    println!("{} {}", hello, hello.len());

    // capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'Worl': {}", hello.contains("Worl"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace(){
        println!("{}", word);
    }

    assert_eq!(11, hello.len());
}