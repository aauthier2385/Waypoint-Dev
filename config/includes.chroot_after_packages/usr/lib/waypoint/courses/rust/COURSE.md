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

### Intermediate: thinking like a Rust programmer (coming in an update)

Ownership, borrowing, text in depth, structs, enums and `match`, `Option`, error handling with `Result` and `?`, lists and lookup tables (`Vec` and `HashMap`), splitting code across files with modules, using other people's code (crates), traits, generics, iterators and closures, writing your own tests, and a to-do list app that saves to a file.

### Advanced: building serious software (coming in an update)

Lifetimes, smart pointers, running code at the same time with threads, async Rust, macros, unsafe Rust and talking to C, making programs faster, workspaces with many crates, documenting and publishing, and a small web server.

New lessons arrive through Waypoint's updates. Your finished lessons are never changed or replaced.

## Reference

**FILES.md** explains every kind of file and folder you'll meet in a Rust project, and how they all fit together. Keep it handy. You'll come back to it.
