use std::io;

fn main() {
    println!("Display the nth fibonacci number");

    println!("Enter the position: ");

    let mut pos_in = String::new();

    io::stdin()
        .read_line(&mut pos_in)
        .expect("Failed to read info");

    // let guess: u32 = match pos_in.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    // };

    // Trim whitespace/newlines and parse the string into an i32
    let number: i32 = pos_in
        .trim()
        .parse()
        .expect("Please type a valid number!");

    let result = fib_num(number);

    println!("The required Fibonacci number: {result}");
    
}

fn fib_num(pos_in: i32) -> i32 {
    if pos_in <= 0 {
        return 0;
    } else if pos_in == 1 {
        return 1;
    }
    
    let mut num1 = 0;
    let mut num2 = 1;

    for _ in 2..=pos_in {
        let next_num = num1 + num2;
        num1 = num2;
        num2 = next_num;
    }

    num2
}
