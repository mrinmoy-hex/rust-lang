// A function can return its final expression without writing `return`.
fn f(x: i32) -> i32 { x + 1 }

fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measurement(5, 'h');

    // The final expression in a block becomes that block's value.
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    println!("{}", f({
        let y = 1;
        y + 1
    }));
}

fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
