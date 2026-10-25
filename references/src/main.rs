fn main() {
    let mut s1 = String::from("hello");
    change(&mut s1);

    // Only one mutable reference may be active at a time.
    let r1 = &mut s1;
    // let r2 = &mut s1;

    // Multiple immutable references are allowed, but cannot overlap a mutable borrow.

    println!("{},", r1);
}

fn change(some_string: &mut String) {
    some_string.push_str(" , world");
}
