// This program is split across several files, and some connections are missing.
// Read the lesson, then connect them.

mod shapes;

fn main() {
    println!("{}", shapes::describe_square(3));
    println!("{}", greetings::hello("Sam"));
    println!("Biggest: {}", math::stats::biggest(&[4, 9, 2]));
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::greetings::hello;
    use crate::math::stats::{biggest, smallest};
    use crate::shapes;

    #[test]
    fn squares() {
        assert_eq!(shapes::describe_square(3), "A 3 by 3 square has an area of 9");
    }

    #[test]
    fn greeting() {
        assert_eq!(hello("Sam"), "Hello, Sam!");
    }

    #[test]
    fn statistics() {
        assert_eq!(biggest(&[4, 9, 2]), 9);
        assert_eq!(smallest(&[4, 9, 2]), 2);
        assert_eq!(smallest(&[-3, 5]), -3);
    }
}
