fn main() {}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

fn first_word(text: &str) -> &str {
    match text.find(' ') {
        Some(position) => &text[..position],
        None => text,
    }
}

struct Highlight<'a> {
    text: &'a str,
}

impl<'a> Highlight<'a> {
    fn new(text: &'a str) -> Highlight<'a> {
        Highlight { text }
    }

    fn first_sentence(&self) -> &str {
        match self.text.find('.') {
            Some(position) => &self.text[..position],
            None => self.text,
        }
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
