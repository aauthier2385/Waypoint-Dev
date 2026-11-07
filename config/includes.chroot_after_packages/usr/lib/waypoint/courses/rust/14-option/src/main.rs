fn main() {
    println!("{:?}", find_first_even(&[1, 3, 8, 5, 10]));
    println!("{:?}", safe_divide(10.0, 0.0));
    println!("{}", describe_phone("Sam"));
    println!("{}", describe_phone("Ana"));
    println!("{}", double_or_zero(Some(21)));
}

/// The first even number in the list, or None.
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    todo!()
}

/// a divided by b, or None if b is 0.
fn safe_divide(a: f64, b: f64) -> Option<f64> {
    todo!()
}

/// A tiny phone book. This one is finished.
fn phone_for(name: &str) -> Option<String> {
    match name {
        "Sam" => Some("555-0101".to_string()),
        "Lee" => Some("555-0199".to_string()),
        _ => None,
    }
}

/// "Sam: 555-0101", or "No number for Ana" when there isn't one.
fn describe_phone(name: &str) -> String {
    todo!()
}

/// Twice the number, or 0 if there isn't one.
fn double_or_zero(maybe: Option<i32>) -> i32 {
    todo!()
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
