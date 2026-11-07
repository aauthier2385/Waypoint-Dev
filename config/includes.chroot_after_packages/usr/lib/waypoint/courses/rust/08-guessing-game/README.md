# Lesson 08: Project: guessing game

**What you'll learn**
- How to combine variables, types, functions, decisions, and loops into one program
- How a program reads what you type
- How larger programs are split into small, testable functions

## The project

You'll finish a game you can actually play. The computer picks a secret number from 1 to 100. You guess, and it tells you whether you're too low or too high until you get it.

The parts that need new skills are already written. Your job is to write the three functions that make the game's decisions.

## How the program is put together

Open `src/main.rs`. It's longer than before, so here's a map:

| Part | What it does |
| --- | --- |
| `use std::io;` | Brings in Rust's tools for reading what you type. `std` is the **standard library**, the code that comes with Rust. |
| `fn main()` | Runs the game: picks a secret, then loops forever asking for guesses. |
| `pick_secret()` | Uses the clock to pick a different number each game. It's finished. |
| `is_valid_guess()` | **You write this.** Is the guess from 1 to 100? |
| `compare()` | **You write this.** Is the guess too low, too high, or right? |
| `score_message()` | **You write this.** What to say at the end, based on how many tries it took. |
| `mod tests` | The checker. |

This is how real programs are built. `main` handles the flow: ask, read, respond, repeat. The decisions live in small functions that are easy to test on their own. If `compare` has a mistake, its test points right at it, instead of you having to play the game over and over to find it.

## Walking through `main`

Read `main` from top to bottom. Each part uses something you've learned:

1. `let secret = pick_secret();` stores the secret in a variable (Lesson 03).
2. `let mut tries = 0;` makes a counter that can change (Lesson 03).
3. `loop { ... }` repeats until the game ends (Lesson 07).
4. `io::stdin().read_line(&mut input)` waits for you to type a line and press **Enter**, then puts the text in `input`. The `&mut` lets `read_line` change `input`. You'll learn exactly what that means in the Ownership and Borrowing lessons.
5. `input.trim().parse()` removes the **Enter** key from the end of the text, then tries to turn it into a number. `match` handles both outcomes: if it worked, keep the number, and if it didn't, ask again. You'll learn `match` properly in the Intermediate level.
6. `continue` skips back to the start of the loop to ask again (Lesson 07).
7. `return` ends `main`, which ends the program.

## Your challenge

Replace each `todo!()`:

1. **`is_valid_guess`**: give back `true` if the guess is from 1 to 100, including 1 and 100.
2. **`compare`**: give back `"Too low!"` if the guess is less than the secret, `"Too high!"` if it's more, and `"You got it!"` if they match.
3. **`score_message`**:
   - 1 try: `"Amazing! First try!"`
   - 2 to 6 tries: `"Nice work! You took 4 tries."`, using the real number of tries
   - 7 or more: `"You got there! It took 9 tries."`, using the real number

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`.

## Then play it

Once the checks pass:

```
cd ../08-guessing-game
cargo run
```

Type a number and press **Enter**. Press **Ctrl+C** to quit at any time.

## Make it yours

These aren't checked, so try anything you like:

- Change the range to 1 to 1,000. Update `pick_secret`, `is_valid_guess`, and the messages.
- Tell the player when they're within 5 of the secret: "So close!"
- Give the player only 10 tries, then reveal the number.

If you change the tests' expected messages without updating the tests, the checker will fail. That's fine once you've already finished the lesson.

## You finished the Beginner level!

You can now write programs that store information, make decisions, repeat steps, and respond to what someone types. In the Intermediate level, you'll learn the ideas that make Rust different from other languages, starting with **ownership**, and you'll split programs across several files.

## New words

- **Standard library**: the code that comes with Rust, called `std`.
- **Input**: information a program receives, like what you type.
- **Parse**: turn text into something else, like a number.
