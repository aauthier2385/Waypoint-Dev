# Lesson 04: Types of data

**What you'll learn**
- The main kinds of data: whole numbers, decimals, true/false, characters, and text
- How to tell Rust what type a variable is
- How to read a "mismatched types" error

## The idea

Every value in Rust has a **type**, meaning what kind of thing it is. A whole number, a decimal number, and a word are all stored differently inside the computer, so Rust needs to know which is which.

Think of types like containers in a kitchen: a carton holds eggs, a jug holds milk. You can't pour milk into an egg carton. Rust checks that every value goes into the right kind of container.

## The main types

| Type | What it holds | Examples |
| --- | --- | --- |
| `i32` | Whole numbers, positive or negative | `-5`, `0`, `42` |
| `u32` | Whole numbers that can't be negative | `0`, `12`, `1000` |
| `f64` | Numbers with a decimal point | `0.75`, `3.14`, `-2.5` |
| `bool` | Only `true` or `false` | `true` |
| `char` | One single character, in **single** quotes | `'a'`, `'7'`, `'★'` |
| `&str` | A piece of text, in **double** quotes | `"apple"` |

The letters and numbers mean something: `i` is for *integer* (a whole number), `u` is for *unsigned* (no minus sign), `f` is for *floating point* (a decimal), and `32` or `64` is how many bits of memory it uses. Bigger means it can hold larger or more precise numbers.

## Writing the type

You can tell Rust the type with a colon after the name:

```rust
let apples: u32 = 12;
```

Read it as: "Make a box called `apples` that holds a `u32`, and put 12 in it."

Usually you don't have to write the type, because Rust figures it out from the value. Writing it is useful when you want to be clear, or when Rust needs to be told.

## Converting between types

Rust won't mix types on its own. You can't multiply a `u32` by an `f64` directly, because it isn't sure what kind of answer you want. You convert with `as`:

```rust
let total = apples as f64 * price;   // treat apples as a decimal, then multiply
```

## Formatting numbers

`{price:.2}` means "show `price` with exactly 2 digits after the decimal point." So `9.0` shows as `9.00`, which looks right for money.

## Try it

```
cd ../04-types
cargo run
```

Rust will report several **mismatched types** errors (`E0308`). Each one says what type was **expected** and what it **found**. For example:

```
error[E0308]: mismatched types
 --> src/main.rs:6:22
  |
6 |     let price: u32 = 0.75;
  |                ---   ^^^^ expected `u32`, found floating-point number
  |                |
  |                expected due to this
```

In plain English: "You said `price` holds whole numbers (`u32`), but 0.75 has a decimal point."

## Your challenge

Fix the three wrong types so the program prints exactly:

```
12 apples at $0.75 each costs $9.00.
On sale: true
Rating: ★
```

1. `price` needs a type that can hold decimals.
2. `on_sale` is a `bool`, but `"yes"` is text. Use the right true/false value. Booleans don't use quotes.
3. `star` is a `char`, but it's written with double quotes, which is text. Characters use single quotes.

Fix one error at a time, running `cargo run` after each. Fixing one sometimes changes what the others say.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`.

## Stuck?

- **Decimal numbers** use `f64`.
- **Only two `bool` values exist**: `true` and `false`, always lowercase and with no quotes.
- **To type ★**, copy it from the error message or from this page. Any single character in single quotes works the same way.

## New words

- **Type**: what kind of value something is.
- **Integer**: a whole number.
- **Floating point**: a number with a decimal point.
- **Boolean**: a true-or-false value, named after the mathematician George Boole.
- **String**: text. In Rust you'll see it as `&str` and, later, as `String`.
