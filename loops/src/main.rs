fn main() {
    // `loop` repeats until a `break`; `break` can return a value from the loop.
    let mut attempts = 0;
    let result = loop {
        attempts += 1;
        if attempts == 3 {
            break format!("finished after {attempts} attempts");
        }
    };
    println!("{result}");

    // A label lets an inner loop stop an enclosing loop.
    let mut searched = false;
    'rows: for row in 0..3 {
        for column in 0..3 {
            println!("Checking row {row}, column {column}");
            if row == 1 && column == 2 {
                searched = true;
                break 'rows;
            }
        }
    }
    println!("Target found: {searched}");

    // `while` is useful when the number of iterations depends on a condition.
    let mut countdown = 3;
    while countdown > 0 {
        println!("{countdown}...");
        countdown -= 1;
    }
    println!("Liftoff!");

    // A `for` loop walks over each element without managing an index.
    let temperatures = [17, 19, 20, 18, 16];
    for temperature in temperatures {
        println!("Today's temperature: {temperature}°C");
    }

    // Ranges are exclusive at the end; use `..=` when the end should be included.
    let sum: i32 = (1..=5).sum();
    println!("Sum of 1 through 5: {sum}");

    // Reverse a range to count down without changing a separate index.
    for number in (1..4).rev() {
        println!("{number}!");
    }
}
