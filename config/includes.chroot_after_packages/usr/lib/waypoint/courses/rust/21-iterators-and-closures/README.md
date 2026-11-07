# Lesson 21: Iterators and closures

**What you'll learn**
- **Closures**: small functions you can write right where you need them
- **Iterators**: a way to process lists step by step, like an assembly line
- The most useful steps: `map`, `filter`, `sum`, `count`, and `collect`
- Passing functions into functions, and giving functions back

## Closures: functions without a name

A **closure** is a short function you write inline, with the inputs between bars:

```rust
let double = |x| x * 2;
println!("{}", double(5));      // 10

let add = |a, b| a + b;
println!("{}", add(2, 3));      // 5
```

Compare with a normal function: `fn double(x: i32) -> i32 { x * 2 }`. A closure skips the name and usually the types, because Rust works them out from how you use it.

**Closures can use variables from around them**, which normal functions can't:

```rust
let tax = 0.08;
let with_tax = |price: f64| price * (1.0 + tax);    // uses tax from outside
```

That's called **capturing**. It's where the name comes from: the closure "closes over" its surroundings.

## Iterators: an assembly line

An **iterator** hands out items one at a time. You've used them: `for word in text.split_whitespace()` loops over an iterator.

Iterators shine when you chain steps together, like stations on an assembly line. Each station does one job and passes items along:

```rust
let scores = vec![55, 92, 78, 30, 88];

let passing: Vec<i32> = scores
    .iter()                      // start the line
    .filter(|s| **s >= 60)       // station 1: keep only passing scores
    .map(|s| s + 5)              // station 2: add 5 bonus points
    .collect();                  // pack the results into a Vec

// passing is [97, 83, 93]
```

Read it top to bottom: "Go through the scores, keep the ones 60 or more, add 5 to each, and collect them."

## The most useful steps

| Step | What it does | Example |
| --- | --- | --- |
| `.map(\|x\| ...)` | Change every item | `.map(\|n\| n * 2)` |
| `.filter(\|x\| ...)` | Keep only items where the closure says `true` | `.filter(\|n\| **n > 0)` |
| `.sum()` | Add them all up | `.sum::<i32>()` |
| `.count()` | Count them | `.count()` |
| `.collect()` | Pack them into a collection | `.collect::<Vec<_>>()` |
| `.any(\|x\| ...)` / `.all(\|x\| ...)` | Is it true for any, or for all? | `.any(\|n\| *n == 0)` |
| `.max()` / `.min()` | The biggest or smallest, as an `Option` | `.max()` |

Nothing happens until a final step like `collect`, `sum`, or `count` asks for results. Until then, the iterator is a plan. That's called being **lazy**, and it means Rust never builds lists in between steps it doesn't need.

## Those extra `*`s

`.iter()` borrows each item, so closures receive references. `filter` borrows *again*, so inside `filter` you'll often see `**n` or a pattern like `|&&n|`. If Rust complains about comparing a reference with a number (`E0308` or `E0277`), add a `*` and try again. You'll get a feel for it with practice.

## Passing functions around

Functions can take closures as inputs:

```rust
fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

apply_twice(|n| n + 3, 10);    // 16
```

`impl Fn(i32) -> i32` means "any function or closure that takes an `i32` and gives back an `i32`." It's the trait idea from Lesson 19 again: `Fn` is the ability "can be called."

Functions can give closures back too. Use `move` so the closure takes ownership of what it captures, because the function's own variables disappear when it ends:

```rust
fn make_adder(amount: i32) -> impl Fn(i32) -> i32 {
    move |x| x + amount
}

let add_ten = make_adder(10);
add_ten(5);    // 15
```

## Your challenge

Open `src/main.rs`. Try to write each one as a chain of iterator steps rather than a `for` loop:

1. **`squares_of_evens`**: keep the even numbers, then square each one.
2. **`total_with_tax`**: add tax to every price, then add them all up. A tax rate of `0.10` means 10%.
3. **`names_starting_with`**: the names that start with a letter, as `String`s.
4. **`count_long_words`**: how many words have at least `min_length` letters.
5. **`apply_twice`**: call `f` on `x`, then call `f` on the result.
6. **`make_multiplier`**: give back a closure that multiplies its input by `factor`.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`.

## Stuck?

- **`squares_of_evens`:** `numbers.iter().filter(|n| *n % 2 == 0).map(|n| n * n).collect()`.
- **`total_with_tax`:** `prices.iter().map(|price| price * (1.0 + tax_rate)).sum()`.
- **`names_starting_with`:** `.filter(|name| name.starts_with(letter))`, then `.map(|name| name.to_string())`.
- **`make_multiplier`:** `move |x| x * factor`.

## New words

- **Closure**: an unnamed function written inline, like `|x| x + 1`.
- **Capture**: a closure using variables from around it.
- **Iterator**: something that hands out items one at a time.
- **Lazy**: does no work until results are asked for.
