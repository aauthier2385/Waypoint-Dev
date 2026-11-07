pub fn weather_report(celsius: f64) -> String {
    let fahrenheit = helpers::celsius_to_fahrenheit(celsius);
    format!("{celsius}°C is {fahrenheit:.0}°F, which feels {}", helpers::feels_like(fahrenheit))
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports() {
        assert_eq!(weather_report(30.0), "30°C is 86°F, which feels hot");
        assert_eq!(weather_report(21.0), "21°C is 70°F, which feels nice");
        assert_eq!(weather_report(-5.0), "-5°C is 23°F, which feels cold");
    }
}
