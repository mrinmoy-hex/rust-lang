fn main() {
    let mut s1 = String::from("hello");
    change(&mut s1);

    // cannot create multiple mutable references
    let r1 = &mut s1;
    // let r2 = &mut s1;

    // ok: to have multiple immutable references
    // error: to have mutable and immutable references along 

    println!("{},", r1);
}

fn change(some_string: &mut String) {
    some_string.push_str(" , world");
}
