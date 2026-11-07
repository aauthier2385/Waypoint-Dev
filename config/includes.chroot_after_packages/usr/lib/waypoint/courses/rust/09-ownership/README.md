# Lesson 09: Ownership

**What you'll learn**
- Where Rust keeps your data while a program runs
- The three rules of ownership, which make Rust different from every other popular language
- What a "move" is, and why Rust says a value was "moved"
- How to make a copy with `.clone()`

Welcome to the Intermediate level! This lesson is the most important one in the course. Take it slowly. Everything after it builds on these ideas.

## The idea: who's responsible for cleaning up?

Every program uses the computer's memory to hold its data. When the data isn't needed anymore, the memory must be given back, or the program slowly fills up memory until the computer struggles.

Most languages handle this in one of two ways:

- **The programmer cleans up by hand.** It's fast, but forgetting, or cleaning up twice, causes crashes and security holes. C and C++ work this way.
- **A "garbage collector" cleans up automatically.** It's safe, but it pauses your program now and then to look for trash. Python, Java, and JavaScript work this way.

Rust does something different: **every value has exactly one owner, and when the owner is done, Rust cleans up right then.** No hand cleanup, no pauses. The compiler checks the rules before your program runs, so mistakes are caught early.

## Two places data can live

A simple way to picture memory:

- **The stack** is like a notepad on your desk. It's quick, but it only holds small things whose size is known ahead of time, like a number or a `true`/`false`.
- **The heap** is like a storage room. It holds things that can grow, like text the user types. You keep a small note on your desk saying which shelf it's on.

A `String` is a note on the desk that points to a shelf in the storage room. That shelf is what ownership protects.

## The three rules

1. **Each value has one owner**, which is the variable holding it.
2. **There can only be one owner at a time.**
3. **When the owner goes away, the value is cleaned up.** A variable goes away at the `}` that ends the block it was made in.

## Moves: handing something over

```rust
let a = String::from("hello");
let b = a;              // the String MOVES from a to b
println!("{a}");        // error! a doesn't own anything anymore
```

When you write `let b = a;`, Rust doesn't copy the text. It hands the shelf over to `b`. If both `a` and `b` owned it, both would try to clean it up, which is exactly the double-cleanup bug Rust prevents. So after the move, `a` is off-limits.

**Passing a value into a function moves it too:**

```rust
fn make_greeting(name: String) -> String { ... }

let name = String::from("Sam");
let first = make_greeting(name);    // name moves into the function
let second = make_greeting(name);   // error: name was already moved
```

## Numbers are copied, not moved

Simple values that live on the stack, like `i32`, `u32`, `f64`, `bool`, and `char`, are just copied, because copying them is cheap:

```rust
let x = 5;
let y = x;          // x is copied
println!("{x}");    // fine
```

Types that copy this way are called **Copy** types.

## Making a real copy: `.clone()`

When you want two separate copies of a `String`, ask for one:

```rust
let a = String::from("hello");
let b = a.clone();      // a brand-new copy on a new shelf
println!("{a} {b}");    // both work
```

Cloning costs a little time and memory, so Rust makes you write it out. That way you always know when a copy happens.

## Giving ownership back

A function can take ownership and then hand it back by returning the value:

```rust
fn add_dot(mut text: String) -> String {
    text.push_str(".");
    text                     // ownership goes back to whoever called
}

let line = String::from("Done");
let line = add_dot(line);    // line moves in, and a changed line comes back
```

`push_str` adds text to the end of a `String`. The function needs `mut` on its parameter because it changes the text.

## Your challenge

Open `src/main.rs`.

1. **`greet_twice`** doesn't build. Run `cargo run` and read the `E0382` error, "use of moved value." It even shows where the move happened. Fix it by cloning `name` for the first call.
2. **`make_loud`**: take ownership of the text, add `"!!!"`, and give it back.
3. **`join_words`**: take two Strings and give back one String with both words and a space between them.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`.

## Stuck?

- **For `greet_twice`:** `make_greeting(name.clone())` on the first call.
- **For `make_loud`:** `format!("{text}!!!")` works. So does making the parameter `mut text: String`, calling `text.push_str("!!!");`, and returning `text`.
- **For `join_words`:** `format!("{first} {second}")`.

## New words

- **Owner**: the variable responsible for a value.
- **Move**: handing ownership to a new owner. The old name can't be used anymore.
- **Clone**: make a separate, full copy.
- **Drop**: what Rust calls cleaning up a value when its owner goes away.
- **Stack and heap**: two parts of memory: quick and fixed-size, or flexible and roomy.
