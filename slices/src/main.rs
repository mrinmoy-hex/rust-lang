// Return a borrowed slice instead of an index, so callers can use the word directly.
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

fn main() {
    let sentence = String::from("hello, Rustacean!");

    // Ranges use a start-inclusive, end-exclusive form: 0..5 selects "hello".
    let greeting = &sentence[0..5];
    let remainder = &sentence[7..];
    println!("Greeting: {greeting}; remainder: {remainder}");

    // A string literal is already a string slice (`&str`).
    let literal: &str = "slices borrow data";
    println!("First word in the literal: {}", first_word(literal));

    // `&str` accepts a slice borrowed from a String as well as a string literal.
    let word = first_word(&sentence);
    println!("First word in the String: {word}");

    // The returned slice borrows from `sentence`; it remains valid while that
    // immutable borrow is in use. Rust prevents conflicting mutation meanwhile.
    println!("The complete sentence is still available: {sentence}");

    // Array slices borrow a consecutive portion of an array.
    let scores = [8, 13, 21, 34, 55];
    let middle_scores = &scores[1..4];
    println!("Middle scores: {middle_scores:?}");
    println!("All scores: {scores:?}");

    // `split_whitespace` handles empty input and Unicode whitespace safely.
    println!(
        "First word of a Unicode sentence: {}",
        first_word("γειά κόσμε")
    );
    println!("First word of empty text: '{}'", first_word(""));
}
