fn main() {
    println!("{}", largest(&[3, 7, 2]));
    println!("{}", largest(&['q', 'z', 'a']));

    let pair = Pair { first: 2.5, second: 9.1 };
    println!("{}", pair.larger());

    let mut plates: Stack<String> = Stack::new();
    plates.push("blue".to_string());
    plates.push("red".to_string());
    println!("Top plate: {:?}", plates.peek());
}

/// The biggest item. Add the trait bounds this function needs.
fn largest<T>(items: &[T]) -> T {
    todo!()
}

struct Pair<T> {
    first: T,
    second: T,
}

impl<T: PartialOrd + Copy> Pair<T> {
    /// Whichever of first and second is bigger.
    fn larger(&self) -> T {
        todo!()
    }
}

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// Finished, as an example.
    fn new() -> Stack<T> {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        todo!()
    }

    /// Takes the top item off, or None if the stack is empty.
    fn pop(&mut self) -> Option<T> {
        todo!()
    }

    /// Looks at the top item without removing it.
    fn peek(&self) -> Option<&T> {
        todo!()
    }

    fn len(&self) -> usize {
        todo!()
    }

    fn is_empty(&self) -> bool {
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
    fn largest_of_anything() {
        assert_eq!(largest(&[3, 7, 2]), 7);
        assert_eq!(largest(&['q', 'z', 'a']), 'z');
        assert_eq!(largest(&[1.5, -2.0, 0.3]), 1.5);
    }

    #[test]
    fn pairs() {
        assert_eq!(Pair { first: 2, second: 9 }.larger(), 9);
        assert_eq!(Pair { first: 'x', second: 'b' }.larger(), 'x');
    }

    #[test]
    fn stacking_text() {
        let mut stack: Stack<String> = Stack::new();
        assert!(stack.is_empty());
        stack.push("blue".to_string());
        stack.push("red".to_string());
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.peek(), Some(&"red".to_string()), "red was added last, so it's on top");
        assert_eq!(stack.pop(), Some("red".to_string()));
        assert_eq!(stack.pop(), Some("blue".to_string()));
        assert_eq!(stack.pop(), None, "an empty stack gives back None");
    }

    #[test]
    fn stacking_numbers() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.peek(), Some(&2));
        assert_eq!(stack.len(), 2, "peek doesn't remove anything");
    }
}
