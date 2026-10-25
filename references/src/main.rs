fn main() {
    let mut s1 = String::from("hello");
    change(&mut s1);

    // Shared borrows can coexist. Their last use is before the mutable borrow below.
    let read_only_1 = &s1;
    let read_only_2 = &s1;
    println!("Two readers can share the same text: {read_only_1} / {read_only_2}");

    // Only one mutable reference may be active at a time.
    let r1 = &mut s1;
    // let r2 = &mut s1;

    // Multiple immutable references are allowed, but cannot overlap a mutable borrow.

    println!("{},", r1);
}

fn change(some_string: &mut String) {
    some_string.push_str(" , world");
}
