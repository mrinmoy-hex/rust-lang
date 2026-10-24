enum IpAddr {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}


fn main() {
    // creating instances of each of the two variants 
    // the variants of the enum are namespaced under its identifier
    let four = IpAddr::V4;
    let six = IpAddr::V6;

    // the name of each enum variant that we define also becomes a function that constructs an instance of the enum
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // option enum
    let some_number = Some(5);
    let another_number = Some(10); // Changed from a char to an integer
    let absent_val: Option<i32> = None;

    // Using unwrap_or with direct values
    let sum = some_number.unwrap_or(4) + another_number.unwrap_or(5);
    println!("Sum: {}", sum); // Output: 15
}
