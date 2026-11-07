# Lesson 27: Async Rust: waiting without wasting time

**What you'll learn**
- What "async" means, and how it differs from threads
- `async fn`, `.await`, and futures
- Running two waits at the same time with `join`
- What a runtime is, and why real projects use tokio

## The idea: a waiter with many tables

A restaurant waiter takes an order and sends it to the kitchen. They don't stand frozen beside the kitchen door until the food is ready. They go take another table's order, refill drinks, and come back when a dish is up. One waiter, many tables, very little wasted time.

Lots of programming is waiting: for a website to answer, a file to load, a database to reply. **Async** code lets a program start something slow, go do other work, and pick it back up when it's ready. That's exactly what the waiter does.

**Async versus threads:** threads are like hiring more cooks. They do more *work* at the same time, using more cores. Async is like one efficient waiter. It does more *waiting* at the same time, without needing more workers. Web servers handling thousands of connections lean on async, because each connection mostly waits.

## `async fn` and `.await`

```rust
async fn fetch_weather() -> String {
    // ...something slow...
    String::from("sunny")
}

async fn plan_day() -> String {
    let weather = fetch_weather().await;     // wait here until it's ready
    format!("It's {weather}, let's go out")
}
```

- **`async fn`** marks a function that can pause while waiting.
- **`.await`** means "wait for this to finish. Meanwhile, other async work can run."

## Futures: a promise of a result

Here's the surprising part: **calling an async function doesn't run it.** It gives back a **future**: a value that represents work which will produce a result later, like an order ticket. Nothing happens until something `.await`s it or a runtime runs it.

```rust
let ticket = fetch_weather();      // nothing has happened yet
let weather = ticket.await;        // now it runs
```

## Runtimes: who runs the futures

`main` can't be `async` on its own. Something has to keep checking futures, running whichever ones can make progress. That's a **runtime**, the restaurant's manager.

This lesson includes a tiny runtime in `src/runtime.rs`, written just for learning:

- **`block_on(future)`** runs a future until it's done. Use it in `main`.
- **`pause(steps)`** stands in for something slow, like a download. Each step is one moment of waiting.
- **`join(a, b)`** runs two futures at the same time.
- **`Log`** is a shared list that records what happened, in order. It's an `Rc<RefCell<Vec<String>>>` from Lesson 25.

**Real projects use tokio**, the most popular Rust runtime, by adding it with `cargo add tokio --features full`. Then `#[tokio::main]` lets `main` itself be async, and `tokio::join!` and `tokio::time::sleep` work just like this lesson's `join` and `pause`. Everything you practice here carries over directly.

## One after another, or at the same time

```rust
// One after another: the second download doesn't start until the first finishes
let a = download("photo", 3, &log).await;
let b = download("song", 1, &log).await;

// At the same time: both start right away, and we wait for both
let (a, b) = join(download("photo", 3, &log), download("song", 1, &log)).await;
```

The log makes the difference visible. One after another records `start photo, finish photo, start song, finish song`. At the same time records `start photo, start song, finish song, finish photo`. The short song finishes first, while the photo is still going.

## Your challenge

Open `src/main.rs`.

1. **`download`**: write the name to the log as `"start NAME"`, wait `steps` steps with `pause(steps).await`, write `"finish NAME"`, and give back `steps * 100`, standing in for the file's size.
2. **`one_after_another`**: download `"photo"` (3 steps), then `"song"` (1 step), one after the other. Give back the total size.
3. **`at_the_same_time`**: the same two downloads, run together with `join`. Give back the total size.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`. The checker looks at the log to confirm each version ran in the right order.

## Stuck?

- **Writing to the log:** `log.borrow_mut().push(format!("start {name}"));`.
- **`one_after_another`:** two `let` lines, each with `.await`, then add the sizes.
- **`at_the_same_time`:** `let (photo, song) = join(download("photo", 3, log), download("song", 1, log)).await;`.

## New words

- **Async**: code that can pause while waiting and let other work run.
- **Future**: a value standing for work that will produce a result later.
- **Await**: wait for a future to finish.
- **Runtime**: the code that runs futures and decides which one goes next.
