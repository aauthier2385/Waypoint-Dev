fn main() {}

fn initials(full_name: &str) -> String {
    let mut result = String::new();
    for word in full_name.split_whitespace() {
        result.push_str(&word[..1].to_uppercase());
    }
    result
}

fn title_case(text: &str) -> String {
    let mut result = String::new();
    for word in text.split_whitespace() {
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(&word[..1].to_uppercase());
        result.push_str(&word[1..]);
    }
    result
}

fn censor(text: &str, word: &str) -> String {
    text.replace(word, &"*".repeat(word.len()))
}

fn is_palindrome(text: &str) -> bool {
    let mut cleaned = String::new();
    for letter in text.to_lowercase().chars() {
        if letter != ' ' {
            cleaned.push(letter);
        }
    }
    let reversed: String = cleaned.chars().rev().collect();
    cleaned == reversed
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
