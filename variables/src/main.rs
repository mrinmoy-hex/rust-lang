fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // shadowing variables
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // boolean types
    let t = true;

    let f: bool = false;    // with explicit type annotation

    // char types
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{}",heart_eyed_cat);

    // compound types

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // accessing a value (destructuring)
    let (x, y, z) = typ;
    println!("The value of y is: {y}");

    let five_hundred = tup.0;

    // arrays
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    // accessing elements
    let first = a[0];
    let second = a[1];

}
