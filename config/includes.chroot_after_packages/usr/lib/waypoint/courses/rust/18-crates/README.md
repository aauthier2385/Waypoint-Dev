# Lesson 18: Using other people's code: crates

**What you'll learn**
- What a crate is, and the two kinds: programs and libraries
- How `Cargo.toml` lists the code your project depends on
- Where crates come from: crates.io, folders, and Git
- What `Cargo.lock` really does
- How one project can have both `main.rs` and `lib.rs`

## The idea: don't rebuild the wheel

Most programs use code that other people wrote: for dates and times, for talking to websites, for random numbers. That shared code comes in packages, and in Rust a package of code is called a **crate**.

There are two kinds:

- A **binary crate** is a program you can run. It has `src/main.rs`.
- A **library crate** is code for *other code* to use. It has `src/lib.rs`, and you can't run it on its own.

Every project you've made so far was a binary crate. This lesson's folder contains both kinds.

## This lesson's layout

```
18-crates/
├── Cargo.toml          ← this project. It needs helpers, but doesn't say so yet.
├── src/
│   ├── lib.rs          ← this project's library part: where your code goes
│   └── main.rs         ← this project's program part: uses lib.rs
└── helpers/            ← a separate library crate with its own Cargo.toml
    ├── Cargo.toml
    └── src/lib.rs
```

`helpers` stands in for a crate someone else wrote. Open `helpers/src/lib.rs` and look at its two `pub` functions.

## Dependencies: the shopping list

To use another crate, list it under `[dependencies]` in your `Cargo.toml`. That's your project's shopping list, and Cargo fetches everything on it before building.

Crates can come from three places:

```toml
[dependencies]
rand = "0.8"                                            # from crates.io, the public crate library
helpers = { path = "helpers" }                          # from a folder on this computer
tools = { git = "https://github.com/someone/tools" }    # straight from a Git repository
```

**crates.io** is the main public collection, with well over 150,000 crates. You can browse it at crates.io. Instead of editing `Cargo.toml` by hand, `cargo add rand` adds a crate from it for you. Downloading from crates.io needs the internet, which is why this lesson uses a folder instead.

## Version numbers

Versions look like `1.4.2`: **major.minor.patch**.

- **Patch** (`1.4.2` → `1.4.3`): bug fixes only.
- **Minor** (`1.4` → `1.5`): new features, but old code still works.
- **Major** (`1.x` → `2.0`): breaking changes, so your code may need updating.

This system is called **semantic versioning**. Writing `rand = "0.8"` in `Cargo.toml` means "any version that's compatible with 0.8."

## `Cargo.lock`: the exact receipt

`Cargo.toml` says what's acceptable. `Cargo.lock` records the exact versions that were actually picked, for every crate, including the crates your crates depend on. That way, building next month, or on a friend's computer, uses the same code. Programs should keep `Cargo.lock` in Git. Run `cargo update` when you *want* newer versions.

## Using a crate in code

Once it's listed, a crate's name works like a module path:

```rust
let warm = helpers::celsius_to_fahrenheit(30.0);
```

Or bring things in with `use helpers::feels_like;`.

## main.rs and lib.rs together

When a project has both, they're two crates in one package. `main.rs` uses `lib.rs` by the **package name**, with `-` turned into `_`. This package is named `weather`:

```rust
// In src/main.rs
use weather::weather_report;
```

This is a common way to organize real projects: the logic lives in `lib.rs`, where it's easy to test, and `main.rs` is a thin layer that runs it. The guessing game in Lesson 08 kept its logic separate from `main` for the same reason.

## Your challenge

1. Run `cargo run`. Rust says it can't find `helpers`, because it isn't on the shopping list yet. Add it under `[dependencies]` in **this lesson's** `Cargo.toml`, the one at the top, not the one in `helpers/`.
2. In `src/lib.rs`, finish `weather_report`. Convert the Celsius temperature using `helpers`, and give back text like `"30°C is 86°F, which feels hot"`. Round Fahrenheit to a whole number with `{:.0}`.
3. Run `cargo run` again, and look at `Cargo.lock`. `helpers` is listed there now.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`.

## Stuck?

- **The dependency line** is `helpers = { path = "helpers" }`.
- **The report** is `format!("{celsius}°C is {fahrenheit:.0}°F, which feels {}", helpers::feels_like(fahrenheit))`, after working out `fahrenheit` on the line before.

## New words

- **Crate**: a package of Rust code, either a program (binary) or a library.
- **Dependency**: a crate your project needs.
- **crates.io**: the public collection of Rust crates.
- **Semantic versioning**: version numbers in the form major.minor.patch.
