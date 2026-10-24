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
}
