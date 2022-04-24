pub fn run(){
    let x = 5;

    let y = 2.7;
    println!("x is {} and y is {}", x, y);
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    let a = 'a';
    let smile = '\u{1F600}';

    println!("{} {}", a, smile);
}