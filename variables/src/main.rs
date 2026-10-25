fn main() {
    // Bindings are immutable by default. Use `mut` only when a value needs to change.
    let mut score = 5;
    println!("Starting score: {score}");
    score += 1;
    println!("Updated score: {score}");

    // Shadowing creates a new binding with the same name; it can also change the type.
    let measurement = "42";
    let measurement: u32 = measurement.parse().expect("measurement must be a number");
    let measurement = measurement + 8;
    println!("Measurement after parsing and adding 8: {measurement}");

    // A constant must have an explicit type and is never mutable.
    const MAX_ATTEMPTS: u8 = 3;
    println!("Maximum attempts: {MAX_ATTEMPTS}");

    // Scalar types include integers, floating-point numbers, booleans, and characters.
    let temperature_celsius: f64 = 21.5;
    let is_raining = false;
    let favorite_character = '🦀';
    println!(
        "Temperature: {temperature_celsius}°C; raining: {is_raining}; favorite: {favorite_character}"
    );

    // Tuples group values of different types; destructuring binds each position.
    let forecast: (&str, f64, bool) = ("Portland", 18.2, true);
    let (city, temperature, rain_expected) = forecast;
    println!("{city}: {temperature}°C, rain expected: {rain_expected}");
    println!(
        "The forecast city is also available by index: {}",
        forecast.0
    );

    // Arrays have a fixed length and all elements share one type.
    let weekday_temperatures = [17, 19, 20, 18, 16];
    let first_day = weekday_temperatures[0];
    let last_day = weekday_temperatures[weekday_temperatures.len() - 1];
    println!("First and last temperatures: {first_day}°C, {last_day}°C");

    // Array types can include their element type and length.
    let month_days: [u8; 3] = [31, 28, 31];
    println!("Days in the first three months: {month_days:?}");
}
