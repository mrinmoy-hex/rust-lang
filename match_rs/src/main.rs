#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(UsState::Alaska) => {
            println!("Hello from Alaska.");
            25
        }
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn add(num1: i32, num2: Option<i32>) -> i32 {
    match num2 {
        Some(i) => num1 + i,
        None => num1,
    }
}

// Match arms must cover every possible value; `_` handles any remaining cases.

fn main() {
    let coin = Coin::Penny;
    println!("Value in {}", value_in_cents(coin));

    let coin_1 = Coin::Quarter(UsState::Alaska);
    println!("Value in {}", value_in_cents(coin_1));

    // `None` is a real case to handle, not an error or a magic number.
    println!("Add result = {}", add(30, Some(20)));
    println!("Adding nothing leaves the total unchanged: {}", add(30, None));
    println!("A dime is worth {} cents.", value_in_cents(Coin::Dime));

    // A catch-all arm handles every dice value not listed explicitly.
    let dice_roll = 3;
    match dice_roll {
        3 => println!("You got a fancy hat 🧢"),
        6 => println!("Your fancy hat got removed ❌"),
        _ => println!("Move {dice_roll} steps."),
    };
}
