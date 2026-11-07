# Lesson 01: Your first program

**What you'll learn**
- How to run a Rust program
- How to change what a program says
- How to check your work

## The idea

A program is a list of instructions the computer follows from top to bottom, like a recipe. Your first program has one instruction: show the words `Hello, world!` on the screen.

Writing a "Hello, world!" program is a tradition. Almost every programmer's first program in a new language looks just like this one.

## Look at the code

Open `src/main.rs` in this lesson's folder. You'll see:

```rust
fn main() {
    println!("Hello, world!");
}
```

Line by line, in plain English:

- `fn main()` means "here's a set of steps called **main**." `fn` is short for *function*, which is a named set of steps. Rust always starts a program by running `main`.
- `{` and `}` are curly braces. Everything between them is part of `main`. Think of them as the start and end of the recipe.
- `println!("Hello, world!");` means "print this line of text."
  - `println` is short for *print line*. Printing here means showing text in the terminal, not using a printer.
  - The `!` means `println!` is a **macro**, a special kind of instruction. For now, just know it always has `!`.
  - Text goes inside double quotes: `"like this"`.
  - The `;` semicolon marks the end of an instruction, like a period at the end of a sentence.

## Try it: run the program

1. Open the terminal: from the top menu, choose **Terminal > New Terminal**. A panel opens at the bottom.
2. The terminal starts in the course folder. Move into this lesson's folder by typing this and pressing **Enter**:

   ```
   cd 01-first-program
   ```

   `cd` means *change directory*, which is another way of saying "go into this folder."
3. Run the program:

   ```
   cargo run
   ```

The first time, Cargo builds your program, which takes a few seconds. Then you'll see:

```
Hello, world!
```

That's your program running. You just ran Rust code!

## What just happened

When you typed `cargo run`:

1. **Cargo** read `Cargo.toml` to learn the project's name.
2. Cargo found your code in `src/main.rs`.
3. Cargo asked **rustc**, the Rust translator, to turn your code into a program the computer can run.
4. The finished program was saved in a new folder called `target`.
5. Cargo ran it, and it printed `Hello, world!`.

You'll learn about every one of these files in the next lesson.

## Your challenge

Make the program print **two** lines, exactly like this:

```
Hello, world!
I am learning Rust!
```

Steps:

1. In `src/main.rs`, click at the end of the `println!` line.
2. Press **Enter** to make a new line.
3. Type a second `println!` instruction with the new words inside the quotes.
4. Run `cargo run` again to see your change.

## Check your work

Press **Ctrl+Shift+B** while `src/main.rs` is open. You can also type this in the terminal, while you're inside the lesson folder:

```
dev learn check
```

If something's not right, the checker explains what it expected and what your program printed.

## Stuck?

- **The words must match exactly**, including capital letters and the `!` at the end.
- **Every instruction needs a `;`** at the end.
- **Text needs quotes on both sides.**
- **If you see red error text**, read it slowly. Rust usually points to the exact line with a `^` marker.

## New words

- **Program**: a list of instructions for a computer.
- **Function**: a named set of steps. `main` is the one that runs first.
- **Terminal**: a window where you type commands.
- **Cargo**: Rust's tool for building and running projects.
- **Compile**: translate code into a program the computer can run. That's rustc's job.
