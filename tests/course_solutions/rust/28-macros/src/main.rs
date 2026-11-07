macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

macro_rules! greet {
    () => {
        String::from("Hello, stranger!")
    };
    ($name:expr) => {
        format!("Hello, {}!", $name)
    };
}

macro_rules! string_vec {
    ($($item:expr),* $(,)?) => {
        vec![$($item.to_string()),*]
    };
}

macro_rules! maximum {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($rest:expr),+) => {{
        let first = $x;
        let rest = maximum!($($rest),+);
        if first > rest {
            first
        } else {
            rest
        }
    }};
}

fn main() {}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    #[test]
    fn squares() {
        assert_eq!(square!(4), 16);
        assert_eq!(square!(2 + 3), 25, "2 + 3 is squared as one unit");
    }

    #[test]
    fn greetings() {
        assert_eq!(greet!(), "Hello, stranger!");
        assert_eq!(greet!("Sam"), "Hello, Sam!");
        let name = String::from("Ana");
        assert_eq!(greet!(name), "Hello, Ana!");
    }

    #[test]
    fn string_vectors() {
        let words: Vec<String> = string_vec!["a", "b", "c"];
        assert_eq!(words, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        let empty: Vec<String> = string_vec![];
        assert!(empty.is_empty());
        let trailing: Vec<String> = string_vec!["x", "y",];
        assert_eq!(trailing.len(), 2, "a trailing comma is allowed");
    }

    #[test]
    fn maximums() {
        assert_eq!(maximum!(7), 7);
        assert_eq!(maximum!(3, 9, 4), 9);
        assert_eq!(maximum!(-1, -8), -1);
        assert_eq!(maximum!(2.5, 1.0, 2.75), 2.75);
    }
}
