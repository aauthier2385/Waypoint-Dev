fn main() {}

fn largest<T: PartialOrd + Copy>(items: &[T]) -> T {
    let mut best = items[0];
    for &item in items {
        if item > best {
            best = item;
        }
    }
    best
}

struct Pair<T> {
    first: T,
    second: T,
}

impl<T: PartialOrd + Copy> Pair<T> {
    fn larger(&self) -> T {
        if self.first > self.second {
            self.first
        } else {
            self.second
        }
    }
}

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
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
