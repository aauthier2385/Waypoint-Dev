# Lesson 12: Structs: making your own types

**What you'll learn**
- How to group related values into one new type, called a **struct**
- How to give a struct its own functions, called **methods**
- The difference between `self`, `&self`, and `&mut self`
- How to print a whole struct for troubleshooting

## The idea: a form with labeled boxes

So far, every variable has held one thing: a number or some text. Real things have several parts. A rectangle has a width *and* a height. A bank account has an owner *and* a balance.

A **struct**, short for *structure*, groups related values under one name. Think of a paper form: it has labeled boxes, called **fields**, and each filled-in copy of the form is one value of that type.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}
```

That's the blank form. To fill one in:

```rust
let door = Rectangle { width: 3, height: 7 };
println!("{}", door.width);   // 3
```

Read a field with a dot: `door.width`.

## Methods: functions that belong to a struct

You add functions to a struct inside an `impl` block, short for *implementation*:

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

let door = Rectangle { width: 3, height: 7 };
println!("{}", door.area());   // 21
```

`self` means "the rectangle this method was called on." So inside `area`, `self.width` is `door`'s width.

## `self`, `&self`, and `&mut self`

The first parameter of a method says how it uses the struct. It's the borrowing rules from Lesson 10 again:

| First parameter | Meaning | Use it when |
| --- | --- | --- |
| `&self` | Borrow to look | Reading fields, like `area` |
| `&mut self` | Borrow to change | Changing fields, like `deposit` |
| `self` | Take ownership | Turning the struct into something else (rare) |

To call a `&mut self` method, the variable needs `mut`:

```rust
let mut account = BankAccount::new("Sam");
account.deposit(50);
```

## Functions without `self`: constructors

A function in `impl` with no `self` is called with `::` instead of a dot. The most common one builds a new value, and by tradition it's called `new`:

```rust
impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }     // short for width: width, height: height
    }
}

let window = Rectangle::new(4, 2);
```

You've used these already: `String::new()` and `String::from("hi")` are constructors.

Inside an `impl`, you can write `Self` with a capital S instead of the struct's name. It means "this type."

## Printing a whole struct

Put `#[derive(Debug)]` above a struct, and Rust writes the printing code for you. Then use `{:?}`:

```rust
#[derive(Debug)]
struct Rectangle { width: u32, height: u32 }

println!("{:?}", door);    // Rectangle { width: 3, height: 7 }
println!("{:#?}", door);   // same, spread over several lines
```

**Derive** means "Rust, please write this standard code for me." You'll meet more derivable abilities in the Traits lesson.

## Your challenge

Open `src/main.rs` and replace each `todo!()`.

**`Rectangle`:**
1. `square(size)`: build a rectangle with equal width and height.
2. `area()`: width times height.
3. `perimeter()`: the distance all the way around.
4. `can_hold(other)`: `true` if the other rectangle fits completely inside this one, meaning it's smaller in **both** width and height.

**`BankAccount`:**
5. `new(owner)`: a new account with that owner and a balance of 0. The owner is a `&str`, but the field is a `String`, so convert it.
6. `deposit(amount)`: add money.
7. `withdraw(amount)`: take money out and give back `true`. If there isn't enough, change nothing and give back `false`.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`.

## Stuck?

- **Perimeter** is `2 * (self.width + self.height)`.
- **`can_hold`:** `self.width > other.width && self.height > other.height`.
- **`new`:** `BankAccount { owner: owner.to_string(), balance: 0 }`.
- **`withdraw`:** check `if amount > self.balance` first.

## New words

- **Struct**: a type you create that groups named values.
- **Field**: one named value inside a struct.
- **Method**: a function that belongs to a type and is called with a dot.
- **Constructor**: a function that builds a new value, usually called `new`.
- **Derive**: ask Rust to write standard code for a type.
