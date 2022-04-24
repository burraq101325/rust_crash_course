pub fn run(){
    // Print to console
    println!("Hello, world!");
    // Positional Argument
    println!(
    "{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "Code"
    );
    // Place holder for positional arguments
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
    // Place holder for debug trait
    println!("{:?}", (12, true, "hello"));
}