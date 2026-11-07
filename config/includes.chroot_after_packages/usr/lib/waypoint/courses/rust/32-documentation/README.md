# Lesson 32: Documentation and publishing

**What you'll learn**
- Writing documentation that lives right next to your code
- Code examples in documentation that are tested automatically
- Building a documentation website with one command
- Getting a crate ready to share on crates.io

## The idea: instructions that can't go out of date

Code that nobody understands is code nobody can use, including you in six months. Rust has documentation built into the language, and one feature that's rare elsewhere: **the examples in your documentation are run as tests.** If you change your code and an example stops working, `cargo test` tells you. Your instructions can't quietly go out of date.

## Three kinds of comments

```rust
//! Describes the whole crate or module. Goes at the very top of a file.

/// Describes the item right below it.
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 { ... }

// A normal comment, for people reading the code. Not part of the documentation.
```

Documentation comments are written in **Markdown**, the same format as these lessons, so you can use `**bold**`, lists, and code.

## Writing good documentation

A good doc comment has:

1. **A one-line summary.** What does it do?
2. **More detail**, if it's needed: what the inputs mean, and what happens in odd cases.
3. **An example** showing how to use it.

````rust
/// Converts a temperature from Celsius to Fahrenheit.
///
/// # Examples
///
/// ```
/// use thermo::celsius_to_fahrenheit;
///
/// assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
/// ```
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
````

The example between the triple backticks is a **doc test**. When you run `cargo test`, Rust pulls each one out and runs it as its own small program. That's why the example starts with `use thermo::...`: it runs from the outside, like someone using your crate.

Common sections are `# Examples`, `# Panics` (when the function can crash), and `# Errors` (what its `Err` values mean).

## See it as a website

```
cargo doc --open
```

This builds a documentation website for your crate and opens it in your browser. It's the same format used for every crate on **docs.rs**, the site that hosts documentation for every crate on crates.io. You've seen the standard library's version if you've ever looked up a Rust function.

## Getting ready to publish

**crates.io** is where Rust programmers share crates. Before publishing, fill in your `Cargo.toml`:

```toml
[package]
name = "thermo"               # must be unique on crates.io
version = "0.1.0"
edition = "2021"
description = "Converts between Celsius and Fahrenheit"
license = "MIT OR Apache-2.0"
repository = "https://github.com/you/thermo"
readme = "README.md"
```

- **`description`** and **`license`** are required. The license tells people what they're allowed to do with your code. `MIT OR Apache-2.0` is what most Rust crates use.
- **`version`** follows the major.minor.patch rules from Lesson 18. Raise the right number whenever you publish changes.

Then:

```
cargo publish --dry-run     # checks everything, without uploading
cargo login                 # once: paste a token from your crates.io account
cargo publish               # uploads it for everyone
```

**Publishing is permanent.** You can't delete a published version. You can only **yank** it, which stops new projects from using it. Always check `--dry-run` first, and never publish anything containing passwords or private information.

For this lesson, you won't publish anything. Just get the crate ready.

## Your challenge

Open `src/lib.rs`.

1. **At the top**, replace the placeholder comment with a `//!` comment describing the crate.
2. **Above each of the three functions**, write a `///` comment with a summary and an `# Examples` section. Include a code example that uses the function and checks its answer.
3. **Run `cargo test`.** You'll see a "Doc-tests thermo" section listing your examples.
4. **Run `cargo doc --open`** to see your documentation as a website.
5. **Extra credit:** add `description` and `license` to `Cargo.toml`, then try `cargo publish --dry-run`. It needs the internet to check crates.io, and it packages your crate without uploading anything. If someone already owns the name `thermo`, it will say so. That's the kind of thing a dry run is for.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`. It needs at least 3 working doc tests, one per function, on top of the checker's 3 tests.

## Stuck?

- **Copy the example above** and change it for each function.
- **For `is_freezing`,** use `assert!(is_freezing(-5.0));` and `assert!(!is_freezing(20.0));`.
- **If a doc test fails**, the error names the function and line, just like a normal test.

## New words

- **Doc comment**: a `///` or `//!` comment that becomes documentation.
- **Doc test**: a code example in documentation that runs as a test.
- **docs.rs**: the website that hosts documentation for every published crate.
- **Publish / yank**: share a crate version on crates.io, and stop new projects from using a version.
