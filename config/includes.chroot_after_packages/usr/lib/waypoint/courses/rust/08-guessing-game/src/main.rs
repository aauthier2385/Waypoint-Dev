// "use" brings in tools from Rust's standard library, the code that comes with Rust.
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let secret = pick_secret();
    let mut tries = 0;
    println!("I'm thinking of a number from 1 to 100.");

    loop {
        println!("Your guess:");

        // Read one line of typing into the variable called input.
        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input).unwrap_or(0);
        if bytes_read == 0 {
            println!("Goodbye!");
            return;
        }

        // Turn the typed text into a number. If it isn't a number, ask again.
        // You'll learn exactly how match works in the Intermediate level.
        let guess: u32 = match input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please type a whole number.");
                continue;
            }
        };

        if !is_valid_guess(guess) {
            println!("Pick a number from 1 to 100.");
            continue;
        }

        tries += 1;
        println!("{}", compare(guess, secret));

        if guess == secret {
            println!("{}", score_message(tries));
            return;
        }
    }
}

/// Picks a secret number from 1 to 100, using the clock to make it different each game.
fn pick_secret() -> u32 {
    let nanoseconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|time| time.subsec_nanos())
        .unwrap_or(0);
    nanoseconds % 100 + 1
}

/// Gives back true if the guess is from 1 to 100.
fn is_valid_guess(guess: u32) -> bool {
    todo!()
}

/// Gives back "Too low!", "Too high!", or "You got it!"
fn compare(guess: u32, secret: u32) -> String {
    todo!()
}

/// 1 try: "Amazing! First try!"
/// 2 to 6 tries: "Nice work! You took 4 tries." (with the real number)
/// 7 or more: "You got there! It took 9 tries." (with the real number)
fn score_message(tries: u32) -> String {
    todo!()
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
