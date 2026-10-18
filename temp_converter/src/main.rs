use std::io;

fn main() {
    loop {
        println!("\nTemperature Converter");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");
        println!("q. Quit");
        print!("Choose an option: ");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input.");

        match choice.trim() {
            "1" => convert_f_to_c(),
            "2" => convert_c_to_f(),
            "q" => break,
            _ => println!("Invalid choice. Try again."),
        }
    }
}

fn convert_f_to_c() {
    let fahrenheit = read_temp();

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

    println!("{fahrenheit:.2} °F = {celsius:.2} °C");
}

fn convert_c_to_f() {
    let celsius = read_temp();
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;

    println!("{celsius:.2} °C = {fahrenheit:.2} °F");
}


fn read_temp()-> f64 {
    loop {
        print!("Enter temperature: ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid temperature. Please try again."),
        }
    }
}
