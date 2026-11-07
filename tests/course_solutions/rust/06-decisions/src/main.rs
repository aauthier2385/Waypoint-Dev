fn main() {}

fn temperature_word(degrees: i32) -> String {
    if degrees < 50 {
        "cold".to_string()
    } else if degrees < 80 {
        "nice".to_string()
    } else {
        "hot".to_string()
    }
}

fn is_even(number: i32) -> bool {
    number % 2 == 0
}

fn ticket_price(age: u32) -> u32 {
    if age < 5 {
        0
    } else if age <= 12 {
        8
    } else if age <= 64 {
        12
    } else {
        9
    }
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temperatures() {
        assert_eq!(temperature_word(30), "cold", "30 degrees should be cold");
        assert_eq!(temperature_word(49), "cold", "49 degrees should be cold");
        assert_eq!(temperature_word(50), "nice", "50 degrees should be nice");
        assert_eq!(temperature_word(79), "nice", "79 degrees should be nice");
        assert_eq!(temperature_word(80), "hot", "80 degrees should be hot");
        assert_eq!(temperature_word(-10), "cold", "-10 degrees should be cold");
    }

    #[test]
    fn even_numbers() {
        assert!(is_even(4), "4 is even");
        assert!(is_even(0), "0 is even");
        assert!(!is_even(7), "7 is not even");
        assert!(is_even(-2), "-2 is even");
    }

    #[test]
    fn tickets() {
        assert_eq!(ticket_price(3), 0, "a 3-year-old gets in free");
        assert_eq!(ticket_price(5), 8, "a 5-year-old pays the child price");
        assert_eq!(ticket_price(12), 8, "a 12-year-old pays the child price");
        assert_eq!(ticket_price(13), 12, "a 13-year-old pays the adult price");
        assert_eq!(ticket_price(64), 12, "a 64-year-old pays the adult price");
        assert_eq!(ticket_price(65), 9, "a 65-year-old pays the senior price");
    }
}
