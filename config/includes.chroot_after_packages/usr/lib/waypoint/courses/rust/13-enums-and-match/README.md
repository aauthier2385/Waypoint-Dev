# Lesson 13: Enums and match

**What you'll learn**
- How to make a type that can be one of several choices, called an **enum**
- How choices can carry their own data
- How `match` handles every choice, and why Rust won't let you forget one
- Matching numbers and ranges

## The idea: a list of possibilities

Some things can only be one of a few options. A traffic light is red, yellow, or green. A direction is north, east, south, or west. An **enum**, short for *enumeration*, which means listing things out, is a type for exactly that:

```rust
enum Direction {
    North,
    East,
    South,
    West,
}

let heading = Direction::North;
```

Each option is called a **variant**. You name one with the enum's name, `::`, and the variant, as in `Direction::North`.

A struct says "this AND this AND this." An enum says "this OR this OR this."

## `match`: one branch per possibility

`match` looks at a value and runs the branch that fits it:

```rust
fn turn_left(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::West,
        Direction::West => Direction::South,
        Direction::South => Direction::East,
        Direction::East => Direction::North,
    }
}
```

Read `=>` as "then." Each line is called an **arm**.

**Rust makes sure you cover every possibility.** If you leave out `Direction::East`, the program won't build. The error says "non-exhaustive patterns," meaning not every case is handled. It's like a checklist the compiler won't let you skip, and it means adding a new variant later shows you every place that needs updating.

## Variants that carry data

Each variant can hold its own values, and different variants can hold different things:

```rust
enum Shape {
    Circle(f64),                        // a radius
    Rectangle(f64, f64),                // width and height
    Triangle { base: f64, height: f64 },  // named, like a struct
}
```

`match` can pull those values out, giving each one a name you choose:

```rust
match shape {
    Shape::Circle(radius) => println!("radius {radius}"),
    Shape::Rectangle(width, height) => println!("{width} by {height}"),
    Shape::Triangle { base, height } => println!("base {base}"),
}
```

## Matching numbers

`match` works on numbers too, including ranges. `_` means "anything else":

```rust
match age {
    0 => "newborn",
    1..=12 => "child",
    13..=19 => "teen",
    _ => "adult",
}
```

Rust checks arms from top to bottom and uses the first one that fits. `_` belongs last, because it fits everything.

## `#[derive(Debug, PartialEq)]`

You'll see this above the enums in this lesson. `Debug` lets you print them with `{:?}`, and `PartialEq` lets you compare them with `==`. The checker needs both.

## Your challenge

Open `src/main.rs` and replace each `todo!()`:

1. **`turn_right`**: North becomes East, East becomes South, and so on around the compass.
2. **`opposite`**: North and South swap, and so do East and West.
3. **`value_in_cents`**: a Penny is 1, a Nickel 5, a Dime 10, and a Quarter 25.
4. **`area`**: a circle is π × radius × radius (use `std::f64::consts::PI`), a rectangle is width × height, and a triangle is base × height ÷ 2.
5. **`size_word`**: negative numbers give `"negative"`, 0 gives `"zero"`, 1–9 gives `"small"`, 10–99 gives `"medium"`, and anything bigger gives `"large"`.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`.

## Stuck?

- **Inside a method, match on `self`:** `match self { Direction::North => Direction::East, ... }`.
- **Negative numbers in a range:** `i32::MIN..=-1` means "the smallest number up to -1." You can also use a **match guard**: `n if n < 0 => "negative".to_string()`.
- **Remember `.to_string()`**, because `size_word` gives back a `String`.

## New words

- **Enum**: a type that's exactly one of a list of choices.
- **Variant**: one choice in an enum.
- **Match arm**: one `pattern => result` line in a `match`.
- **Exhaustive**: covers every possibility. Rust requires `match` to be exhaustive.
- **Pattern**: the left side of a match arm, describing what it fits.
