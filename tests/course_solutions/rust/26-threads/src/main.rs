use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {}

fn greet_from_thread(name: String) -> String {
    let handle = thread::spawn(move || format!("Hi from {name}"));
    handle.join().unwrap()
}

fn parallel_sum(numbers: Vec<u64>, pieces: usize) -> u64 {
    let size = ((numbers.len() + pieces - 1) / pieces).max(1);
    let mut handles = Vec::new();
    for chunk in numbers.chunks(size) {
        let chunk = chunk.to_vec();
        handles.push(thread::spawn(move || chunk.iter().sum::<u64>()));
    }
    handles.into_iter().map(|handle| handle.join().unwrap()).sum()
}

fn count_together(threads: usize, per_thread: u32) -> u32 {
    let count = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 0..threads {
        let count = Arc::clone(&count);
        handles.push(thread::spawn(move || {
            for _ in 0..per_thread {
                *count.lock().unwrap() += 1;
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let total = *count.lock().unwrap();
    total
}

fn collect_messages(workers: usize) -> Vec<String> {
    let (sender, receiver) = mpsc::channel();
    for number in 1..=workers {
        let sender = sender.clone();
        thread::spawn(move || {
            sender.send(format!("worker {number} is done")).unwrap();
        });
    }
    drop(sender);
    let mut messages: Vec<String> = receiver.iter().collect();
    messages.sort();
    messages
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
