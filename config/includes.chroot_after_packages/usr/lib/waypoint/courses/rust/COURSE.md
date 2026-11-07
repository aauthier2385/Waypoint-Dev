# The Waypoint Rust Course

Welcome! This course takes you from never having written code to building real programs in Rust. You don't need any experience. Every new word is explained the first time it shows up.

## What is Rust?

A program is a list of instructions for a computer. Computers only understand very simple instructions, so people write programs in a **programming language** that's easier for humans to read. Then a tool translates it into something the computer can run.

Rust is one of those languages. It's known for being fast and for catching mistakes *before* your program runs. That second part matters a lot when you're learning: when something is wrong, Rust usually tells you exactly where and why.

## How this course works

Every lesson is a folder, and every folder is a real Rust project. That means everything you practice here is exactly how real Rust projects work.

In each lesson:

1. **Read the lesson.** It opens as `README.md` on one side of VS Code.
2. **Try the examples.** The steps tell you exactly what to click and type.
3. **Do the challenge.** You'll change the code in the lesson's folder.
4. **Check your work.** Press **Ctrl+Shift+B**, or type `dev learn check` in the terminal. The checker tells you, in plain English, what's right and what still needs work.

When you pass, the checker tells you which lesson is next. You can close everything and come back later. Just open **Rust Course** from the applications menu, or type `dev learn` in a terminal, and it picks up where you left off.

**Getting errors is normal.** Professional programmers see error messages all day. They're not a sign you're doing badly. They're Rust's way of helping. This course teaches you how to read them.

## Getting around VS Code

- **Explorer** (top icon on the left): shows every lesson folder and file. Click a file to open it.
- **Editor** (the big middle area): where you read lessons and change code. Files save on their own after a moment.
- **Terminal** (menu **Terminal > New Terminal**): a place to type commands. It opens at the bottom.
- **Ctrl+Shift+B**: checks the lesson for the file you're looking at.

If VS Code asks **"Do you trust the authors of the files in this folder?"**, click **Yes, I trust the authors**. These are the course's own files, and the Check button won't work otherwise.

## The course map

### Beginner: the basics

| Lesson | What you'll learn |
| --- | --- |
| 01 Your first program | Running a program and changing what it says |
| 02 Inside a Rust project | What every file and folder does, and how the tools work together |
| 03 Variables | Storing information and changing it |
| 04 Types of data | Whole numbers, decimals, true/false, and text |
| 05 Functions | Naming a set of steps so you can reuse it |
| 06 Making decisions | Doing different things depending on the situation |
| 07 Repeating things | Loops |
| 08 Project: guessing game | Putting it all together in a game you can play |

### Intermediate: thinking like a Rust programmer

| Lesson | What you'll learn |
| --- | --- |
| 09 Ownership | Who's responsible for data, and why Rust "moves" values |
| 10 Borrowing and references | Lending values with `&` and `&mut` |
| 11 Working with text | `String` and `&str`, slices, and text tools |
| 12 Structs | Making your own types, with methods |
| 13 Enums and match | Types with choices, and handling every case |
| 14 Option | Handling "there might be nothing here" safely |
| 15 Handling errors with Result | `Ok`, `Err`, and the `?` operator |
| 16 Lists and lookup tables | `Vec` and `HashMap` |
| 17 Splitting code across files | Modules, `mod`, `use`, and `pub` |
| 18 Using other people's code | Crates, dependencies, and `Cargo.lock` |
| 19 Traits | Abilities that many types can share |
| 20 Generics | One piece of code for many types |
| 21 Iterators and closures | Processing lists step by step |
| 22 Writing your own tests | Finding and fixing bugs with tests |
| 23 Project: a to-do list app | A real tool that saves to a file |

### Advanced: building serious software

| Lesson | What you'll learn |
| --- | --- |
| 24 Lifetimes | Making sure borrowed values never outlive what they borrow |
| 25 Smart pointers | `Box`, `Rc`, `RefCell`, and recursion |
| 26 Threads | Doing several things at once, safely |
| 27 Async Rust | Waiting without wasting time |
| 28 Macros | Code that writes code |
| 29 Unsafe Rust and talking to C | Raw pointers, C functions, and safe wrappers |
| 30 Making programs fast | Release builds, Big O, and measuring |
| 31 Workspaces | Projects made of several crates |
| 32 Documentation and publishing | Doc comments, doc tests, and crates.io |
| 33 Project: a web server | How the web works, built from scratch |

Improvements arrive through Waypoint's updates. Lessons you've already started are never changed or replaced.

## Reference

**FILES.md** explains every kind of file and folder you'll meet in a Rust project, and how they all fit together. Keep it handy. You'll come back to it.
