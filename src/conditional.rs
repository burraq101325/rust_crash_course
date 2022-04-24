pub fn run(){
    let number = 10;
    if number < 0 {
        println!("{} is negative", number);
    } else if number > 0 {
        println!("{} is positive", number);
    } else {
        println!("{} is zero", number);
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);
}   