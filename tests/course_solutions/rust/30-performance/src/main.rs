use std::collections::HashSet;
use std::time::Instant;

fn main() {}

fn has_duplicates(numbers: &[u32]) -> bool {
    let mut seen = HashSet::new();
    for &number in numbers {
        if !seen.insert(number) {
            return true;
        }
    }
    false
}

fn join_words(words: &[&str]) -> String {
    let mut text = String::new();
    for word in words {
        text.push_str(word);
        text.push(' ');
    }
    text
}

fn fibonacci(n: u32) -> u64 {
    let (mut previous, mut current) = (0u64, 1u64);
    for _ in 0..n {
        let next = previous + current;
        previous = current;
        current = next;
    }
    previous
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    /// Runs the work on another thread and fails if it takes longer than 3 seconds.
    fn within_three_seconds<T: Send + 'static>(work: impl FnOnce() -> T + Send + 'static) -> T {
        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || {
            let _ = sender.send(work());
        });
        receiver.recv_timeout(Duration::from_secs(3)).expect("too slow: it didn't finish within 3 seconds")
    }

    #[test]
    fn duplicates_are_still_right() {
        assert!(has_duplicates(&[1, 2, 3, 2]));
        assert!(!has_duplicates(&[1, 2, 3]));
        assert!(!has_duplicates(&[]));
    }

    #[test]
    fn duplicates_are_fast() {
        let mut numbers: Vec<u32> = (0..200_000).collect();
        assert!(!within_three_seconds({ let numbers = numbers.clone(); move || has_duplicates(&numbers) }));
        numbers.push(5);
        assert!(within_three_seconds(move || has_duplicates(&numbers)));
    }

    #[test]
    fn joining_is_still_right() {
        assert_eq!(join_words(&["a", "b"]), "a b ");
        assert_eq!(join_words(&[]), "");
    }

    #[test]
    fn joining_is_fast() {
        let text = within_three_seconds(|| join_words(&vec!["word"; 200_000]));
        assert_eq!(text.len(), 1_000_000);
    }

    #[test]
    fn fibonacci_is_still_right() {
        let first: Vec<u64> = (0..10).map(fibonacci).collect();
        assert_eq!(first, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn fibonacci_is_fast() {
        assert_eq!(within_three_seconds(|| fibonacci(90)), 2_880_067_194_370_816_120);
    }
}
