# Lesson 02: Inside a Rust project

**What you'll learn**
- What every file and folder in a Rust project is for
- How VS Code, the terminal, Cargo, and rustc work together
- What to do when Cargo can't find your code

## The idea

A Rust project is a folder with a specific layout. Cargo expects certain files in certain places, the way a mail carrier expects your mailbox at the front of the house. If something is in the wrong place, Cargo can't find it.

This lesson's project is broken on purpose: one file is in the wrong place. You'll learn the layout, then fix it.

## The files in a Rust project

Open the **Explorer** on the left and expand `01-first-program`. After you ran it in Lesson 01, it looks like this:

```
01-first-program/
├── Cargo.toml     ← the project's ID card
├── Cargo.lock     ← the exact receipt of everything used
├── .gitignore     ← tells Git what not to save
├── lesson.toml    ← only in this course: how to check the lesson
├── README.md      ← the lesson you read
├── src/           ← "source": where your code lives
│   └── main.rs    ← where the program starts
└── target/        ← Cargo's workshop, where finished programs go
```

### `Cargo.toml`

Click it to open it. It looks like this:

```toml
[package]
name = "first-program"
version = "0.1.0"
edition = "2021"

[dependencies]
```

- `[package]` starts the section that describes this project.
- `name` is the program's name.
- `version` is which version of *your* program this is. You'd raise it when you release changes.
- `edition` is which edition of Rust's rules to use. Rust adds an edition every few years, and old code keeps working.
- `[dependencies]` is where you'd list other people's code your project needs. It's empty for now.

`.toml` is a file type for settings. It uses `name = value` lines grouped under `[headings]`.

### `src/` and `main.rs`

`src` is short for **source code**, the code people write. **This is the only folder where Cargo looks for your Rust code.** `.rs` files contain Rust code, and `main.rs` is the special file where a program starts.

### `Cargo.lock`

Cargo made this the first time it built your project. It records the exact version of everything your project used, so it builds the same way on any computer. Don't edit it. Cargo handles it.

### `target/`

When Cargo builds your code, it puts the results here. Look inside `target/debug/` and you'll find a file called `first-program`. **That file is your finished program.** It's what actually ran when you typed `cargo run`.

Everything in `target/` can be rebuilt, so it's safe to delete. Git ignores it because it's big and anyone can recreate it.

### `.gitignore`

It lists what Git should not save, which here is just `target/`. Files that start with a dot are hidden by default in many places, which is why you may not have noticed it.

## How everything works together

```
You edit src/main.rs in VS Code
        │  (VS Code saves the file)
        ▼
You type "cargo run" in the terminal
        │
        ▼
Cargo reads Cargo.toml  →  finds src/main.rs
        │
        ▼
rustc translates your code  →  error? It stops and explains.
        │
        ▼
Program saved to target/debug/  →  Cargo runs it  →  output in the terminal
```

VS Code is only for reading and writing files. The terminal is where you give commands. Cargo is the manager that does the building, and rustc is the translator it hires. Each tool has one job, and the files are how they pass work to each other.

The **FILES.md** guide in the course folder explains every other file you'll meet, so keep it handy.

## Your challenge

Look in this lesson's folder, `02-inside-a-project`. `main.rs` is sitting on its own, not inside `src`. Try running it first so you can see the error:

```
cd ../02-inside-a-project
cargo run
```

`cd ..` means "go up one folder," so `cd ../02-inside-a-project` goes up out of Lesson 01 and into Lesson 02. Cargo will complain that there are **no targets**, meaning it found nothing to build.

**Fix it by moving `main.rs` into a new `src` folder.** Choose either way:

**Using VS Code**
1. In the Explorer, right-click the `02-inside-a-project` folder and choose **New Folder**.
2. Name it `src` and press **Enter**.
3. Drag `main.rs` onto the `src` folder.

**Using the terminal**

```
mkdir src
mv main.rs src/
```

`mkdir` means *make directory*. `mv` means *move*.

Then run `cargo run` again. You should see:

```
Cargo found my code!
```

## Check your work

Press **Ctrl+Shift+B** with `src/main.rs` open, or run `dev learn check` in the terminal.

## Extra credit

Make a new project of your own from scratch:

```
cd ~/Projects
cargo new my-first-rust
```

Then look inside it. Cargo made `Cargo.toml`, `src/main.rs`, and `.gitignore` for you, plus a Git history in `.git/`. Every Rust project starts like this.

## New words

- **Source code**: the code people write. That's why the folder is called `src`.
- **Binary**: a finished program the computer can run, like the file in `target/debug/`.
- **Directory**: another word for folder.
- **Extension**: the letters after the last dot in a file name, like `.rs` or `.toml`.
