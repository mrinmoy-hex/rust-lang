#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }

    // `if let` is concise for one pattern, but unlike `match` it does not handle every case.
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    // Use `match` when both possibilities matter; `None` gets its own clear path.
    let optional_name = None::<&str>;
    match optional_name {
        Some(name) => println!("Hello, {name}!"),
        None => println!("No name was provided, so we'll use a friendly default."),
    }
}
