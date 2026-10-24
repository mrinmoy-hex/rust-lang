struct User {
    active: bool,
    email: String,
    username: String,
    sign_in_count: u64
}

fn main() {
    let mut user_1 = User {
        email: String::from("mrinmoy@email.com"),
        username: String::from("Mrinmoy"),
        active: true,
        sign_in_count: 0,
    };

    user_1.username = String::from("hello_world"); 
    println!("The new user name is: {}", user_1.username);

    let user_2 = User {
        email: String::from("test@gmail.com"),
        username: String::from("Test"),
        ..user_1
    };
}
