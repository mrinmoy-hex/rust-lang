/*
Ownership is Rust's system for managing values and their lifetimes.

1. Each value has one owner at a time.
2. Assigning a non-`Copy` value to another binding moves ownership.
3. When the owner goes out of scope, the value is dropped.

Types that implement `Copy`, such as `i32`, are copied on assignment instead.
*/

fn main() {
    let x: i32 = 5;
    let y: i32 = x;     // `i32` implements `Copy`, so `x` remains usable.
    //

    let s1: String = String::from("hello");
    let s2: String = s1;        // `String` moves; `s1` cannot be used afterward.

    // `clone` makes a separate heap allocation when you really need two Strings.
    let s3 = s2.clone();
    println!("Copied integer: {y}; moved string: {s2}; cloned string: {s3}");
    
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
