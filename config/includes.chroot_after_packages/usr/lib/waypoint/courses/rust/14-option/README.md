# Lesson 14: Option: when something might be missing

**What you'll learn**
- How Rust handles "there might be nothing here" safely
- `Some` and `None`, the two sides of `Option`
- Getting the value out with `match`, `if let`, and `unwrap_or`
- Why Rust doesn't have "null"

## The idea: a box that might be empty

Lots of questions might not have an answer. What's the first even number in this list? There might not be one. What's Ana's phone number? Maybe we don't have it.

Many languages answer "nothing" with a special value called **null**. The trouble is that any value *might* secretly be null, and if your code forgets to check, the program crashes. Its inventor, Tony Hoare, called it his "billion-dollar mistake."

Rust has no null. Instead, when something might be missing, its type says so, using `Option`:

```rust
enum Option<T> {
    Some(T),   // there's a value, and here it is
    None,      // there's nothing
}
```

It's just an enum, like the ones in the last lesson. `T` stands for "any type," so `Option<i32>` is "maybe a number" and `Option<String>` is "maybe some text." You'll learn how `<T>` works in the Generics lesson.

Think of a gift box: it either has something inside (`Some`) or it's empty (`None`). **You have to open it before you can use what's inside**, and Rust won't let you skip that step.

## Making an Option

```rust
fn first_letter(word: &str) -> Option<char> {
    if word.is_empty() {
        None
    } else {
        Some(word.chars().next().unwrap())
    }
}
```

`Some` and `None` are built in, so you don't need `Option::` in front.

## Opening the box

**With `match`**, handling both cases:

```rust
match first_letter(word) {
    Some(letter) => println!("It starts with {letter}"),
    None => println!("The word is empty"),
}
```

**With `if let`**, when you only care about `Some`:

```rust
if let Some(letter) = first_letter(word) {
    println!("It starts with {letter}");
}
```

**With `unwrap_or`**, using a backup value if it's empty:

```rust
let letter = first_letter(word).unwrap_or('?');
```

**With `.unwrap()`**, which takes the value out and *crashes* if there isn't one. Only use it when you're certain there's a value, or in quick experiments. Using `match`, `if let`, or `unwrap_or` instead is how Rust programs avoid crashing.

## Handy Option tools

| Code | What it does |
| --- | --- |
| `maybe.is_some()` / `maybe.is_none()` | Ask whether there's a value |
| `maybe.unwrap_or(0)` | The value, or 0 if there isn't one |
| `maybe.map(\|n\| n * 2)` | Change the value if there is one, and keep `None` as `None` |

`|n| n * 2` is a small unnamed function called a **closure**. You'll learn closures properly in Lesson 21.

## Your challenge

Open `src/main.rs` and replace each `todo!()`:

1. **`find_first_even`**: the first even number in the list, or `None` if there isn't one.
2. **`safe_divide`**: `a / b`, or `None` if `b` is 0, because dividing by zero has no answer.
3. **`describe_phone`**: use `phone_for`, which is finished. Give back `"Sam: 555-0101"` if there's a number, or `"No number for Ana"` if there isn't.
4. **`double_or_zero`**: twice the number if there is one, or 0 if it's `None`.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`.

## Stuck?

- **`find_first_even`:** loop over `numbers`, and as soon as you find one, `return Some(*number);`. After the loop, give back `None`. The `*` is there because looping over a borrowed list gives you `&i32`s.
- **`double_or_zero`:** `match maybe { Some(n) => n * 2, None => 0 }`.

## New words

- **Option**: Rust's type for "maybe a value, maybe nothing."
- **Some / None**: the two variants of `Option`.
- **Null**: the "nothing" value in other languages that Rust doesn't have.
- **Unwrap**: take the value out of an `Option`, crashing if there isn't one.
