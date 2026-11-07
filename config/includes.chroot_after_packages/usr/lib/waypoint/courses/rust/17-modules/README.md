# Lesson 17: Splitting code across files

**What you'll learn**
- How to split a program into several files, called **modules**
- How files connect with `mod`, `use`, and `pub`
- How folders of modules work
- What "private" means, and why it's the default

This is the lesson on how files in a Rust project talk to each other. Keep **FILES.md** open alongside it.

## The idea: chapters in a book

A real program can have tens of thousands of lines. One giant file would be impossible to find your way around. So programs are split into **modules**, each one about a single topic, like chapters in a book or rooms in a house.

In Rust, each `.rs` file in `src` is usually one module. This lesson's project has these files:

```
src/
├── main.rs          ← starts the program, and connects the others
├── shapes.rs        ← the "shapes" module
├── greetings.rs     ← you'll create this one
└── math/            ← a folder module, called "math"
    ├── mod.rs       ← the "math" module's own file
    └── stats.rs     ← the "math::stats" module, inside math
```

## Files don't see each other on their own

Just putting a file in `src` does nothing. Rust only knows about a module when its **parent** declares it with `mod`. That's like a table of contents: if a chapter isn't listed, it isn't in the book.

```rust
// In src/main.rs
mod shapes;      // "There's a module called shapes. Its code is in src/shapes.rs."
```

`main.rs` is the root, the top of the tree, so it's where top-level modules get declared.

## Folders of modules

For a folder like `math/`, Rust looks for `src/math/mod.rs`. That file is the `math` module itself, and it declares the modules inside the folder:

```rust
// In src/math/mod.rs
pub mod stats;   // "math contains stats, in src/math/stats.rs, and others may use it."
```

The tree of modules matches the tree of folders:

```
crate            (main.rs)
├── shapes       (shapes.rs)
├── greetings    (greetings.rs)
└── math         (math/mod.rs)
    └── stats    (math/stats.rs)
```

`crate` means the top, which is your whole project.

## Private by default: `pub`

Everything in a module is **private** unless you mark it `pub`, for *public*. Private items can only be used inside their own module.

```rust
// In src/shapes.rs
pub fn describe_square(size: u32) -> String { ... }   // others can use this
fn helper() { ... }                                    // only shapes.rs can use this
```

Why private by default? It lets each module keep its inner workings to itself. You can change or rename private things freely, knowing nothing else in the program depends on them. Only the `pub` parts are a promise to the rest of your code.

## Reaching into modules: paths and `use`

To use something from a module, write its **path**, with `::` between each step:

```rust
let text = shapes::describe_square(3);
let big = math::stats::biggest(&[4, 9, 2]);
```

If you use something often, `use` creates a shortcut:

```rust
use math::stats::biggest;

let big = biggest(&[4, 9, 2]);
```

`use` doesn't load any files. It just saves typing. The `mod` line is what connects a file.

## Your challenge

Run `cargo run` first. You'll see several errors. Fix them one at a time.

1. **`shapes.rs`** exists and is declared, but `main.rs` can't use its functions because they're private. Mark both functions `pub`.
2. **`math`** isn't connected. Add `mod math;` to `main.rs`. Then, in `src/math/mod.rs`, declare its `stats` module publicly.
3. **In `src/math/stats.rs`**, finish `smallest`.
4. **Create a new file** called `src/greetings.rs`. In VS Code, right-click the `src` folder and choose **New File**. Write a public function `hello` that takes a `&str` name and gives back `"Hello, Sam!"`. Then connect it from `main.rs`.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`.

## Stuck?

- **`E0603` "function is private":** add `pub` before `fn`.
- **`E0433` "failed to resolve"** or **`E0425`:** a `mod` line is missing in the parent file.
- **`E0583` "file not found for module":** the file's name or location doesn't match the `mod` line. `mod greetings;` needs `src/greetings.rs`.
- **`stats.rs` needs:** `pub fn smallest(numbers: &[i32]) -> i32`, which can work just like `biggest`.

## New words

- **Module**: a named section of code, usually one file.
- **Crate**: a whole Rust project, either a program or a library. `crate::` means its top.
- **Path**: the route to something, like `math::stats::biggest`.
- **Public / private**: whether other modules can use something. Private is the default.
