# Lesson 28: Macros: code that writes code

**What you'll learn**
- What a macro is, and how it differs from a function
- Writing your own with `macro_rules!`
- Matching different inputs with different rules
- Repetition: macros that take any number of inputs

## The idea: a stamp, not a machine

You've used macros since Lesson 01: `println!`, `format!`, `vec!`, `assert_eq!`, `todo!`. The `!` marks a macro.

A function is like a machine: values go in, it runs, and a value comes out. A macro is like a rubber stamp: **before your program is compiled, Rust replaces the macro with the code it stands for.** `vec![1, 2, 3]` is stamped out into code that makes a `Vec` and pushes three items.

That lets macros do things functions can't:

- **Take any number of inputs.** `println!` takes one, two, or ten values. A function always takes a fixed number.
- **Take different kinds of inputs**, like code, types, or names, and not just values.
- **Write repetitive code for you**, so you write the pattern once.

## Your first macro

```rust
macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

square!(4)          // becomes 4 * 4
square!(2 + 3)      // becomes (2 + 3) * (2 + 3), which is 25
```

- `macro_rules! square` starts a macro called `square`.
- `($x:expr)` is the **pattern**: what the macro accepts. `$x` is a name for the input, and `expr` says it's an **expression**, meaning any piece of code that produces a value.
- `=> { $x * $x }` is the **replacement**: the code to stamp out.

Notice `square!(2 + 3)` gives 25, not `2 + 3 * 2 + 3 = 11`. Rust treats each `expr` as one complete unit, as if it had parentheses around it. Macros in some other languages, like C, get this wrong.

## Several rules

A macro can have several rules, separated by `;`. Rust uses the first one that matches:

```rust
macro_rules! hello {
    () => {
        String::from("Hello!")
    };
    ($name:expr) => {
        format!("Hello, {}!", $name)
    };
}

hello!()          // "Hello!"
hello!("Ana")     // "Hello, Ana!"
```

## Repetition

`$( ... ),*` means "this part, repeated any number of times, separated by commas." Use the same `$( ... )*` in the replacement to repeat the code once for each:

```rust
macro_rules! shout_all {
    ($($word:expr),*) => {
        vec![$($word.to_uppercase()),*]
    };
}

shout_all!("hi", "there")    // vec!["HI", "THERE"]
```

Use `*` for "zero or more" and `+` for "one or more." Adding `$(,)?` at the end of a pattern allows an optional trailing comma.

## Macros calling themselves

A macro can use itself on the rest of its input, like the recursive functions in Lesson 25:

```rust
macro_rules! add_all {
    ($x:expr) => { $x };                                   // one number: that's the answer
    ($x:expr, $($rest:expr),+) => { $x + add_all!($($rest),+) };   // first + the sum of the rest
}

add_all!(1, 2, 3)    // 1 + (2 + 3)
```

## When to use macros

Reach for a function first. Functions are easier to read, test, and get good error messages from. Use a macro when a function genuinely can't do the job: any number of inputs, or writing out repetitive code. Rust also has **procedural macros**, like `#[derive(Debug)]`, which are full Rust programs that write code. They're beyond this course, but they're how crates like serde work.

To see what a macro stamps out, install `cargo-expand` with `cargo install cargo-expand`, then run `cargo expand`.

## Your challenge

Open `src/main.rs`. `square!` is finished as an example. Write three more macros above `main`:

1. **`greet!`**: `greet!()` gives `"Hello, stranger!"`, and `greet!("Sam")` gives `"Hello, Sam!"`.
2. **`string_vec!`**: `string_vec!["a", "b"]` gives a `Vec<String>`. Allow any number of items, including zero, and an optional trailing comma.
3. **`maximum!`**: `maximum!(3, 9, 4)` gives `9`. Take one or more items.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`. Until you've written all three, Rust reports "cannot find macro," because the checker uses them.

## Stuck?

- **`string_vec!`:** `($($item:expr),* $(,)?) => { vec![$($item.to_string()),*] }`.
- **`maximum!`:** for the second rule, work it out in a block: `{ let first = $x; let rest = maximum!($($rest),+); if first > rest { first } else { rest } }`.
- **Macros must come before** the code that uses them in the file.

## New words

- **Macro**: code that writes code, expanded before compiling.
- **Expression**: any piece of code that produces a value.
- **Pattern / replacement**: what a macro rule matches, and what it turns into.
- **Expand**: replace a macro with the code it stands for.
