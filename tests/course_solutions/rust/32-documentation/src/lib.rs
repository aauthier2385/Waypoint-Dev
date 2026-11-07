//! Converts temperatures between Celsius and Fahrenheit.

/// Converts a temperature from Celsius to Fahrenheit.
///
/// # Examples
///
/// ```
/// use thermo::celsius_to_fahrenheit;
///
/// assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
/// ```
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

/// Converts a temperature from Fahrenheit to Celsius.
///
/// # Examples
///
/// ```
/// use thermo::fahrenheit_to_celsius;
///
/// assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
/// ```
pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

/// True if water freezes at this Celsius temperature.
///
/// # Examples
///
/// ```
/// use thermo::is_freezing;
///
/// assert!(is_freezing(-5.0));
/// assert!(!is_freezing(20.0));
/// ```
pub fn is_freezing(celsius: f64) -> bool {
    celsius <= 0.0
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
    }

    #[test]
    fn to_celsius() {
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
    }

    #[test]
    fn freezing() {
        assert!(is_freezing(0.0), "0°C counts as freezing");
    }
}
