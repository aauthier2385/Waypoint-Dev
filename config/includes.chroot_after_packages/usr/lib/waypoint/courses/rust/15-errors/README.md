# Lesson 15: Handling errors with Result

**What you'll learn**
- How Rust programs report that something went wrong, without crashing
- `Ok` and `Err`, the two sides of `Result`
- The `?` operator, which passes errors along in one character
- Making your own error type with an enum

## The idea: a delivery that might fail

`Option` answers "is there a value?" Sometimes you also need to know *why* there isn't one. Turning `"abc"` into a number fails, and the person should hear "abc isn't a number," not just "nothing."

That's what `Result` is for:

```rust
enum Result<T, E> {
    Ok(T),    // it worked, and here's the value
    Err(E),   // it failed, and here's what went wrong
}
```

Think of ordering a package: either it arrives (`Ok`), or you get a note explaining what happened (`Err`). Like `Option`, you have to check which one you got before using it.

## Rust has two kinds of errors

1. **Problems you expect and can handle**, like a missing file, bad typing, or no internet. These use `Result`, and the program carries on.
2. **Bugs that should never happen**, like reaching past the end of a list. These **panic**, which means the program stops. `todo!()` and `.unwrap()` on `None` panic, as you've seen.

Good Rust code uses `Result` for anything a user or the outside world could cause.

## Making a Result

```rust
fn parse_age(text: &str) -> Result<u32, String> {
    match text.trim().parse::<u32>() {
        Ok(age) => Ok(age),
        Err(_) => Err(format!("\"{text}\" is not a number")),
    }
}
```

`.parse::<u32>()` tries to turn text into a `u32` and gives back a `Result`. The `::<u32>` tells it which type to aim for. `Err(_)` means "an error, and I don't need the details." The `_` ignores it.

You've used this already: the guessing game in Lesson 08 used `match input.trim().parse()`.

## The `?` operator

Often a function does several steps that can each fail, and you want to stop at the first failure. Writing a `match` for each step gets long. `?` does it for you:

```rust
fn add_two(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let first: i32 = a.parse()?;     // if this fails, hand back the error right now
    let second: i32 = b.parse()?;
    Ok(first + second)
}
```

Read `?` as: **"If this worked, give me the value. If it failed, stop and pass the error back to whoever called me."** It only works inside a function that itself gives back a `Result`, with an error type that fits.

## Your own error types

For errors that come from your own rules, an enum lists every way things can go wrong:

```rust
#[derive(Debug, PartialEq)]
enum TransferError {
    AmountIsZero,
    NotEnoughMoney,
}
```

Then whoever calls your function can use `match` to handle each case differently, like showing a different message for each.

## Your challenge

Open `src/main.rs` and replace each `todo!()`:

1. **`parse_age`**: turn text into an age. Spaces around it are fine.
   - Not a number: `Err("\"abc\" is not a number")`, quoting the original text.
   - Over 150: `Err("200 is too old")`.
   - Otherwise: `Ok(age)`.
2. **`add_numbers`**: turn two pieces of text into `i32`s and add them. Use `?`.
3. **`transfer`**: give back the new balance after taking out `amount`, or `TransferError::AmountIsZero` if `amount` is 0, or `TransferError::NotEnoughMoney` if it's more than `balance`.
4. **`total_age`**: add up a list of ages written as text, using `parse_age` with `?`. Stop at the first bad one.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`.

## Stuck?

- **`parse_age`:** first `match text.trim().parse::<u32>()`. In the `Ok(age)` arm, check whether `age > 150`.
- **`add_numbers`:** `let first: i32 = a.parse()?;`, then the same for `b`, then `Ok(first + second)`.
- **`total_age`:** start `let mut total = 0;`, loop over `texts`, `total += parse_age(text)?;`, then `Ok(total)`.

## New words

- **Result**: Rust's type for "it worked, with a value" or "it failed, with an error."
- **Ok / Err**: the two variants of `Result`.
- **Panic**: when a program stops because of a bug.
- **Propagate**: pass an error back up to the caller, which is what `?` does.
