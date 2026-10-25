// A struct names a group of related fields.
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// A tuple struct gives a tuple's fields a distinct type and meaning.
#[derive(Debug)]
struct Color(u8, u8, u8);

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    // Struct literals name each field, regardless of the order in the definition.
    let first_user = User {
        active: true,
        username: String::from("river"),
        email: String::from("river@example.com"),
        sign_in_count: 1,
    };
    println!(
        "{} ({}) is active: {}; sign-ins: {}.",
        first_user.username, first_user.email, first_user.active, first_user.sign_in_count
    );

    // Field init shorthand works when the local variable and field have the same name.
    let username = String::from("sage");
    let email = String::from("sage@example.com");
    let second_user = build_user(email, username);
    println!("Created another account: {second_user:#?}");

    // Struct update syntax copies fields not explicitly replaced from another instance.
    // Non-Copy fields such as `String` are moved, so `first_user` is not used afterward.
    let updated_user = User {
        email: String::from("new-address@example.com"),
        ..first_user
    };
    println!("Updated account: {updated_user:#?}");

    let rust_orange = Color(255, 120, 35);
    println!(
        "An RGB color has red={}, green={}, and blue={} components.",
        rust_orange.0, rust_orange.1, rust_orange.2
    );
}
