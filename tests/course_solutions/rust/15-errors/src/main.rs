use std::num::ParseIntError;

fn main() {}

fn parse_age(text: &str) -> Result<u32, String> {
    match text.trim().parse::<u32>() {
        Ok(age) if age > 150 => Err(format!("{age} is too old")),
        Ok(age) => Ok(age),
        Err(_) => Err(format!("\"{text}\" is not a number")),
    }
}

fn add_numbers(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let first: i32 = a.parse()?;
    let second: i32 = b.parse()?;
    Ok(first + second)
}

#[derive(Debug, PartialEq)]
enum TransferError {
    AmountIsZero,
    NotEnoughMoney,
}

fn transfer(balance: u32, amount: u32) -> Result<u32, TransferError> {
    if amount == 0 {
        return Err(TransferError::AmountIsZero);
    }
    if amount > balance {
        return Err(TransferError::NotEnoughMoney);
    }
    Ok(balance - amount)
}

fn total_age(texts: &[&str]) -> Result<u32, String> {
    let mut total = 0;
    for text in texts {
        total += parse_age(text)?;
    }
    Ok(total)
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
