# Lesson 25: Smart pointers: Box, Rc, and RefCell

**What you'll learn**
- What a pointer is, and what makes one "smart"
- `Box`: putting something on the heap, and building types that contain themselves
- `Rc`: letting several owners share one value
- `RefCell`: changing something that's shared
- Recursion: a function that calls itself

## The idea: a pointer is an address

A **pointer** is a value that holds the *address* of something else in memory, like a slip of paper with "Shelf 42" written on it. References (`&`) are pointers that only borrow.

**Smart pointers** are pointers that also *own* what they point to and handle extra jobs automatically. You've already used two: `String` and `Vec` both point to data on the heap and clean it up when they're dropped. This lesson covers three more.

| Smart pointer | What it gives you | Everyday comparison |
| --- | --- | --- |
| `Box<T>` | One owner, data on the heap | A storage unit with one key |
| `Rc<T>` | Several owners share one value | A shared family photo album |
| `RefCell<T>` | Change data even when it's shared | A sign-out sheet: one person changes it at a time |

## `Box`: a type that contains itself

Picture a train: each car is a number plus the rest of the train, and the last car is the end.

```rust
enum List {
    Item(i32, List),    // a number, then the rest of the list
    End,
}
```

Rust rejects this with error `E0072`, "recursive type has infinite size." To know how much space a `List` takes, Rust needs the size of `List`, which includes a `List`, which includes a `List`... forever.

The fix is to store the rest of the list somewhere else and keep just its address:

```rust
enum List {
    Item(i32, Box<List>),
    End,
}

let list = List::Item(1, Box::new(List::Item(2, Box::new(List::End))));
```

A `Box` is always the size of one address, so Rust knows exactly how big a `List` is. `Box::new(value)` moves the value onto the heap.

## Recursion: a function that calls itself

To add up a `List`, notice that the sum is "this number plus the sum of the rest":

```rust
fn sum(list: &List) -> i32 {
    match list {
        List::Item(value, rest) => value + sum(rest),
        List::End => 0,
    }
}
```

`sum` calls itself on a smaller piece each time, until it reaches `End`. A function calling itself is called **recursion**. It works naturally for any shape that contains smaller copies of itself, like lists, folder trees, and family trees.

## `Rc`: shared ownership

Ownership says each value has one owner. But sometimes several parts of a program genuinely share something, like three people on a team who all need the same project plan.

`Rc` stands for **reference counted**. It keeps a count of owners and cleans up when the last one is dropped:

```rust
use std::rc::Rc;

let plan = Rc::new(String::from("Launch in June"));
let copy = Rc::clone(&plan);       // a second owner. The text is NOT copied.
println!("{}", Rc::strong_count(&plan));   // 2
```

`Rc::clone` only adds one to the count, so it's fast. Everything shared through an `Rc` is read-only.

## `RefCell`: changing shared data

`Rc` alone can't change its value, because two owners changing it at once would break the borrowing rules. `RefCell` moves those rules from **compile time** to **run time**: it checks them while the program runs.

```rust
use std::cell::RefCell;

let score = RefCell::new(0);
*score.borrow_mut() += 10;          // borrow it mutably, and change it
println!("{}", score.borrow());     // borrow it to read: 10
```

If your code breaks the rules, like holding a `borrow()` and asking for `borrow_mut()` at the same time, the program panics instead of failing to build. That's why you only reach for `RefCell` when you really need it.

Put them together, as `Rc<RefCell<T>>`, and you get a value that several owners share **and** can change:

```rust
let counter = Rc::new(RefCell::new(0));
let other = Rc::clone(&counter);
*other.borrow_mut() += 1;
println!("{}", counter.borrow());   // 1. Both names see the change.
```

Changing something through a shared, read-only-looking handle is called **interior mutability**.

## Your challenge

Open `src/main.rs`.

1. **`List`** doesn't build. Fix it with `Box`.
2. **`sum`** and **`length`**: add up the numbers, and count the items. Use recursion.
3. **`share`**: give back a `Vec` of `times` new owners of the same value, using `Rc::clone`.
4. **`new_counter`** and **`bump`**: make a shared counter that starts at 0, and add 1 to one.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`.

## Stuck?

- **`length`** is like `sum`, but each item counts 1: `1 + length(rest)`.
- **`share`:** `(0..times).map(|_| Rc::clone(value)).collect()`.
- **`new_counter`:** `Rc::new(RefCell::new(0))`. **`bump`:** `*counter.borrow_mut() += 1;`.

## New words

- **Pointer**: a value holding the address of something in memory.
- **Smart pointer**: a pointer that owns its data and manages it automatically.
- **Recursion**: a function that calls itself on a smaller piece of the problem.
- **Reference counting**: keeping track of how many owners a value has.
- **Interior mutability**: changing data through a handle that looks read-only.
