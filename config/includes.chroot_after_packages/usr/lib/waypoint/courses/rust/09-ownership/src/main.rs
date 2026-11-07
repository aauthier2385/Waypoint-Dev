fn main() {
    println!("{}", greet_twice());
    let loud = make_loud(String::from("hello"));
    println!("{loud}");
    println!("{}", join_words(String::from("good"), String::from("morning")));
}

/// Builds a greeting. This function takes ownership of the name.
fn make_greeting(name: String) -> String {
    format!("Hi, {name}!")
}

/// Greets Sam twice. It doesn't build yet. Read the error, then fix it.
fn greet_twice() -> String {
    let name = String::from("Sam");
    let first = make_greeting(name);
    let second = make_greeting(name);
    format!("{first} {second}")
}

/// Takes ownership of some text, adds "!!!" to the end, and gives it back.
fn make_loud(text: String) -> String {
    todo!()
}

/// Gives back both words in one String, with a space between them.
fn join_words(first: String, second: String) -> String {
    todo!()
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
