// Add a //! comment at the top of this file describing the whole crate.

pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

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
