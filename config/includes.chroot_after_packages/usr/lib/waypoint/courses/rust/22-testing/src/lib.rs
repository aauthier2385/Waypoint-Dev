//! Three small functions. Each one has a bug. Write tests to find them, then fix them.

/// Gives back "Fizz" for multiples of 3, "Buzz" for multiples of 5,
/// "FizzBuzz" for multiples of both, and otherwise the number itself as text.
pub fn fizzbuzz(n: u32) -> String {
    if n % 3 == 0 {
        "Fizz".to_string()
    } else if n % 5 == 0 {
        "Buzz".to_string()
    } else if n % 15 == 0 {
        "FizzBuzz".to_string()
    } else {
        n.to_string()
    }
}

/// A leap year is divisible by 4, except years divisible by 100,
/// unless they're also divisible by 400. So 2024 and 2000 are leap years, and 1900 isn't.
pub fn is_leap_year(year: u32) -> bool {
    year % 4 == 0
}

/// 90 and up is 'A', 80 and up is 'B', 70 and up is 'C', 60 and up is 'D', and lower is 'F'.
pub fn letter_grade(score: u32) -> char {
    if score > 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 70 {
        'C'
    } else if score >= 60 {
        'D'
    } else {
        'F'
    }
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checker_fizzbuzz() {
        assert_eq!(fizzbuzz(30), "FizzBuzz", "30 is a multiple of both 3 and 5");
        assert_eq!(fizzbuzz(9), "Fizz");
        assert_eq!(fizzbuzz(10), "Buzz");
        assert_eq!(fizzbuzz(1), "1");
    }

    #[test]
    fn checker_leap_years() {
        assert!(is_leap_year(2024));
        assert!(is_leap_year(2000), "2000 is divisible by 400");
        assert!(!is_leap_year(1900), "1900 is divisible by 100 but not 400");
        assert!(!is_leap_year(2023));
    }

    #[test]
    fn checker_grades() {
        assert_eq!(letter_grade(90), 'A', "exactly 90 is an A");
        assert_eq!(letter_grade(89), 'B');
        assert_eq!(letter_grade(60), 'D');
        assert_eq!(letter_grade(0), 'F');
    }
}
