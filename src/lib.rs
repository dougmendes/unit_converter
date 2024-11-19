// src/conversions.rs

pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

pub fn meters_to_miles(meters: f64) -> f64 {
    meters / 1609.34
}

pub fn miles_to_meters(miles: f64) -> f64 {
    miles * 1609.34
}

pub fn kilograms_to_pounds(kg: f64) -> f64 {
    kg * 2.20462
}

pub fn pounds_to_kilograms(lb: f64) -> f64 {
    lb / 2.20462
}
