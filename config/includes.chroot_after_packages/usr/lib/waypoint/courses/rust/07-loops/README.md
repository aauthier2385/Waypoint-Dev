# Lesson 07: Repeating things

**What you'll learn**
- How to repeat steps with `for`, `while`, and `loop`
- How to count up through a range of numbers
- How to go through text one letter at a time
- How to keep a running total

## The idea

Computers are great at doing the same thing many times without getting bored. A **loop** repeats a block of code. Rust has three kinds:

| Loop | Use it when | Plain English |
| --- | --- | --- |
| `for` | You know what to go through | "For each thing in this list, do this." |
| `while` | You repeat until something changes | "While this is true, keep going." |
| `loop` | You repeat until you decide to stop | "Keep going forever until I say `break`." |

## `for` loops and ranges

```rust
for number in 1..=5 {
    println!("{number}");
}
```

This prints 1, 2, 3, 4, and 5, each on its own line. `1..=5` is a **range**, meaning every whole number from 1 *through* 5. `1..5`, without the `=`, stops *before* 5.

The example at the top of `src/main.rs` counts down. `.rev()` means *reverse*, so it goes 3, 2, 1.

## Keeping a running total

To add things up inside a loop, start a mutable variable outside it, then add to it each time around:

```rust
let mut total = 0;
for price in [3, 5, 2] {
    total += price;      // same as: total = total + price
}
// total is now 10
```

`+=` is shorthand for "add to what's already there." There's also `-=`, `*=`, and `/=`.

## Going through text letter by letter

`.chars()` splits text into its characters, so a `for` loop can visit each one:

```rust
for letter in "hi!".chars() {
    println!("{letter}");   // prints h, then i, then !
}
```

To check whether a letter is one of several, put them in text and use `.contains`:

```rust
if "aeiou".contains(letter) {
    // letter is a lowercase vowel
}
```

## `while` loops

```rust
let mut fuel = 3;
while fuel > 0 {
    println!("Flying! Fuel left: {fuel}");
    fuel -= 1;
}
```

Rust checks the condition before each round. **Make sure something inside the loop changes the condition**, or the loop will never end. If that happens, the checker stops it and tells you.

## `loop` and `break`

```rust
loop {
    // ...do something...
    if done {
        break;   // leave the loop
    }
}
```

You'll use `loop` in the next lesson's game. `continue` is its partner: it skips the rest of this round and starts the next one.

## Your challenge

Replace each `todo!()`:

1. **`sum_up_to`**: add up every number from 1 to `n`. Use a `for` loop with a range and a running total.
2. **`count_vowels`**: count the vowels in the text, both uppercase and lowercase. Use `.chars()` and `.contains`.
3. **`doublings_to_reach`**: start at `start`, keep doubling it, and count how many doublings it takes to reach `goal` or more. Use a `while` loop.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`.

## Stuck?

- **Each function follows the same pattern:** make a `mut` variable, change it inside the loop, then give it back on the last line with no semicolon.
- **For capitals**, put them in the list too: `"aeiouAEIOU"`.
- **For doublings**, you need two variables: the current amount and the count. The loop should keep going while the amount is less than the goal.

## New words

- **Loop**: code that repeats.
- **Range**: a span of numbers, like `1..=10`.
- **Iterate**: to go through items one at a time, like a `for` loop does.
- **Infinite loop**: a loop that never stops. Usually a mistake.
