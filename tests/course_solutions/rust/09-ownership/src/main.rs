fn main() {}

fn make_greeting(name: String) -> String {
    format!("Hi, {name}!")
}

fn greet_twice() -> String {
    let name = String::from("Sam");
    let first = make_greeting(name.clone());
    let second = make_greeting(name);
    format!("{first} {second}")
}

fn make_loud(mut text: String) -> String {
    text.push_str("!!!");
    text
}

fn join_words(first: String, second: String) -> String {
    format!("{first} {second}")
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_twice() {
        assert_eq!(greet_twice(), "Hi, Sam! Hi, Sam!");
    }

    #[test]
    fn loud() {
        assert_eq!(make_loud(String::from("hello")), "hello!!!");
        assert_eq!(make_loud(String::new()), "!!!", "empty text becomes just !!!");
    }

    #[test]
    fn ownership_comes_back() {
        let text = String::from("wow");
        let text = make_loud(text);
        assert_eq!(text, "wow!!!");
    }

    #[test]
    fn joining() {
        assert_eq!(join_words(String::from("good"), String::from("morning")), "good morning");
    }
}
