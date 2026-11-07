# Lesson 16: Lists and lookup tables

**What you'll learn**
- `Vec`: a list that can grow and shrink
- `HashMap`: a lookup table that matches keys to values
- Adding, reading, and looping over both
- The `entry` trick for counting things

## The idea: containers for many things

So far, each variable has held one value or a fixed handful. Real programs juggle collections: a playlist of songs, a list of scores, a phone book of names and numbers. Rust's standard library gives you ready-made containers for these.

| Container | Everyday comparison | Use it for |
| --- | --- | --- |
| `Vec<T>` | A numbered list | Things in order, like scores or to-dos |
| `HashMap<K, V>` | A dictionary or phone book | Looking something up by name |

## `Vec`: a list that grows

`Vec` is short for *vector*, a math word for a list of things.

```rust
let mut scores: Vec<i32> = Vec::new();   // empty list of i32s
scores.push(10);                         // add to the end
scores.push(25);

let more = vec![1, 2, 3];   // shortcut for a list with items already in it

println!("{}", scores[0]);        // 10, the first item. Positions start at 0.
println!("{}", scores.len());     // 2, how many items
```

**Positions start at 0, not 1.** The first item is `scores[0]`. Asking for a position that doesn't exist, like `scores[99]`, panics. When you're not sure it exists, `scores.get(99)` gives back an `Option` instead.

Looping:

```rust
for score in &scores {       // borrow each one to look
    println!("{score}");
}
```

`&[i32]` in a function's inputs means "a borrowed list of i32s." It accepts a borrowed `Vec`, like `&scores`, or a fixed list, like `&[1, 2, 3]`.

Useful tools: `.contains(&5)`, `.is_empty()`, `.sort()`, `.pop()` to remove the last item and get it as an `Option`, and `.iter().sum::<i32>()` to add them all up.

## `HashMap`: look things up by name

```rust
use std::collections::HashMap;

let mut ages: HashMap<String, u32> = HashMap::new();
ages.insert("Sam".to_string(), 34);
ages.insert("Ana".to_string(), 27);

match ages.get("Sam") {
    Some(age) => println!("Sam is {age}"),
    None => println!("I don't know Sam"),
}
```

The thing you look up by, `"Sam"`, is the **key**. What you find, `34`, is the **value**. `.get` gives back an `Option`, because the key might not be there.

`use std::collections::HashMap;` at the top brings `HashMap` into your file. Unlike `Vec`, it isn't available automatically.

**HashMaps don't keep any order.** If you loop over one, the items can come out in any order. That's fine for looking things up, but sort the keys first if you need them in order.

## The `entry` trick: counting

To count things, you need "if this word is new, start it at 0, and then add 1." `entry` does that in one line:

```rust
*counts.entry(word).or_insert(0) += 1;
```

Read it as: "find this word's spot, putting a 0 there if it's new, then add 1 to whatever is there." The `*` is there because `or_insert` gives back a reference to the number, as you saw in Lesson 10.

## Your challenge

Open `src/main.rs` and replace each `todo!()`:

1. **`average`**: the average of a list, or `None` if it's empty.
2. **`evens`**: a new `Vec` with only the even numbers, in the same order.
3. **`remove_duplicates`**: a new `Vec` with each number only once, keeping the first time each appears. `[3, 1, 3, 2, 1]` gives back `[3, 1, 2]`.
4. **`word_counts`**: how many times each word appears, in lowercase. `"the cat and THE hat"` counts `"the"` twice.
5. **`add_stock`**: add an amount to an item in an inventory, starting at 0 for new items.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`.

## Stuck?

- **`average`:** check `numbers.is_empty()` first. Add them up with a loop, then divide by `numbers.len() as f64`.
- **`remove_duplicates`:** build a new `Vec`, and only `push` a number `if !result.contains(&number)`.
- **`word_counts`:** loop over `text.split_whitespace()`, and use `word.to_lowercase()` as the key.
- **`add_stock`:** `*inventory.entry(item.to_string()).or_insert(0) += amount;`.

## New words

- **Vec**: a growable list.
- **Index**: an item's position in a list, starting at 0.
- **HashMap**: a table that matches keys to values.
- **Key / value**: what you look up by, and what you find.
