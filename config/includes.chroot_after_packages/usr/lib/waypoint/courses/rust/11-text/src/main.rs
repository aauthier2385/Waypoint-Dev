fn main() {
    println!("{}", initials("Ada Lovelace"));
    println!("{}", title_case("hello big world"));
    println!("{}", censor("darn it, darn", "darn"));
    println!("{}", is_palindrome("Never odd or even"));
}

/// "Ada Lovelace" gives back "AL".
fn initials(full_name: &str) -> String {
    todo!()
}

/// "hello big world" gives back "Hello Big World".
fn title_case(text: &str) -> String {
    todo!()
}

/// Replaces every appearance of word with the same number of stars.
fn censor(text: &str, word: &str) -> String {
    todo!()
}

/// True if the text reads the same both ways, ignoring capitals and spaces.
fn is_palindrome(text: &str) -> bool {
    todo!()
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn making_initials() {
        assert_eq!(initials("Ada Lovelace"), "AL");
        assert_eq!(initials("grace brewster murray hopper"), "GBMH", "lowercase names still give capitals");
    }

    #[test]
    fn making_titles() {
        assert_eq!(title_case("hello big world"), "Hello Big World");
        assert_eq!(title_case("rust"), "Rust");
        assert_eq!(title_case("  extra   spaces  "), "Extra Spaces", "extra spaces are dropped");
    }

    #[test]
    fn censoring() {
        assert_eq!(censor("darn it, darn", "darn"), "**** it, ****");
        assert_eq!(censor("all clean", "darn"), "all clean");
    }

    #[test]
    fn palindromes() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("Never odd or even"));
        assert!(!is_palindrome("Rust"));
    }
}
