# Lesson 31: Workspaces: projects with many crates

**What you'll learn**
- How big Rust projects are split into several crates
- How a **workspace** ties them together
- How crates in a workspace depend on each other
- Running and testing one crate, or all of them

## The idea: one building, many apartments

As projects grow, one crate isn't enough. A game might have the game rules, a graphics part, and a server. Each is its own crate, with its own `Cargo.toml`, so they're easier to understand, test, and reuse.

A **workspace** groups those crates into one project. Think of an apartment building: each apartment (crate) has its own space, but they share one address, one mailroom, and one set of utilities.

In a workspace, the crates share:

- **One `target/` folder**, so shared code is only built once.
- **One `Cargo.lock`**, so every crate uses the same versions of everything.

## This lesson's layout

```
31-workspaces/
├── Cargo.toml            ← the workspace: lists its members. It has no code of its own.
├── app/                  ← a binary crate: the program people run
│   ├── Cargo.toml
│   └── src/main.rs
└── scorekeeper/          ← a library crate: the scoring rules
    ├── Cargo.toml
    └── src/lib.rs
```

## The workspace `Cargo.toml`

```toml
[workspace]
members = ["app", "scorekeeper"]
resolver = "2"
```

- `[workspace]` replaces `[package]`. This file describes the building, not an apartment.
- `members` lists the crates, by folder.
- `resolver = "2"` picks the modern rules for choosing versions. Always include it.

## Crates using each other

Being in the same workspace doesn't make crates able to use each other. Like Lesson 18, you list the dependency, with a path relative to the crate's **own** folder:

```toml
# app/Cargo.toml
[dependencies]
scorekeeper = { path = "../scorekeeper" }
```

`..` means "up one folder," so from `app/`, `../scorekeeper` is the sibling folder.

## Commands in a workspace

Run these from the top folder:

| Command | What it does |
| --- | --- |
| `cargo build` | Builds every crate |
| `cargo test` | Tests every crate |
| `cargo run -p app` | Runs the `app` crate. `-p` means *package*. |
| `cargo test -p scorekeeper` | Tests only `scorekeeper` |

You can also `cd` into a crate's folder and run commands there. Cargo still finds the workspace above it.

## Why split things up this way?

- **Clear boundaries.** `scorekeeper` can't reach into `app`, so the scoring rules stay independent of how they're shown.
- **Reuse.** A web version or a phone version could use `scorekeeper` too.
- **Faster builds.** Changing `app` doesn't rebuild `scorekeeper`.

Many big Rust projects work this way, including the Rust compiler itself.

## Your challenge

1. Run `cargo run -p app`. It fails, because `app` doesn't know about `scorekeeper`.
2. Add `scorekeeper` to the workspace's `members`, in the top `Cargo.toml`.
3. Add `scorekeeper` as a dependency in `app/Cargo.toml`.
4. In `scorekeeper/src/lib.rs`, finish the `Scoreboard` methods:
   - **`add_points`**: add points to a player, starting at 0 for new players.
   - **`score_of`**: a player's score, or 0 if they haven't played.
   - **`ranking`**: every player and score, highest first. Break ties alphabetically.
   - **`leader`**: the top player and score, or `None` if nobody has played.
5. Run `cargo run -p app`, then look in `target/`. Both crates were built there.

## Check your work

Press **Ctrl+Shift+B**, or run `dev learn check`.

## Stuck?

- **`ranking`:** collect `self.scores` into a `Vec<(String, u32)>` with `.iter().map(|(name, score)| (name.clone(), *score)).collect()`. Then sort it with `.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)))`, which means "higher score first, then name A to Z."
- **`leader`:** `self.ranking().into_iter().next()`.

## New words

- **Workspace**: a group of crates that share one `target/` folder and one `Cargo.lock`.
- **Member**: a crate that belongs to a workspace.
- **`-p`**: tells Cargo which package, meaning which crate, to work on.
