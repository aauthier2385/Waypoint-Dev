# Lesson 23: Project: a to-do list app

**What you'll learn**
- How to build a real tool that saves its data between runs
- Reading and writing files
- Reading commands typed after the program's name
- Putting structs, `Vec`, `Result`, modules, and tests together

## The project

You'll finish a to-do list you control from the terminal:

```
cargo run -- add "Buy milk"
cargo run -- add "Walk the dog"
cargo run -- done 1
cargo run -- list
```

```
1. [x] Buy milk
2. [ ] Walk the dog
2 tasks, 1 done
```

Your list is saved in a file called `todo.txt`, so it's still there the next time you run the program.

The `--` in `cargo run -- add ...` means "everything after this goes to my program, not to Cargo."

## How it's put together

| File | What it does | Status |
| --- | --- | --- |
| `src/main.rs` | Reads the command you typed, calls the library, and prints the result | Finished |
| `src/lib.rs` | The `Task` and `TodoList` types and everything they can do | **Yours to finish** |

Like the guessing game and Lesson 18, all the decisions live in `lib.rs`, where they're easy to test. `main.rs` only connects the pieces.

## New tools in this project

### Reading what was typed: `std::env::args`

```rust
let args: Vec<String> = std::env::args().collect();
```

`args[0]` is the program's own name, `args[1]` is the command, like `add`, and so on. `main.rs` uses `match` on the command to decide what to do.

### Files: `std::fs`

```rust
let text = std::fs::read_to_string("todo.txt")?;    // read a whole file into a String
std::fs::write("todo.txt", text)?;                  // replace a file's contents
```

Both give back a `Result`, because files can be missing, or you might not have permission to change them. `save` and `load` in `lib.rs` are finished. Read them to see how the errors are handled. A missing file isn't an error here, just an empty list.

### Saving data as text

Your list is stored in a simple format, one task per line:

```
[x] Buy milk
[ ] Walk the dog
```

Turning data into text so it can be saved or sent is called **serializing**. Turning that text back into data is **deserializing**, or **parsing**. Real programs often use formats like JSON with a crate like `serde`, but the idea is exactly the same.

## Counting from 1 versus from 0

People number tasks from 1. `Vec` positions start at 0. So task number 1 is `self.tasks[0]`, and task `n` is `self.tasks[n - 1]`. Before using a number, check that it's between 1 and the number of tasks. Otherwise the program will panic, or try to compute `0 - 1`, which can't exist for a `usize`.

## Your challenge

Open `src/lib.rs` and replace each `todo!()`:

1. **`add`**: add a new, unfinished task. Remove spaces from the ends of the title. An empty title gives back `Err("A task needs a title")`.
2. **`complete`**: mark task number `n` as done. If there's no such task, give back `Err("There's no task 5")`, with the real number.
3. **`remove`**: take task `n` out of the list and give it back, with the same error as `complete`. `Vec` has a `.remove(position)` method.
4. **`summary`**: `"3 tasks, 1 done"`. Use `"1 task"` for exactly one task.
5. **`to_text`**: every task as a line, `[x] Title` or `[ ] Title`, each ending in a new line, `\n`.
6. **`from_text`**: the reverse. Skip blank lines and lines that don't start with `[x] ` or `[ ] `.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`. Then use your app for real with the commands at the top of this lesson.

## Stuck?

- **Checking the number:** `if n == 0 || n > self.tasks.len() { return Err(format!("There's no task {n}")); }`.
- **`to_text`:** start with `String::new()`, and for each task, `push_str` either `"[x] "` or `"[ ] "`, then the title, then `"\n"`.
- **`from_text`:** for each `line` in `text.lines()`, `line.strip_prefix("[x] ")` gives back `Some(title)` if the line starts with it.

## Make it yours

- Add an `undo` command that marks a task as not done.
- Add `clear` to remove every finished task.
- Show unfinished tasks first.

## You finished the Intermediate level!

You can now design your own types, handle errors properly, organize code across files, use other crates, write tests, and build tools that keep data between runs. The Advanced level covers what's left in Rust's toolbox: lifetimes, smart pointers, running code at the same time, macros, performance, and more.

## New words

- **Command-line arguments**: the words typed after a program's name.
- **Serialize / deserialize**: turn data into text or bytes, and back again.
- **Persist**: keep data after the program stops, usually in a file.
