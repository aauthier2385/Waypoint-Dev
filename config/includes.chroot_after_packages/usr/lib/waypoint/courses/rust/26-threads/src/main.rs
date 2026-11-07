use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("{}", greet_from_thread(String::from("Sam")));
    let numbers: Vec<u64> = (1..=1_000_000).collect();
    println!("Sum: {}", parallel_sum(numbers, 4));
    println!("Count: {}", count_together(4, 1000));
    println!("{:?}", collect_messages(3));
}

/// Builds a greeting on another thread. It doesn't build yet: read the error.
fn greet_from_thread(name: String) -> String {
    let handle = thread::spawn(|| format!("Hi from {name}"));
    handle.join().unwrap()
}

/// Adds up the numbers, splitting the work across pieces threads.
fn parallel_sum(numbers: Vec<u64>, pieces: usize) -> u64 {
    todo!()
}

/// threads threads each add 1 to a shared count, per_thread times.
fn count_together(threads: usize, per_thread: u32) -> u32 {
    todo!()
}

/// workers threads each send "worker N is done". Gives back the messages, sorted.
fn collect_messages(workers: usize) -> Vec<String> {
    todo!()
}

// ---------------------------------------------------------------
// The checker. You don't need to change anything below this line.
// ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting() {
        assert_eq!(greet_from_thread(String::from("Sam")), "Hi from Sam");
    }

    #[test]
    fn summing_in_parallel() {
        let numbers: Vec<u64> = (1..=10_000).collect();
        assert_eq!(parallel_sum(numbers.clone(), 4), 50_005_000);
        assert_eq!(parallel_sum(numbers, 3), 50_005_000, "it still works when the pieces aren't even");
        assert_eq!(parallel_sum(vec![5], 8), 5, "more threads than numbers is fine");
    }

    #[test]
    fn counting_together() {
        assert_eq!(count_together(4, 1000), 4000, "no additions are lost");
        assert_eq!(count_together(1, 5), 5);
    }

    #[test]
    fn messages() {
        assert_eq!(collect_messages(3), vec!["worker 1 is done", "worker 2 is done", "worker 3 is done"]);
        assert!(collect_messages(0).is_empty());
    }
}
