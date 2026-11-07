# Lesson 06: Making decisions

**What you'll learn**
- How to make your program choose what to do with `if`, `else if`, and `else`
- How to compare values
- How to tell whether a number is even

## The idea

Programs often need to choose. If it's raining, take an umbrella. Otherwise, wear sunglasses. In Rust, you write that choice with `if`:

```rust
if raining {
    println!("Take an umbrella");
} else {
    println!("Wear sunglasses");
}
```

Rust checks the **condition** after `if`. If it's `true`, it runs the first block. If it's `false`, it runs the `else` block. Only one block ever runs.

## Comparing values

A condition is anything that's `true` or `false`. You usually make one by comparing:

| Symbol | Meaning | Example that's `true` |
| --- | --- | --- |
| `==` | is equal to | `5 == 5` |
| `!=` | is not equal to | `5 != 3` |
| `<` | is less than | `3 < 5` |
| `>` | is greater than | `5 > 3` |
| `<=` | is less than or equal to | `5 <= 5` |
| `>=` | is greater than or equal to | `6 >= 5` |

**`=` and `==` are different.** One `=` stores a value in a variable. Two `==` asks whether two values are equal.

You can combine conditions: `&&` means "and," `||` means "or," and `!` means "not."

```rust
if age >= 13 && age <= 19 {
    println!("Teenager");
}
```

## More than two choices

Use `else if` to check more conditions, in order. Rust stops at the first one that's true:

```rust
if score >= 90 {
    "A"
} else if score >= 80 {
    "B"
} else {
    "keep practicing"
}
```

Order matters. If you checked `>= 80` first, a score of 95 would get "B," because Rust would stop there.

## The example in this lesson

Open `src/main.rs`. The `sign_of` function is finished, so read it first:

```rust
fn sign_of(number: i32) -> String {
    if number > 0 {
        "positive".to_string()
    } else if number < 0 {
        "negative".to_string()
    } else {
        "zero".to_string()
    }
}
```

Two things to notice:

- **The whole `if` is the answer.** Each block ends with a value and no semicolon, so whichever block runs, its value is what the function gives back.
- **`.to_string()`** turns a piece of text in quotes into a `String`, which is the type this function promised to give back.

## The remainder trick

`%` gives the **remainder** after dividing. `7 % 2` is `1`, because 7 divided by 2 is 3 with 1 left over. `8 % 2` is `0`. So a number is even when `number % 2 == 0`.

## Your challenge

Replace each `todo!()`:

1. **`temperature_word`**: below 50 gives back `"cold"`, 50 to 79 gives back `"nice"`, and 80 or more gives back `"hot"`.
2. **`is_even`**: gives back `true` for even numbers and `false` for odd ones. A comparison is already `true` or `false`, so you can give back the comparison itself without an `if`.
3. **`ticket_price`**: under 5 is free (`0`), 5 to 12 is `8`, 13 to 64 is `12`, and 65 or older is `9`.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`. The tests check the edges, like exactly 50 and exactly 80, because that's where mistakes usually hide.

## Stuck?

- **Check one range at a time**, from smallest to largest.
- **"50 to 79"** means `degrees >= 50 && degrees <= 79`. If you've already handled everything below 50, then `degrees < 80` is enough.
- **For `is_even`**, the whole function can be one line: `number % 2 == 0`.

## New words

- **Condition**: a question whose answer is `true` or `false`.
- **Branch**: one of the paths an `if` can take.
- **Remainder**: what's left over after dividing, written as `%`.
