use std::time::{SystemTime, UNIX_EPOCH};

fn main() {}

fn pick_secret() -> u32 {
    let nanoseconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|time| time.subsec_nanos())
        .unwrap_or(0);
    nanoseconds % 100 + 1
}

fn is_valid_guess(guess: u32) -> bool {
    guess >= 1 && guess <= 100
}

fn compare(guess: u32, secret: u32) -> String {
    if guess < secret {
        "Too low!".to_string()
    } else if guess > secret {
        "Too high!".to_string()
    } else {
        "You got it!".to_string()
    }
}

fn score_message(tries: u32) -> String {
    if tries == 1 {
        "Amazing! First try!".to_string()
    } else if tries <= 6 {
        format!("Nice work! You took {tries} tries.")
    } else {
        format!("You got there! It took {tries} tries.")
    }
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_guesses() {
        assert!(is_valid_guess(1), "1 is a valid guess");
        assert!(is_valid_guess(50), "50 is a valid guess");
        assert!(is_valid_guess(100), "100 is a valid guess");
        assert!(!is_valid_guess(0), "0 is too small");
        assert!(!is_valid_guess(101), "101 is too big");
    }

    #[test]
    fn comparing() {
        assert_eq!(compare(10, 42), "Too low!", "10 is lower than 42");
        assert_eq!(compare(90, 42), "Too high!", "90 is higher than 42");
        assert_eq!(compare(42, 42), "You got it!", "42 matches 42");
    }

    #[test]
    fn scores() {
        assert_eq!(score_message(1), "Amazing! First try!");
        assert_eq!(score_message(2), "Nice work! You took 2 tries.");
        assert_eq!(score_message(6), "Nice work! You took 6 tries.");
        assert_eq!(score_message(7), "You got there! It took 7 tries.");
    }

    #[test]
    fn secret_is_in_range() {
        let secret = pick_secret();
        assert!((1..=100).contains(&secret), "the secret should be from 1 to 100");
    }
}
