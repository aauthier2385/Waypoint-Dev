fn main() {
    println!("{}", longest("apple", "fig"));
    println!("{}", first_word("hello world"));

    let book = String::from("Call me Ishmael. Some years ago, never mind how long.");
    let highlight = Highlight::new(&book);
    println!("{}", highlight.first_sentence());
}

/// Gives back whichever text is longer. The first one wins a tie.
fn longest(a: &str, b: &str) -> &str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

/// The text up to the first space.
fn first_word(text: &str) -> &str {
    todo!()
}

/// Points at part of a longer text, without copying it.
struct Highlight {
    text: &str,
}

impl Highlight {
    fn new(text: &str) -> Highlight {
        Highlight { text }
    }

    /// The text before the first period.
    fn first_sentence(&self) -> &str {
        todo!()
    }
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longer_text() {
        assert_eq!(longest("apple", "fig"), "apple");
        assert_eq!(longest("hi", "there"), "there");
        assert_eq!(longest("same", "size"), "same", "the first one wins a tie");
    }

    #[test]
    fn the_answer_lives_as_long_as_both_inputs() {
        let outer = String::from("a long sentence");
        let answer;
        {
            let inner = String::from("short");
            answer = longest(&outer, &inner).to_string();
        }
        assert_eq!(answer, "a long sentence");
    }

    #[test]
    fn first_words() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("single"), "single");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn highlights() {
        let book = String::from("Call me Ishmael. Some years ago.");
        let highlight = Highlight::new(&book);
        assert_eq!(highlight.first_sentence(), "Call me Ishmael");
        assert_eq!(Highlight::new("No period here").first_sentence(), "No period here");
    }
}
