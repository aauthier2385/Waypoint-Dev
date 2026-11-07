# Lesson 22: Writing your own tests

**What you'll learn**
- How to write tests that check your own code
- Unit tests and integration tests, and where each one lives
- `assert!`, `assert_eq!`, `assert_ne!`, and `#[should_panic]`
- How to use tests to find and fix bugs

## The idea: a safety net

Every lesson so far had a checker: tests someone wrote for you. In real projects, **you** write them. Tests are small programs that run your code and confirm the answers are right.

Why bother, when you could just run the program and look?

- **Tests check everything in seconds**, every time, without you clicking through anything.
- **Tests catch "I fixed one thing and broke another."** Run them after every change.
- **Tests make you think about tricky cases**: zero, empty text, the biggest number, the exact edge of a range. That's where most bugs live.

Think of a safety net under a trapeze artist. It doesn't do the trick for you, but it lets you try bold changes without fear.

## Writing a test

A test is a function marked `#[test]`. Inside, you **assert**, which means "this had better be true":

```rust
#[test]
fn adding_works() {
    assert_eq!(add(2, 3), 5);                  // these two must be equal
    assert_ne!(add(2, 3), 6);                  // these must NOT be equal
    assert!(add(2, 2) > 3, "should be over 3");  // this must be true
}
```

The text at the end is optional. It's shown if the test fails, so make it say what you expected.

**Checking for crashes on purpose.** If your code *should* panic, for example given an empty list, mark the test `#[should_panic]`:

```rust
#[test]
#[should_panic]
fn empty_list_panics() {
    average(&[]);
}
```

## Where tests live

| Kind | Where | Tests... |
| --- | --- | --- |
| **Unit tests** | At the bottom of the same file, inside `#[cfg(test)] mod tests { ... }` | Small pieces from the inside, including private functions |
| **Integration tests** | Their own files in a `tests/` folder next to `src/` | Your library from the outside, the way someone else would use it |

`#[cfg(test)]` means "only build this when testing," so tests never end up in your finished program.

An integration test file uses your library by its crate name, just like Lesson 18. This package is named `grading`:

```rust
// tests/my_tests.rs
use grading::fizzbuzz;

#[test]
fn three_is_fizz() {
    assert_eq!(fizzbuzz(3), "Fizz");
}
```

Integration tests can only reach `pub` items, which is exactly what outside users would see.

## Running tests

```
cargo test              # run every test
cargo test leap         # run only tests with "leap" in their name
```

Cargo lists each test as `ok` or `FAILED`, and shows details for any that fail.

## Test-driven fixing

A good way to fix a bug:

1. **Write a test that shows the bug.** It should fail.
2. **Fix the code.**
3. **Run the test again.** It passes, and now that bug can never come back without you knowing.

## Your challenge

`src/lib.rs` has three functions, and **each one has a bug**. Read the `///` comment above each to learn what it's supposed to do.

1. **Make a new file:** `tests/my_tests.rs`. Make the `tests` folder first, next to `src`, not inside it.
2. **Write at least 5 tests of your own** in it. Aim for cases that might reveal the bugs.
3. **Run `cargo test`** and read what fails.
4. **Fix the three bugs** in `src/lib.rs`.

The checker at the bottom of `src/lib.rs` also tests the functions, but try to find each bug with your own tests before you look at it.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`. It needs every test to pass, and at least 5 tests of yours on top of the checker's 3.

## Stuck?

- **Start the file with** `use grading::{fizzbuzz, is_leap_year, letter_grade};`.
- **`fizzbuzz` bug:** what does `fizzbuzz(15)` give? Which `if` does 15 hit first?
- **`is_leap_year` bug:** 1900 was *not* a leap year, but 2000 was.
- **`letter_grade` bug:** what does exactly 90 give?

## New words

- **Test**: code that runs other code and checks the result.
- **Assert**: state something that must be true, or the test fails.
- **Unit test / integration test**: testing a piece from inside, or the whole library from outside.
- **Regression**: an old bug coming back. Tests prevent them.
- **Edge case**: an input right at a boundary, like 0, empty, or exactly 90.
