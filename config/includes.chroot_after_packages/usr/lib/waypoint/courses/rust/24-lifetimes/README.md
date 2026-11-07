# Lesson 24: Lifetimes

**What you'll learn**
- Why borrowed values must never outlive what they borrow
- What a lifetime is, and what `'a` means
- When Rust works out lifetimes on its own, and when you have to write them
- Structs that hold references

Welcome to the Advanced level. These lessons cover the parts of Rust that experienced programmers reach for when building serious software. Each one builds on Ownership and Borrowing, so revisit Lessons 09 and 10 any time something feels slippery.

## The idea: a library card can't outlast the library

A reference is a loan. A loan only makes sense while the thing loaned still exists. If the library burns down, your library card points at nothing. In other languages, using a reference to something that's gone is called a **dangling pointer**. It's one of the most common causes of crashes and security holes in software.

Rust prevents it completely. Every reference has a **lifetime**: the stretch of code where it's valid. The compiler checks that no reference is used after the value it points to has been dropped.

```rust
let card;
{
    let library = String::from("Main Street Library");
    card = &library;
}                        // library is dropped here
println!("{card}");      // error: library doesn't live long enough
```

## Usually, Rust figures it out

Most of the time you never write lifetimes, because Rust follows a few simple rules. For example, if a function borrows one thing and gives back a reference, the answer must come from that one thing:

```rust
fn first_word(text: &str) -> &str {     // no lifetimes needed
    ...
}
```

Working out lifetimes automatically like this is called **lifetime elision**. *Elide* means "leave out."

## When you have to say it

Now look at a function with **two** borrowed inputs:

```rust
fn longest(a: &str, b: &str) -> &str {
    if a.len() >= b.len() { a } else { b }
}
```

Rust stops with error `E0106`, "missing lifetime specifier." It can't tell whether the answer borrows from `a` or `b`, and that matters: whoever calls `longest` needs to know how long the answer stays valid.

You tell it with a **lifetime parameter**:

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
```

Read `'a` as a label. It's pronounced "lifetime a," and the apostrophe marks it as a lifetime. This says: "`a` and `b` both live for at least some stretch called `'a`, and the answer is valid for that same stretch." In practice, the answer is valid as long as **both** inputs are still around.

**Lifetimes don't change how long anything lives.** They describe relationships, so the compiler can check your code. It's like a label on a borrowed tool that says "must be returned by Friday."

## Structs that borrow

A struct can hold a reference, but it must say how long that reference lasts:

```rust
struct Highlight<'a> {
    text: &'a str,
}
```

This says: "A `Highlight` can't outlive the text it points to." Its methods go in `impl<'a> Highlight<'a>`, the same pattern as generics in Lesson 20. Lifetimes *are* a kind of generic parameter.

## `'static`: lives forever

Text in quotes is built into your program, so it's valid for the entire run. Its type is `&'static str`. You saw it in Lesson 18's `feels_like`.

## Your challenge

Run `cargo run` and read the errors first.

1. **`longest`**: add the lifetime parameter so it builds.
2. **`first_word`**: give back the text up to the first space, or all of it if there's no space. You won't need to write any lifetimes, because elision handles it.
3. **`Highlight`**: add a lifetime to the struct and to its `impl`.
4. **`Highlight::first_sentence`**: give back the text before the first `.`, or all of it if there's no period.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`.

## Stuck?

- **`first_word`:** `match text.find(' ') { Some(position) => &text[..position], None => text }`. `.find` gives back the position of the first match as an `Option`.
- **`Highlight::new`:** `fn new(text: &'a str) -> Highlight<'a>`.
- **`first_sentence`:** the same pattern as `first_word`, looking for `'.'`.

## New words

- **Lifetime**: the stretch of code where a reference is valid.
- **Dangling reference**: a reference to something that's already gone. Rust doesn't allow them.
- **Lifetime parameter**: a label like `'a` that connects the lifetimes of references.
- **Elision**: Rust working out lifetimes so you don't have to write them.
