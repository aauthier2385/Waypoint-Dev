fn main() {
    let mut sentence = String::from("Rust is fun");
    println!("{} words", count_words(&sentence));

    add_exclamation(&mut sentence);
    println!("{sentence}");
    println!("Longest word: {} letters", longest_word_length(&sentence));

    let mut scores = [3, 7, 10];
    double_all(&mut scores);
    println!("{scores:?}");
}

/// Counts the words. Right now this takes ownership of the text.
/// Change it so it borrows instead.
fn count_words(text: String) -> usize {
    text.split_whitespace().count()
}

/// Adds "!" to the end of the text, changing the original.
fn add_exclamation(text: &mut String) {
    todo!()
}

/// Gives back the number of letters in the longest word.
fn longest_word_length(text: &str) -> usize {
    todo!()
}

/// Doubles every number in the list, changing the original list.
fn double_all(numbers: &mut [i32]) {
    todo!()
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
