# Lesson 30: Making programs fast

**What you'll learn**
- Debug builds versus release builds, and why the difference is huge
- How to think about speed with "Big O," explained without math
- How to measure instead of guessing
- Three common slowdowns, and how to fix each one

## Step one: build for speed

`cargo run` makes a **debug** build. It compiles quickly and keeps extra checks, which is perfect while you're writing code. For real use, build in **release** mode:

```
cargo run --release
cargo build --release      # the program goes in target/release/
```

Release builds take longer to compile, but they often run **10 to 100 times faster**, because the compiler spends time optimizing. Always measure speed with `--release`.

## How the work grows: "Big O" in plain English

The most important question about speed isn't how fast something is today, but **how much slower it gets as the input grows**. Programmers describe it with "Big O":

| Name | Meaning | Everyday example |
| --- | --- | --- |
| O(1) | The same time no matter the size | Looking up a word in a `HashMap` |
| O(n) | Twice the data, twice the time | Reading every page of a book once |
| O(n²) | Twice the data, **four times** the time | Comparing every person in a room with every other person |
| O(2ⁿ) | One more item **doubles** the time | Trying every combination of a lock |

An O(n²) program that handles 1,000 items in a blink can take hours on 1,000,000. Picking a better approach beats any amount of fine-tuning.

## Measure, don't guess

```rust
use std::time::Instant;

let start = Instant::now();
do_the_work();
println!("Took {:?}", start.elapsed());
```

For careful comparisons, real projects use **benchmarks**, with the `criterion` crate and `cargo bench`, and **profilers** like `perf`, which show exactly where the time goes. Programmers are famously bad at guessing which part of a program is slow.

## Three common slowdowns

### 1. Comparing everything with everything

```rust
// O(n²): for each number, check every number after it
for i in 0..numbers.len() {
    for j in (i + 1)..numbers.len() {
        if numbers[i] == numbers[j] { return true; }
    }
}
```

Instead, remember what you've seen in a `HashSet`, a `HashMap` with only keys. Checking it is O(1), so the whole job becomes O(n):

```rust
use std::collections::HashSet;

let mut seen = HashSet::new();
for &number in numbers {
    if !seen.insert(number) {     // insert gives back false if it was already there
        return true;
    }
}
```

### 2. Rebuilding text over and over

`text = format!("{text}{word} ");` in a loop makes a brand-new `String` every time, copying everything built so far. That's O(n²) copying. Instead, keep one `String` and add to it with `push_str`. You can even reserve the room up front with `String::with_capacity(size)` or `Vec::with_capacity(size)`.

### 3. Doing the same work twice

The obvious way to compute Fibonacci numbers, where each is the sum of the two before, is `fib(n - 1) + fib(n - 2)`. But each of those calls repeats the other's work, again and again. That's O(2ⁿ). `fib(50)` takes billions of calls. Keeping the last two numbers in a loop makes it O(n), so even `fib(90)` is instant.

## Other speed habits

- **Avoid needless `.clone()`.** Borrow with `&` instead.
- **Prefer iterators.** They're as fast as hand-written loops, and sometimes faster.
- **Use threads** (Lesson 26) for big jobs that split cleanly.

## Your challenge

Open `src/main.rs`. All three functions give the **right answers**, but they're far too slow for big inputs. Rewrite each one so it's fast, keeping the results the same:

1. **`has_duplicates`**: use a `HashSet`.
2. **`join_words`**: build one `String`.
3. **`fibonacci`**: use a loop.

Run `cargo run --release` before and after to see the difference.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`. Each check gives your code a big input and 3 seconds to finish, in a debug build.

## Stuck?

- **`join_words`:** `let mut text = String::new(); for word in words { text.push_str(word); text.push(' '); } text`. Make sure the output has exactly the same spaces as before.
- **`fibonacci`:** start with `let (mut previous, mut current) = (0, 1);`, and loop `n` times, moving both forward.

## New words

- **Release build**: an optimized build, made with `--release`.
- **Big O**: a way of describing how work grows with the size of the input.
- **Benchmark**: a careful measurement of how fast code runs.
- **Profiler**: a tool that shows where a program spends its time.
