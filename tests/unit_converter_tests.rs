// tests/unit_converter_tests.rs


use unit_converter::{celsius_to_fahrenheit, fahrenheit_to_celsius, meters_to_miles, miles_to_meters, kilograms_to_pounds, pounds_to_kilograms};

#[test]
fn test_celsius_to_fahrenheit() {
    let result = celsius_to_fahrenheit(0.0);
    assert!((result - 32.0).abs() < 1e-6, "Expected 32.0, got {}", result);
}

#[test]
fn test_fahrenheit_to_celsius() {
    let result = fahrenheit_to_celsius(32.0);
    assert!((result - 0.0).abs() < 1e-6, "Expected 0.0, got {}", result);
}

#[test]
fn test_meters_to_miles() {
    let result = meters_to_miles(1609.34);
    assert!((result - 1.0).abs() < 1e-6, "Expected 1.0, got {}", result);
}

#[test]
fn test_miles_to_meters() {
    let result = miles_to_meters(1.0);
    assert!((result - 1609.34).abs() < 1e-6, "Expected 1609.34, got {}", result);
}

#[test]
fn test_kilograms_to_pounds() {
    let result = kilograms_to_pounds(1.0);
    assert!((result - 2.20462).abs() < 1e-6, "Expected 2.20462, got {}", result);
}

#[test]
fn test_pounds_to_kilograms() {
    let result = pounds_to_kilograms(2.20462);
    assert!((result - 1.0).abs() < 1e-6, "Expected 1.0, got {}", result);
}
