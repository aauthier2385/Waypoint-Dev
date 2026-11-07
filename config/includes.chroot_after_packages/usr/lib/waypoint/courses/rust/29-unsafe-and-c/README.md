# Lesson 29: Unsafe Rust and talking to C

**What you'll learn**
- What `unsafe` means, and what it doesn't mean
- Raw pointers
- Calling functions written in C
- Wrapping unsafe code in a safe function, which is the most important skill here

## The idea: the safety rails come off, on purpose

Everything Rust has checked for you so far, such as ownership, borrowing, and lifetimes, keeps your code safe. But some work can't be checked by the compiler. Talking directly to hardware, calling code written in another language, or squeezing out the last bit of speed are examples.

For that, Rust has `unsafe`. Think of a power tool with its guard removed: sometimes the job needs it, but now **you** are responsible for keeping your fingers clear.

```rust
unsafe {
    // code here can do a few extra things the compiler can't check
}
```

**`unsafe` doesn't turn off Rust's checks.** Ownership and borrowing still apply. It unlocks exactly five extra abilities, mainly:

1. Following a **raw pointer**.
2. Calling an `unsafe` function, which includes every C function.
3. A few rarer ones: changing global variables, and implementing certain special traits.

Writing `unsafe` is a promise: "I've checked this myself, and it's correct." When something goes badly wrong in a Rust program, you only need to look inside `unsafe` blocks.

## Raw pointers

A reference (`&`) always points to something valid, and Rust guarantees it. A **raw pointer**, written `*const T` (read-only) or `*mut T` (changeable), is just an address, with no guarantees at all. It could point to nothing, or to memory that's already been freed.

```rust
let mut number = 5;
let pointer = &mut number as *mut i32;   // making one is safe
unsafe {
    *pointer += 1;                       // following one isn't
}
```

## Talking to C

C is the oldest and most widely used language for system software. Linux itself and countless libraries are written in it. Rust can call C functions directly. Calling code in another language is called **FFI**, the *foreign function interface*.

You describe the C functions in an `extern "C"` block:

```rust
use std::os::raw::c_int;

extern "C" {
    fn abs(n: c_int) -> c_int;     // from the C standard library, which every Linux program has
}

let answer = unsafe { abs(-7) };   // 7
```

Rust can't see inside C code, so it can't check it. That's why every C call is `unsafe`.

### Text is different in C

C marks the end of text with a hidden zero byte, and passes text around as a raw pointer to its first letter. Rust text doesn't work that way. `CString` converts between them:

```rust
use std::ffi::CString;

let c_text = CString::new("hello").unwrap();   // fails if the text contains a zero byte
let pointer = c_text.as_ptr();                  // a *const c_char that C understands
```

## The golden rule: a safe wrapper

Good Rust code keeps `unsafe` small and hidden inside a **safe function**: one that checks everything needed, so whoever calls it can't misuse it. Rust's own standard library is built this way. `Vec` uses `unsafe` inside, but every method you call on it is safe.

```rust
fn second(numbers: &[i32]) -> Option<i32> {
    if numbers.len() < 2 {
        return None;                                  // check first...
    }
    Some(unsafe { *numbers.get_unchecked(1) })        // ...so this is always valid
}
```

`get_unchecked` skips the bounds check that `numbers[1]` does, so it's slightly faster, and dangerous if the position doesn't exist. The `if` above makes it safe. Add a comment saying *why* each `unsafe` block is correct. Rust programmers call it a **SAFETY comment**.

## Your challenge

Open `src/main.rs`.

1. **`c_abs`** doesn't build. Read the `E0133` error and fix it.
2. **`c_length`**: count the letters in some text using C's `strlen`.
3. **`swap_through_pointers`**: swap two numbers using raw pointers.
4. **`read_at`**: a safe wrapper that gives back the number at a position, or `None` if the position doesn't exist. Use `get_unchecked`, and make sure it's only called for valid positions.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`.

## Stuck?

- **`c_length`:** `let text = CString::new(text).unwrap();`, then `unsafe { strlen(text.as_ptr()) }`. Keep the `CString` in a variable, so it's still alive while C reads it.
- **`swap_through_pointers`:** turn both into `*mut i32` with `as`, then inside `unsafe`, `let temporary = *a; *a = *b; *b = temporary;`, using the pointers.
- **`read_at`:** `if index >= numbers.len() { return None; }`.

## New words

- **Unsafe**: code where you take responsibility for checks the compiler can't do.
- **Raw pointer**: an address with no safety guarantees, written `*const T` or `*mut T`.
- **FFI**: calling code written in another language.
- **Safe wrapper**: a safe function that hides carefully checked `unsafe` code inside.
