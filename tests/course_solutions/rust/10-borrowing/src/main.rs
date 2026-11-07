fn main() {}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn add_exclamation(text: &mut String) {
    text.push_str("!");
}

fn longest_word_length(text: &str) -> usize {
    let mut longest = 0;
    for word in text.split_whitespace() {
        if word.len() > longest {
            longest = word.len();
        }
    }
    longest
}

fn double_all(numbers: &mut [i32]) {
    for number in numbers.iter_mut() {
        *number *= 2;
    }
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counting_only_borrows() {
        let sentence = String::from("one two three");
        assert_eq!(count_words(&sentence), 3);
        assert_eq!(sentence, "one two three", "the text should still be usable afterward");
    }

    #[test]
    fn exclaiming() {
        let mut text = String::from("Hi");
        add_exclamation(&mut text);
        add_exclamation(&mut text);
        assert_eq!(text, "Hi!!");
    }

    #[test]
    fn longest_word() {
        assert_eq!(longest_word_length("I love Rust programming"), 11);
        assert_eq!(longest_word_length("a bb ccc"), 3);
        assert_eq!(longest_word_length(""), 0, "empty text has no words");
    }

    #[test]
    fn doubling() {
        let mut numbers = [1, 2, -3];
        double_all(&mut numbers);
        assert_eq!(numbers, [2, 4, -6]);
    }
}
