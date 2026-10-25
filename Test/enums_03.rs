enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let op = Some(String::from("abc"));

    let op_num1 = Some(1);
    let op_num2 = Some(2);

    let result = op_num1.unwrap() + op_num2.unwrap();
    println!("{result}");

    // Enum variants can carry different kinds of data; `match` handles each shape.
    let message = Message::Move { x: 3, y: -2 };
    match message {
        Message::Quit => println!("Close the app."),
        Message::Move { x, y } => println!("Move to ({x}, {y})."),
        Message::Write(text) => println!("Message: {text}"),
        Message::ChangeColor(red, green, blue) => {
            println!("Change color to RGB({red}, {green}, {blue}).");
        }
    }
}
