# Lesson 05: Functions

**What you'll learn**
- How to make your own functions
- How to give a function inputs and get an answer back
- How the checker's tests work

## The idea

A **function** is a named set of steps you can use over and over. You've been using one all along: `main`. Now you'll write your own.

Think of a function like a kitchen appliance. A blender takes ingredients in, does its job, and gives you a smoothie. You don't have to think about how it works each time. You just use it.

## The parts of a function

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

| Part | Meaning |
| --- | --- |
| `fn` | "Here comes a function." |
| `add` | The function's name. |
| `(a: i32, b: i32)` | The **inputs**, called *parameters*. Each one has a name and a type, so this takes two whole numbers called `a` and `b`. |
| `-> i32` | The type of answer it gives back, its **return type**. The arrow reads as "gives back." |
| `a + b` | The answer. **The last line, with no semicolon, is what the function gives back.** |

Using a function is called **calling** it:

```rust
let total = add(2, 3);   // total is now 5
```

### The semicolon rule

This is a small detail with a big effect:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b      // no semicolon: this is the answer
}
```

If you add a semicolon, as in `a + b;`, the line becomes a plain step and nothing is given back. Rust will complain with `mismatched types`, because you promised an `i32` and gave back nothing.

## Building text

To build text that includes values, use `format!`. It works like `println!`, but gives the text back instead of showing it:

```rust
let message = format!("You have {} points", 10);   // "You have 10 points"
```

Text you build this way has the type `String`. You'll learn how `String` differs from `&str` in the Intermediate level. For now, a function that gives back text uses `-> String`.

## How the checker works

Scroll to the bottom of `src/main.rs`. Below the line that says *"The checker"* are **tests**: small pieces of code that call your functions and confirm the answers are right.

```rust
assert_eq!(double(21), 42, "double(21) should be 42");
```

`assert_eq!` means "make sure these two things are equal." If they aren't, the test fails and shows both values. Real programmers write tests like this for their own code. You'll write your own later in the course.

## todo!()

Three functions contain `todo!()`, which means "not written yet." The program can still build, but if a function marked `todo!()` actually runs, the program stops with a "not yet implemented" message.

## Your challenge

Replace each `todo!()` with code that gives back the right answer:

1. `double`: give back the number times two. In Rust, `*` means multiply.
2. `greet`: give back `"Hello, "` plus the name plus `"!"`. Use `format!`.
3. `minutes_in`: give back the number of minutes in that many hours.

You can run the program with `cargo run` to see your answers. Before you finish, `cargo run` will stop at the first `todo!()` it reaches.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`. Each test is listed separately, so you can finish one function at a time and check as you go.

## Stuck?

- **Don't put a semicolon** after the answer line.
- **Multiplying:** `n * 2`. **Adding:** `a + b`. **Subtracting:** `a - b`. **Dividing:** `a / b`.
- **For `greet`:** `format!("Hello, {name}!")`.

## New words

- **Parameter**: an input a function takes.
- **Return value**: the answer a function gives back.
- **Call**: to use a function.
- **Test**: code that checks other code gives the right answers.
