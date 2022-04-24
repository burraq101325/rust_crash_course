pub fn run () {
    let mut count = 0;

    // infinite loop
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);

    //     if count == 10 {
    //         break;
    //     }
    // }
    // while loop
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz: {}", count);
        } else if count % 3 == 0 {
            println!("fizz: {}", count);
        }else if count % 5 == 0 {
            println!("Buzz: {}", count);
        }else {
            println!("{}", count);
        }
        count += 1;
    }

    // for loop
    for x in 1..100 {
        if x % 15 == 0 {
            println!("FizzBuzz: {}", x);
        } else if x % 3 == 0 {
            println!("fizz: {}", x);
        }else if x % 5 == 0 {
            println!("Buzz: {}", x);
        }else {
            println!("{}", x);
        }
    
    }
}  