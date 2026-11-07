# Lesson 11: Working with text

**What you'll learn**
- The difference between `String` and `&str`, once and for all
- Building and changing text
- Splitting text into words and taking pieces of it
- Useful text tools: uppercase, replacing, repeating, and reversing

## The idea: two kinds of text

You've seen both `String` and `&str`. Here's the difference in one picture:

- **`String`** is text you **own**. It lives on the heap, and it can grow and change, like a notebook that's yours.
- **`&str`** is a **borrowed view** of text, sometimes just part of it. It's like a page someone lets you read, or a highlighted sentence on that page.

Text in quotes, like `"hello"`, is a `&str`. It's baked into your program, so you can look but not change.

| You want to... | Use |
| --- | --- |
| Take text as an input to a function | `&str`, which accepts either kind |
| Give back new text from a function | `String` |
| Build up text bit by bit | `let mut text = String::new();` |

## Turning one into the other

```rust
let owned: String = "hello".to_string();   // &str → String (a copy)
let also: String = String::from("hello");  // same thing
let view: &str = &owned;                   // String → &str (a borrow, no copy)
```

## Building text

```rust
let mut text = String::new();   // empty
text.push_str("Hello");         // add text
text.push(' ');                 // add one character (single quotes)
text.push_str("there");
// text is now "Hello there"
```

`format!` is often easier: `format!("{greeting}, {name}!")`.

## Useful tools

| Code | Result |
| --- | --- |
| `"Hi".to_uppercase()` | `"HI"` |
| `"Hi".to_lowercase()` | `"hi"` |
| `"  hi  ".trim()` | `"hi"`, with spaces removed from the ends |
| `"a cat".replace("cat", "dog")` | `"a dog"` |
| `"ab".repeat(3)` | `"ababab"` |
| `"hello".len()` | `5` |
| `"hello".contains("ell")` | `true` |
| `"one two".split_whitespace()` | each word, one at a time |

## Taking a piece: slices

Square brackets with a range give you a **slice**, a `&str` view of part of the text:

```rust
let word = "Rustacean";
let first = &word[..1];    // "R"    : from the start, up to position 1
let rest  = &word[1..];    // "ustacean" : from position 1 to the end
let mid   = &word[0..4];   // "Rust"
```

**A warning about slices:** positions count **bytes**, not letters. Plain English letters are one byte each, so this works fine for them. Letters with accents, emoji, and many other alphabets use several bytes, and slicing through the middle of one crashes the program. When you need to handle any language safely, use `.chars()` instead, which goes letter by letter.

## Reversing text

`.chars().rev().collect()` reverses text. It splits text into letters, reverses their order, and collects them back into a `String`:

```rust
let backwards: String = "abc".chars().rev().collect();   // "cba"
```

You'll learn how `collect` works in the Iterators lesson. For now, use it as a recipe.

## Your challenge

Open `src/main.rs` and replace each `todo!()`:

1. **`initials`**: `"Ada Lovelace"` gives back `"AL"`. Take the first letter of each word, in capitals.
2. **`title_case`**: `"hello big world"` gives back `"Hello Big World"`, with one space between words.
3. **`censor`**: replace every appearance of a word with the same number of `*`s. `censor("darn it, darn", "darn")` gives back `"**** it, ****"`.
4. **`is_palindrome`**: `true` if text reads the same forwards and backwards, ignoring capitals and spaces. `"Never odd or even"` is one.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`.

## Stuck?

- **Initials:** start with `String::new()`. For each word in `split_whitespace()`, `push_str(&word[..1].to_uppercase())`.
- **Title case:** for each word, add a space first unless the text is still empty, then add `word[..1].to_uppercase()` and `&word[1..]`.
- **Palindrome:** build a lowercase copy without spaces, using a loop over `text.to_lowercase().chars()` that skips `' '`. Then compare it with its reverse.

## New words

- **Slice**: a borrowed view of part of something, like `&word[1..]`.
- **Byte**: the smallest unit of computer memory. One English letter is one byte.
- **Character**: one letter, digit, symbol, or emoji. Rust's type for it is `char`.
