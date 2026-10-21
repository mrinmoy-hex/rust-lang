/* 
Ownership is a discipline for safely using memory within 

In Rust, it doesn't allow to interpret memory as an array of bytes, for instance.

Variables live in frames. A frame is a mapping from variables to values within a single scope, such as a function.

Frames are organized into a stack of currently-called-functions.

-------------- Ownership rules -------------------

1. Each value in Rust has a variable that's called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

*/

fn main() {
    let x: i32 = 5;
    let y: i32 = x;     // copy
    //

    let s1: String = String::from("hello");
    let s2: String = s1;        // move (not shallow copy)
    
    let str1 = gives_ownership();
    let str2 = String::from("hello");
    let str3 = takes_and_gives_back(str2);
    println!("str1 = {}, str3 = {}", str1, str3);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}


fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
