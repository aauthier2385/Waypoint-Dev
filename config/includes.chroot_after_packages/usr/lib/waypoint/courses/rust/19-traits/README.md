# Lesson 19: Traits: shared abilities

**What you'll learn**
- How to describe an ability that many types can share, called a **trait**
- How to give a type that ability with `impl Trait for Type`
- Default methods that types get for free
- Writing functions that accept any type with a certain ability
- How `{}` printing works for your own types

## The idea: a skill certificate

Different things can share an ability. A dog, a car, and a song can all be *described*. A square and a circle can both have an *area*. A **trait** names an ability and lists what a type must be able to do to have it.

Think of a certificate, like a lifeguard certificate. It says: "Anyone with this can swim 500 meters and do rescues." Many different people can earn it. Once someone has it, you know what they can do, without knowing anything else about them.

```rust
trait Describe {
    fn describe(&self) -> String;     // the requirement: no body, just the promise
}
```

## Earning the certificate: `impl Trait for Type`

```rust
struct Dog { name: String }

impl Describe for Dog {
    fn describe(&self) -> String {
        format!("a dog named {}", self.name)
    }
}
```

Now `Dog` has the `Describe` ability, so `rex.describe()` works. If you leave out a required method, Rust stops with error `E0046`.

## Default methods: free extras

A trait can also include finished methods. Every type with the trait gets them automatically:

```rust
trait Describe {
    fn describe(&self) -> String;

    fn shout(&self) -> String {               // has a body, so it's a default
        self.describe().to_uppercase()
    }
}
```

`Dog` only wrote `describe`, but `rex.shout()` works too. A type can replace a default with its own version if it wants.

## Functions that accept anything with an ability

```rust
fn introduce(thing: &impl Describe) -> String {
    format!("Meet {}", thing.describe())
}
```

`&impl Describe` means "borrow anything that has the Describe ability." You can pass a `Dog`, a `Car`, or any type you add later. This is one of Rust's most powerful ideas: write code once, and it works with types that don't even exist yet.

## Mixing different types in one list: `dyn`

A `Vec` normally holds one type. To hold "anything with the Area ability," use `&dyn Area`:

```rust
let shapes: Vec<&dyn Area> = vec![&square, &circle];
```

`dyn` stands for *dynamic*. Rust works out which `area` to call while the program runs. You'll use it in `total_area`.

## Traits you've already met

Many of Rust's built-in features are traits:

| Trait | Ability | You've seen it as |
| --- | --- | --- |
| `Debug` | Print for programmers with `{:?}` | `#[derive(Debug)]` |
| `PartialEq` | Compare with `==` | `#[derive(PartialEq)]` |
| `Clone` | Make a copy with `.clone()` | `String` has it |
| `Display` | Print for people with `{}` | Numbers and text have it |

`derive` writes the `impl` for you. `Display` can't be derived, because only you know how your type should look to people. So you write it:

```rust
use std::fmt;

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}°C", self.degrees)
    }
}
```

`write!` works like `format!`, but writes into `f`, which is wherever the text is going. After this, `println!("{}", temp)` and `temp.to_string()` both work.

## Your challenge

Open `src/main.rs`. `Dog` is finished as an example.

1. **`Car`**: give it the `Describe` ability. A 2020 Ford describes itself as `"a 2020 Ford"`.
2. **`introduce`**: give back `"Meet "` followed by the thing's description.
3. **`Temperature`**: give it `Display` so it prints like `21.5°C`.
4. **`Square` and `Circle`**: give both the `Area` ability. A square's area is side × side, and a circle's is π × radius × radius.
5. **`total_area`**: add up the area of every shape in the list.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`.

## Stuck?

- **Copy the shape of `impl Describe for Dog`** and change it for `Car`.
- **`shout` already works for `Car`** once `describe` is written. That's the default method doing its job.
- **`total_area`:** loop over `shapes` and add up `shape.area()`.

## New words

- **Trait**: a named ability that types can have.
- **Implement**: give a type a trait by writing `impl Trait for Type`.
- **Default method**: a trait method with a body that types get automatically.
- **Trait object (`dyn`)**: a value whose exact type is known only while the program runs.
