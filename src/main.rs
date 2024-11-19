use std::io;

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0 ) * 5.0 / 9.0
}

fn meters_to_miles(meters: f64) -> f64 {
    meters / 1609.34
}
fn miles_to_meters(miles: f64) -> f64 {
    miles * 1609.34
}

fn kilograms_to_pounds(kg: f64) -> f64 {
    kg * 2.20462
}

fn pounds_to_kilograms(lb: f64) -> f64 {
    lb / 2.20462
}
fn main() {
    println!("Unit Converter");
    println!("Choose a conversion type:");
    println!("1: Celsius to Fahrenheit");
    println!("2: Fahrenheit to Celsius");
    println!("3: Meters to Miles");
    println!("4: Miles to Meters");
    println!("5: Kilograms to Pounds");
    println!("6: Pounds to Kilograms");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid choice");
            return;
        }
    };

    println!("Enter the value to convert:");

    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("failed to read line");

    let value: f64 = match value.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid value");
            return;
        }
    };

    let result = match choice {
        1 => celsius_to_fahrenheit(value),
        2 => fahrenheit_to_celsius(value),
        3 => meters_to_miles(value),
        4 => miles_to_meters(value),
        5 => kilograms_to_pounds(value),
        6 => pounds_to_kilograms(value),
        _ => {
            println!("Invalid choice");
            return;
        }
    };

    println!("Converted value: {}", result);
}
