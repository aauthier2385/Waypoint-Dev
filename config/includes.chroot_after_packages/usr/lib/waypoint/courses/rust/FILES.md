# Every File and Folder, Explained

This is your reference guide. It explains what each file and folder in a Rust project is for, and how all the pieces work together. You don't need to memorize it. Come back whenever you see something you don't recognize.

## First: files, folders, and extensions

A **file** is one saved document, like a photo or a letter. A **folder** (also called a *directory*) holds files and other folders, like a drawer.

The letters after the last dot in a file's name are its **extension**. They tell you, and your computer, what kind of file it is. `photo.jpg` is a picture. `main.rs` is Rust code.

You may hear people say "main" and "src" on their own. Those are short for `main.rs`, a **file**, and `src`, a **folder**. They're the two most important parts of a Rust project.

## The big picture: how everything works together

Here's what happens from typing code to seeing your program run:

```
 You type code in VS Code
          │
          ▼
 src/main.rs  ◄── your code, saved as a file
          │
 You type "cargo run" in the terminal
          │
          ▼
 Cargo reads Cargo.toml ──► learns your project's name and what it needs
          │
          ▼
 Cargo asks rustc to translate src/main.rs into computer instructions
          │
          ├── Mistake found? rustc prints an error. Nothing runs.
          │
          ▼
 The finished program is saved in target/debug/
          │
          ▼
 Cargo runs it, and what it prints appears in the terminal
```

Git and GitHub sit alongside this. Git keeps a history of your files, and GitHub stores a copy online when you push. Neither one is needed to build or run code.

## The tools

| Tool | What it is | Everyday comparison |
| --- | --- | --- |
| **VS Code** | The program where you read and write code | A word processor for code |
| **Terminal** | A window where you type commands | Texting instructions to your computer |
| **rustc** | The Rust *compiler*: translates your code into a program | A translator |
| **Cargo** | Rust's project manager. It runs rustc for you, runs tests, and downloads other people's code | A general contractor who hires the translator and orders supplies |
| **Git** | Keeps snapshots of your project over time | Save points in a video game |
| **GitHub** | A website that stores copies of Git projects online | A cloud backup you can share |
| **dev** | Waypoint's helper command, like `dev learn check` | Your study buddy |

You almost never run `rustc` yourself. You run Cargo, and Cargo runs rustc.

## Inside a Rust project

```
my-project/            ← the project folder. Its name doesn't matter to Rust.
├── Cargo.toml         ← the project's ID card and shopping list
├── Cargo.lock         ← exact versions of everything used (made automatically)
├── src/               ← "source": all your Rust code lives here
│   ├── main.rs        ← where your program starts
│   ├── lib.rs         ← (optional) reusable code other files can use
│   └── other.rs       ← (optional) more code, split up by topic
├── tests/             ← (optional) extra tests that check your code from outside
├── examples/          ← (optional) small example programs
├── target/            ← Cargo's workshop: finished programs (made automatically)
├── .gitignore         ← tells Git which files not to save
└── README.md          ← a description of the project for people
```

### `Cargo.toml`: the ID card and shopping list

`.toml` files hold settings in a simple `name = value` style. `Cargo.toml` tells Cargo about your project:

```toml
[package]
name = "my-project"     # the program's name
version = "0.1.0"       # which version this is
edition = "2021"        # which edition of the Rust language rules to use

[dependencies]
# Other people's code this project uses goes here, like:
# rand = "0.8"
```

- `[package]` describes this project.
- `[dependencies]` lists **crates**, packages of code other people wrote, that your project needs. Cargo downloads them for you.
- `[dev-dependencies]` lists crates only needed for testing.
- Anything after a `#` is a comment for humans.

Every Rust project has exactly one `Cargo.toml` at its top level. Cargo finds it by looking in the folder you're in, then the folder above, and so on.

### `Cargo.lock`: the exact receipt

Cargo creates this file the first time you build. Where `Cargo.toml` says "I need a crate called rand, version 0.8 or newer," `Cargo.lock` writes down the *exact* version that was used. That way the project builds the same way on every computer. **Don't edit it by hand.** Cargo keeps it up to date.

### `src/`: your code

`src` is short for **source**, as in source code: the code people write. Cargo looks here, and only here, for your Rust files.

