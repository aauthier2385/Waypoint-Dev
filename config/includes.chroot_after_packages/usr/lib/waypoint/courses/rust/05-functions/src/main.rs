fn main() {
    println!("2 + 3 = {}", add(2, 3));
    println!("Double 21 is {}", double(21));
    println!("{}", greet("Sam"));
    println!("3 hours is {} minutes", minutes_in(3));
}

/// Adds two numbers. This one is finished, as an example.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Gives back the number times two.
fn double(n: i32) -> i32 {
    todo!()
}

/// Gives back a greeting, like "Hello, Sam!"
fn greet(name: &str) -> String {
    todo!()
}

/// Gives back how many minutes are in this many hours.
fn minutes_in(hours: u32) -> u32 {
    todo!()
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
