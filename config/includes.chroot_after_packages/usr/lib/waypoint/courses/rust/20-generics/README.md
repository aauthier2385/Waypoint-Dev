# Lesson 20: Generics: code for any type

**What you'll learn**
- How to write one function or struct that works with many types
- What `<T>` means, and why you've already seen it everywhere
- How to require abilities with **trait bounds**
- Building your own generic container

## The idea: a recipe with a blank

Imagine you wrote "find the biggest number" for whole numbers, then needed it for decimals, then for letters. You'd copy the same code three times, changing only the type. Three copies means three places for bugs.

**Generics** let you write it once, with a blank where the type goes:

```rust
fn largest<T>(items: &[T]) -> T { ... }
```

`<T>` means "this function works with some type. Call it `T` for now." Whoever uses the function fills in the blank. `largest(&[3, 7, 2])` uses `i32`, and `largest(&['a', 'z'])` uses `char`.

It's like a recipe that says "1 cup of fruit." The same steps work for apples or berries.

`T` is just a name. It's short for *type*, and it's tradition, but any name works.

## You've been using generics all along

- `Vec<i32>` and `Vec<String>` are one generic `Vec<T>` with different blanks filled in.
- `Option<T>` is "maybe a `T`."
- `Result<T, E>` has two blanks: the success type and the error type.
- `HashMap<K, V>` has a key type and a value type.

## Trait bounds: not just any type

Here's a catch. Inside `largest`, you'll compare with `>`. But can *every* type be compared? What would it mean for one `Dog` to be bigger than another? Rust won't let you use `>` on a plain `T`, and you'll see error `E0369` if you try.

The fix is to say which abilities `T` needs. That's called a **trait bound**:

```rust
fn largest<T: PartialOrd + Copy>(items: &[T]) -> T { ... }
```

Read it as: "`T` can be any type, **as long as** it can be compared (`PartialOrd`) and copied (`Copy`)." Numbers and `char`s have both, so they work. A type without them is turned away, with a clear error, before the program ever runs.

This is the trait certificate from the last lesson in action: generics say "any type," and bounds say "with these certificates."

## Generic structs

Structs can have blanks too:

```rust
struct Pair<T> {
    first: T,
    second: T,
}

let numbers = Pair { first: 3, second: 8 };          // Pair<i32>
let words = Pair { first: "hi", second: "there" };   // Pair<&str>
```

Methods go in `impl<T> Pair<T>`, which says "for every type T, here are Pair<T>'s methods." Methods that need abilities add bounds:

```rust
impl<T: PartialOrd + Copy> Pair<T> {
    fn larger(&self) -> T { ... }
}
```

## Is it slow?

No. When you build, Rust writes a separate copy of the code for each type you actually use. That's called **monomorphization**, which means "making one shape." Generic code runs exactly as fast as code written for one type.

## Your challenge

Open `src/main.rs` and replace each `todo!()`:

1. **`largest`**: give back the biggest item. First, add the trait bounds to the `fn` line. Try writing the body first if you want to see the error.
2. **`Pair::larger`**: give back whichever of `first` and `second` is bigger.
3. **`Stack<T>`**: a stack is a pile where you add to and take from the top, like a stack of plates. Using the `Vec` inside it, finish:
   - `push(item)`: put an item on top.
   - `pop()`: take the top item off, or `None` if the stack is empty.
   - `peek()`: look at the top item without removing it.
   - `len()` and `is_empty()`.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`. The checks use your code with numbers, letters, decimals, and text, all through the same functions.

## Stuck?

- **`largest`:** start with `let mut best = items[0];`, then compare each item. `Copy` is what lets you take `items[0]` out of the list.
- **`Stack`:** each method can be one line. `Vec` already has `push`, `pop`, `last`, `len`, and `is_empty`.

## New words

- **Generic**: written with a placeholder type, so it works with many types.
- **Type parameter**: the placeholder itself, like `T`.
- **Trait bound**: a requirement that a type has certain traits, like `T: PartialOrd`.
- **Stack**: a pile where the last thing added is the first thing removed.
