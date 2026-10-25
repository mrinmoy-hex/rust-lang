fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // An `if` expression can initialize a binding; both branches must have the same type.
    let conditon = true;
    let number = if conditon { 5 } else { 6 };

    println!("The value of number is: {number}");

    // Rust checks branches from top to bottom, so put the biggest cutoff first.
    let score = 82;
    let grade = if score >= 90 {
        "A"
    } else if score >= 75 {
        "B"
    } else if score >= 60 {
        "C"
    } else {
        "keep practicing"
    };
    println!("A score of {score} gets: {grade}");

    // Combine simple checks when a decision depends on more than one thing.
    let has_ticket = true;
    let is_member = false;
    let can_enter = has_ticket && is_member;
    if can_enter {
        println!("Welcome in!");
    } else {
        println!("You need a ticket and membership to enter.");
    }
}
