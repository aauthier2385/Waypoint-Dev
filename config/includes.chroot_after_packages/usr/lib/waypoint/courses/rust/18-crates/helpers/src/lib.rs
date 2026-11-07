//! A small library crate. Other projects can use it by listing it in their Cargo.toml.

/// Converts Celsius to Fahrenheit.
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

/// Describes a Fahrenheit temperature in one word.
pub fn feels_like(fahrenheit: f64) -> &'static str {
    if fahrenheit < 50.0 {
        "cold"
    } else if fahrenheit < 80.0 {
        "nice"
    } else {
        "hot"
    }
}
