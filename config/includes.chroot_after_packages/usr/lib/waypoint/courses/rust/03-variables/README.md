# Lesson 03: Variables

**What you'll learn**
- How to store information with `let`
- How to show stored information in text
- Why Rust locks variables, and how to unlock them with `mut`
- How to read your first real error message

## The idea

A **variable** is a labeled box that holds a value. You give the box a name, put something in it, and use the name later instead of retyping the value.

```rust
let score = 0;
```

In plain English: "Make a box called `score` and put `0` in it." `let` is the word that makes a new variable.

## Showing variables in text

Put a variable's name inside curly braces in a `println!`, and Rust swaps in its value:

```rust
let name = "Sam";
println!("Hi, {name}!");   // prints: Hi, Sam!
```

## Locked boxes

Here's something that surprises people: **in Rust, variables can't change unless you say so.** Once you put something in the box, the lid is locked.

That's on purpose. In big programs, a value that changes when you don't expect it is one of the most common causes of bugs. Rust makes you say up front which values are allowed to change, so anyone reading the code knows.

To unlock the box, add `mut`, short for *mutable*, meaning "able to change":

```rust
let mut score = 0;   // this box can change
score = score + 10;  // now score is 10
```

`score = score + 10` means "take what's in `score`, add 10, and put the result back in `score`." The `=` here means *store*, not *equals* like in math.

## Try it

1. Move the terminal into this lesson's folder:

   ```
   cd ../03-variables
   ```

2. Run the program:

   ```
   cargo run
   ```

It won't run. Rust stops and shows an error. That's expected, so let's read it.

## Reading the error

You'll see something like this:

```
error[E0384]: cannot assign twice to immutable variable `score`
 --> src/main.rs:8:5
  |
3 |     let score = 0;
  |         ----- first assignment to `score`
...
8 |     score = score + 10;
  |     ^^^^^^^^^^^^^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
3 |     let mut score = 0;
  |         +++
```

Piece by piece:

- `error[E0384]` is the error's ID number. You can look up any error with `rustc --explain E0384`.
- `cannot assign twice to immutable variable` means "you tried to change a locked box." *Immutable* means "can't change."
- `src/main.rs:8:5` is the location: the file, then line 8, then column 5.
- The `^^^^` marks show the exact spot.
- **The `help:` section tells you how to fix it.** It shows `let mut score` with `+++` under the part to add.

Rust error messages often include the fix. Read them all the way to the end.

## Your challenge

1. Fix the error by adding `mut`, then run `cargo run` to see the first two lines.
2. Sam finds another coin worth **15** points. At the bottom of `main`, add code so the full output is:

```
Sam starts with 0 points.
Sam now has 10 points.
Sam finds another coin!
Sam now has 25 points.
```

Use `{name}` and `{score}` in your `println!` lines rather than typing "Sam" and "25." That way the program does the work.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`.

## Stuck?

- Add `mut` in only one place: where `score` is created with `let`.
- The new lines go above the final `}`.
- To add 15, copy the line `score = score + 10;` and change the number.

## New words

- **Variable**: a named box that holds a value.
- **Immutable**: can't be changed. Rust variables are immutable unless you add `mut`.
- **Mutable**: can be changed.
- **Assign**: put a value into a variable with `=`.
