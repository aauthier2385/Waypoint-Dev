use std::num::ParseIntError;

fn main() {
    println!("{:?}", parse_age(" 42 "));
    println!("{:?}", parse_age("abc"));
    println!("{:?}", add_numbers("2", "40"));
    println!("{:?}", transfer(50, 80));
    println!("{:?}", total_age(&["30", "12", "oops"]));
}

/// Turns text into an age, or explains what's wrong.
fn parse_age(text: &str) -> Result<u32, String> {
    todo!()
}

/// Adds two numbers written as text. Use ? on each one.
fn add_numbers(a: &str, b: &str) -> Result<i32, ParseIntError> {
    todo!()
}

#[derive(Debug, PartialEq)]
enum TransferError {
    AmountIsZero,
    NotEnoughMoney,
}

/// Gives back the new balance after taking out amount.
fn transfer(balance: u32, amount: u32) -> Result<u32, TransferError> {
    todo!()
}

/// Adds up ages written as text, stopping at the first bad one.
fn total_age(texts: &[&str]) -> Result<u32, String> {
    todo!()
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ages() {
        assert_eq!(parse_age("42"), Ok(42));
        assert_eq!(parse_age("  7 "), Ok(7), "spaces around the number are fine");
        assert_eq!(parse_age("abc"), Err("\"abc\" is not a number".to_string()));
        assert_eq!(parse_age("200"), Err("200 is too old".to_string()));
    }

    #[test]
    fn adding() {
        assert_eq!(add_numbers("2", "40"), Ok(42));
        assert_eq!(add_numbers("-5", "5"), Ok(0));
        assert!(add_numbers("2", "two").is_err(), "\"two\" isn't a number");
    }

    #[test]
    fn transfers() {
        assert_eq!(transfer(50, 20), Ok(30));
        assert_eq!(transfer(50, 50), Ok(0));
        assert_eq!(transfer(50, 0), Err(TransferError::AmountIsZero));
        assert_eq!(transfer(50, 80), Err(TransferError::NotEnoughMoney));
    }

    #[test]
    fn totals() {
        assert_eq!(total_age(&["30", "12"]), Ok(42));
        assert_eq!(total_age(&[]), Ok(0));
        assert_eq!(total_age(&["30", "oops", "x"]), Err("\"oops\" is not a number".to_string()),
                   "it stops at the first bad one");
    }
}
