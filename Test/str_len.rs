fn main() {
    let s = String::from("I am Mrinmoy");
    let (s1, l) = calculate_len(s);
    
    println!("The len of {s1} is {l}");

    let borrowed_text = String::from("Borrowing keeps ownership with the caller.");
    let borrowed_length = calculate_len_by_reference(&borrowed_text);
    println!("'{borrowed_text}' has {borrowed_length} bytes.");
}

fn calculate_len(s: String) -> (String, usize) {
    let result = s.len();
    (s, result)
}

// Borrowing lets a function read a String without taking ownership of it.
fn calculate_len_by_reference(s: &str) -> usize {
    s.len()
}
