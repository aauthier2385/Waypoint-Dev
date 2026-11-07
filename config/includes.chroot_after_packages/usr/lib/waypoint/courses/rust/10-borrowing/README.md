# Lesson 10: Borrowing and references

**What you'll learn**
- How to let a function use a value without taking it
- The difference between `&` (look) and `&mut` (look and change)
- The one big borrowing rule, and why it prevents bugs
- What `*` means in front of a variable

## The idea: lending instead of giving

In the last lesson, passing a `String` to a function gave it away. That's awkward: often a function only needs to *look* at something, like counting its words, and you want it back afterward.

Rust lets you **lend** a value instead. Lending is called **borrowing**, and the loan itself is called a **reference**. It's like lending a friend a book: they can read it, but it's still yours, and they give it back when they're done.

## `&`: borrow to look

Put `&` in front of a value to lend it, and in front of a type to say a function borrows:

```rust
fn count_letters(text: &str) -> usize {
    text.len()
}

let word = String::from("hello");
let n = count_letters(&word);    // lend it
println!("{word} has {n} letters");   // still ours
```

A function that only borrows can't change what it borrowed. That's a promise the compiler enforces.

**`&str` is the best type for borrowed text.** It works whether the caller has a `String` or text in quotes, so `count_letters(&word)` and `count_letters("hi")` both work.

## `&mut`: borrow to change

If the function needs to change the original, lend it with `&mut`, a **mutable reference**:

```rust
fn add_dot(text: &mut String) {
    text.push_str(".");
}

let mut line = String::from("Done");
add_dot(&mut line);
println!("{line}");    // Done.
```

Three things have to line up: the variable is `mut`, the call says `&mut`, and the function's type says `&mut`. It's very clear about what can change, and where.

## The one big rule

At any moment, you can have **either**:

- **any number of `&` borrows** (many people reading), **or**
- **exactly one `&mut` borrow** (one person changing),

but never both at the same time.

Think of a whiteboard. Lots of people can read it at once. But if someone's erasing and rewriting, nobody should be reading halfway through, and two people shouldn't erase at once. This rule stops a whole family of bugs where data changes while something else is using it. In other languages those bugs show up as rare, hard-to-find crashes. In Rust, they're compile errors (`E0502` and `E0499`).

## Changing numbers through a reference: `*`

A reference *points to* a value. To change the value it points to, put `*` in front. That's called **dereferencing**:

```rust
fn add_one(number: &mut i32) {
    *number += 1;     // "change the thing number points to"
}
```

To change every number in a list, borrow each item mutably with `.iter_mut()`:

```rust
for number in numbers.iter_mut() {
    *number += 1;
}
```

`&mut [i32]` means "a borrowed list of `i32`s that I'm allowed to change." The square brackets mean a list of items.

## Your challenge

Open `src/main.rs`.

1. **`count_words`** takes ownership, but `main` and the checker lend it `&sentence`. Run `cargo run` to see the mismatched types error, then change the parameter so it borrows. `&str` is best.
2. **`add_exclamation`**: add `"!"` to the end of the borrowed text.
3. **`longest_word_length`**: give back the length of the longest word. Loop over `text.split_whitespace()`, which gives you each word, and keep the biggest `.len()`.
4. **`double_all`**: double every number in the list, changing the original.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`.

## Stuck?

- **For `count_words`:** `fn count_words(text: &str) -> usize`.
- **For `longest_word_length`:** start with `let mut longest = 0;`, then for each word, `if word.len() > longest { longest = word.len(); }`.
- **For `double_all`:** `for number in numbers.iter_mut() { *number *= 2; }`.

## New words

- **Borrow**: use a value without taking ownership.
- **Reference**: the borrowed thing itself, written `&value` or `&mut value`.
- **Mutable reference**: a borrow that's allowed to change the value.
- **Dereference**: reach through a reference to the value, with `*`.
