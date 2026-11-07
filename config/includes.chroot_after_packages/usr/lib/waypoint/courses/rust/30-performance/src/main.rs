use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let numbers: Vec<u32> = (0..20_000).collect();
    let start = Instant::now();
    println!("Duplicates? {} ({:?})", has_duplicates(&numbers), start.elapsed());

    let words = vec!["word"; 20_000];
    let start = Instant::now();
    println!("Joined {} characters ({:?})", join_words(&words).len(), start.elapsed());

    let start = Instant::now();
    println!("fibonacci(32) = {} ({:?})", fibonacci(32), start.elapsed());
}

/// True if any number appears more than once. Correct, but O(n²).
fn has_duplicates(numbers: &[u32]) -> bool {
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            if numbers[i] == numbers[j] {
                return true;
            }
        }
    }
    false
}

/// Every word followed by a space. Correct, but it copies the text over and over.
fn join_words(words: &[&str]) -> String {
    let mut text = String::new();
    for word in words {
        text = format!("{text}{word} ");
    }
    text
}

/// 0, 1, 1, 2, 3, 5, 8, ... Correct, but it repeats work exponentially.
fn fibonacci(n: u32) -> u64 {
    if n < 2 {
        n as u64
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
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
