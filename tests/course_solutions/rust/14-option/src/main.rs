fn main() {}

fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for number in numbers {
        if number % 2 == 0 {
            return Some(*number);
        }
    }
    None
}

fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

fn phone_for(name: &str) -> Option<String> {
    match name {
        "Sam" => Some("555-0101".to_string()),
        "Lee" => Some("555-0199".to_string()),
        _ => None,
    }
}

fn describe_phone(name: &str) -> String {
    match phone_for(name) {
        Some(number) => format!("{name}: {number}"),
        None => format!("No number for {name}"),
    }
}

fn double_or_zero(maybe: Option<i32>) -> i32 {
    match maybe {
        Some(n) => n * 2,
        None => 0,
    }
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_even() {
        assert_eq!(find_first_even(&[1, 3, 8, 5, 10]), Some(8));
        assert_eq!(find_first_even(&[1, 3, 5]), None, "no even numbers means None");
        assert_eq!(find_first_even(&[]), None, "an empty list means None");
    }

    #[test]
    fn dividing() {
        assert_eq!(safe_divide(10.0, 4.0), Some(2.5));
        assert_eq!(safe_divide(1.0, 0.0), None, "dividing by zero gives None");
    }

    #[test]
    fn phones() {
        assert_eq!(describe_phone("Sam"), "Sam: 555-0101");
        assert_eq!(describe_phone("Lee"), "Lee: 555-0199");
        assert_eq!(describe_phone("Ana"), "No number for Ana");
    }

    #[test]
    fn doubling() {
        assert_eq!(double_or_zero(Some(21)), 42);
        assert_eq!(double_or_zero(None), 0);
    }
}
