fn main() {
    let s = String::from("I am Mrinmoy");
    let (s1, l) = calculate_len(s);
    
    println!("The len of {s1} is {l}");
}

fn calculate_len(s: String) -> (String, usize) {
    let result = s.len();
    (s, result)
}
