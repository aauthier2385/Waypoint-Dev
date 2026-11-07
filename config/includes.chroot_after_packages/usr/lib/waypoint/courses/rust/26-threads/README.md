# Lesson 26: Doing things at the same time: threads

**What you'll learn**
- What a thread is, and why programs use several
- Starting threads with `thread::spawn`, and waiting for them with `join`
- Why threads need `move`
- Sending messages between threads with channels
- Sharing data safely with `Arc` and `Mutex`

## The idea: more cooks in the kitchen

A program normally does one thing at a time, in order. But your computer's processor has several **cores**, each able to work independently. Your Ryzen 9 has 12, for example. A **thread** is an independent line of work, and a program with several threads can use several cores at once.

Think of a kitchen. One cook makes dinner step by step. Four cooks can chop, boil, and bake at the same time, and dinner's ready sooner. But they have to coordinate: two cooks grabbing the same knife at once is trouble.

In most languages, that "same knife" problem, two threads changing the same data at once, causes **data races**: random, rare bugs that are notoriously hard to find. **Rust catches data races at compile time.** It's often called "fearless concurrency."

## Starting a thread

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("Hello from another thread!");
    42
});

let answer = handle.join().unwrap();    // wait for it to finish, and get its result
```

`spawn` takes a closure and runs it on a new thread. It gives back a **handle**, and `.join()` waits for the thread to finish. It gives back a `Result`, which is `Err` only if the thread panicked.

## Threads need `move`

A new thread might keep running after the function that started it has ended. So it can't just *borrow* that function's variables, because they might disappear. Use `move` to hand ownership over to the thread:

```rust
let name = String::from("Sam");
let handle = thread::spawn(move || format!("Hi from {name}"));
```

Without `move`, Rust stops with error `E0373`, "closure may outlive the current function."

## Channels: passing notes

One safe way for threads to work together is to send messages, like passing notes through a slot in the door. A **channel** has a sending end and a receiving end:

```rust
use std::sync::mpsc;

let (sender, receiver) = mpsc::channel();

let worker = sender.clone();        // each thread gets its own sender
thread::spawn(move || {
    worker.send(String::from("done chopping")).unwrap();
});

drop(sender);                       // close the original, so the loop below ends
for message in receiver {           // waits for each message until every sender is gone
    println!("{message}");
}
```

`mpsc` stands for *multiple producer, single consumer*: many senders, one receiver.

## Sharing data: `Arc` and `Mutex`

Sometimes threads really do need the same data, like a shared total.

- **`Mutex`**, short for *mutual exclusion*, is a lock. Only one thread can hold it at a time, so only one can change the data at once. It's like the one bathroom key at a gas station.
- **`Arc`** is the thread-safe version of `Rc` from the last lesson. The A stands for *atomic*, which means its count is updated safely even when threads do it at the same moment.

```rust
use std::sync::{Arc, Mutex};

let total = Arc::new(Mutex::new(0));
let mut handles = Vec::new();

for _ in 0..4 {
    let total = Arc::clone(&total);
    handles.push(thread::spawn(move || {
        *total.lock().unwrap() += 1;      // take the key, change it, and the key is returned at the }
    }));
}
for handle in handles {
    handle.join().unwrap();
}
println!("{}", *total.lock().unwrap());   // 4
```

If you try to share a `Rc` or a plain `RefCell` between threads, Rust refuses to build. Their bookkeeping isn't safe for threads, and the compiler knows it. That check is the "fearless" part.

## Your challenge

Open `src/main.rs`.

1. **`greet_from_thread`** doesn't build. Read the `E0373` error and fix it.
2. **`parallel_sum`**: split the numbers into `pieces` parts, add up each part on its own thread, then add up the results.
3. **`count_together`**: start `threads` threads that each add 1 to a shared count `per_thread` times. Give back the final count.
4. **`collect_messages`**: start `workers` threads that each send `"worker N is done"` through a channel. Collect the messages and sort them.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`.

## Stuck?

- **Splitting the work:** `numbers.chunks(size)` gives you slices. Use `size = (numbers.len() + pieces - 1) / pieces`, which rounds up, and `.max(1)`. Each thread needs its own copy, from `chunk.to_vec()`, moved in.
- **The count:** after joining every thread, read it with `*count.lock().unwrap()`.
- **Messages:** after starting the threads, `drop(sender)`, then `receiver.iter().collect::<Vec<String>>()`, then `.sort()`.

## New words

- **Thread**: an independent line of work inside a program.
- **Concurrency**: doing several things during the same period of time.
- **Data race**: two threads changing the same data at once. Rust prevents them.
- **Channel**: a one-way pipe for sending messages between threads.
- **Mutex / Arc**: a lock that lets one thread at a time change data, and a thread-safe shared owner.