### `.rs` files: Rust code

Every file ending in `.rs` contains Rust code.

- **`src/main.rs`** is where a program starts. Inside it, the `fn main()` function runs first, every time. A project with `main.rs` is a **binary crate**, a program you can run.
- **`src/lib.rs`** holds code meant to be used by other code, not run directly. A project with `lib.rs` is a **library crate**. A project can have both.
- **Other `.rs` files**, like `src/scores.rs`, are **modules**: code split into topics so it's easier to find.

### How `.rs` files connect to each other

Files don't see each other automatically. You connect them:

```rust
// In src/main.rs
mod scores;          // "There's a module called scores. Look in src/scores.rs."
use scores::total;   // "I want to use the total function from it."

fn main() {
    println!("{}", total());
}
```

```rust
// In src/scores.rs
pub fn total() -> u32 {   // "pub" means "other files are allowed to use this"
    42
}
```

- `mod` tells Rust that a module exists and where to find it.
- `use` creates a shortcut so you don't have to type the full path every time.
- `pub` stands for public. Without it, things are private to their own file.

For a folder full of modules, Rust looks for `src/scores/mod.rs` or `src/scores.rs`. You'll practice all of this in the Intermediate lesson on modules.

### `target/`: Cargo's workshop

When Cargo builds your code, it puts everything it makes here.

- `target/debug/` holds normal builds. They're quick to make, slower to run, and easier to troubleshoot.
- `target/release/` holds builds from `cargo build --release`. They take longer to make but run much faster, so they're what you share with others.
- The finished program has no extension on Linux. In `target/debug/my-project`, the file named `my-project` *is* your program.

Everything in `target/` can be rebuilt, so it's safe to delete. `cargo clean` deletes it for you. It can get large, which is why Git ignores it.

### `tests/`, `examples/`, and `benches/`

- `tests/` holds **integration tests**: separate files that use your code the way someone else would. `cargo test` runs them.
- `examples/` holds small example programs. Run one with `cargo run --example name`.
- `benches/` holds **benchmarks**, which measure how fast code runs.

Tests can also live inside `.rs` files, in a section marked `#[cfg(test)]`. That's how the checker works in this course's lessons.

### Other files you'll see

| File | What it does |
| --- | --- |
| `README.md` | Describes the project. `.md` means **Markdown**: plain text with simple marks for headings, lists, and bold. GitHub shows it on the project's page. |
| `.gitignore` | Lists files Git should *not* save, like `target/`. A name that starts with a dot is hidden by default. |
| `.git/` | Git's history of your project. Created by `git init`. Don't change it by hand. |
| `.vscode/` | Settings for VS Code in this project. `tasks.json` defines buttons like "Check my work." `settings.json` holds editor preferences. |
| `.json` | Settings in a format with curly braces `{ }`, used by VS Code and many other tools. |
| `lesson.toml` | Only in this course: tells the checker how to check the lesson. |
| `build.rs` | Advanced: code that Cargo runs *before* building your project, for special setup steps. |
| `.cargo/config.toml` | Advanced: changes how Cargo behaves for this project. |
| `rust-toolchain.toml` | Advanced: says which version of Rust the project needs. |
| `LICENSE` | Says what others are allowed to do with your code. |

## Commands you'll use most

| Command | What it does |
| --- | --- |
| `cargo run` | Builds your program, then runs it |
| `cargo build` | Builds it without running it |
| `cargo check` | Looks for mistakes without building a program. It's the fastest way to check. |
| `cargo test` | Runs the tests |
| `cargo new name` | Creates a brand-new project with `Cargo.toml` and `src/main.rs` |
| `cargo clean` | Deletes the `target/` folder |
| `cd folder` | Moves the terminal into a folder. `cd ..` goes up one level. |
| `ls` | Lists what's in the current folder |
| `dev learn` | Opens this course where you left off |
| `dev learn check` | Checks the lesson you're in |
| `dev learn status` | Shows which lessons you've finished |

## What goes to GitHub

When you push a project, Git uploads `Cargo.toml`, `Cargo.lock`, `src/`, `tests/`, `README.md`, and the rest of your files. It skips anything listed in `.gitignore`, which is why `target/` never gets uploaded. Anyone who downloads your project can rebuild it with `cargo build`.
