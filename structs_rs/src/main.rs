// Structs in rust are custom data types that group related data together under a single name

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // creating an instance of the User struct 
        let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };


    let user2 = User {
        active: true,
        username: String::from("mrinmoy-hex"),
        email: String::from("someone@outlook.com"),
        sign_in_count: 1,
    };
    
    println!("The name of user is {}",user1.username);

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, 
        email,
        sign_in_count: 1,
    }
}
