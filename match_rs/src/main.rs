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

/*
 *  Matches are exhaustive => the arms' patterns must cover all possibilities
 *
 *
 * */

fn main() {
    let coin = Coin::Penny;
    println!("Value in {}", value_in_cents(coin));

    let coin_1 = Coin::Quarter(UsState::Alaska);
    println!("Value in {}", value_in_cents(coin_1));

    println!("Add result = {}", add(30, Some(20)));

    // dice-rool program
    let dice_roll = 3;
    match dice_roll {
        3 => println!("You got a fancy hat 🧢"),
        6 => println!("Your fancy hat got removed ❌"),
        _ => println!("Move {dice_roll} steps."),
    };
}
