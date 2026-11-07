fn main() {}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn double(n: i32) -> i32 {
    n * 2
}

fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

fn minutes_in(hours: u32) -> u32 {
    hours * 60
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding() {
        assert_eq!(add(2, 3), 5, "add(2, 3) should be 5");
    }

    #[test]
    fn doubling() {
        assert_eq!(double(21), 42, "double(21) should be 42");
        assert_eq!(double(-4), -8, "double(-4) should be -8");
    }

    #[test]
    fn greeting() {
        assert_eq!(greet("Sam"), "Hello, Sam!", "greet(\"Sam\") should be \"Hello, Sam!\"");
        assert_eq!(greet("Ana"), "Hello, Ana!", "greet(\"Ana\") should be \"Hello, Ana!\"");
    }

    #[test]
    fn minutes() {
        assert_eq!(minutes_in(3), 180, "minutes_in(3) should be 180");
        assert_eq!(minutes_in(0), 0, "minutes_in(0) should be 0");
    }
}
