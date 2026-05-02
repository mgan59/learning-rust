# Project 01: Hello CLI

A minimal command-line tool that greets the user by name. This project introduces the
fundamentals of a Rust binary, the standard library, and how command-line arguments work.

---

## Table of Contents

1. [Project Structure](#project-structure)
2. [How to Run](#how-to-run)
3. [Code Walkthrough](#code-walkthrough)
4. [Concepts Covered](#concepts-covered)
5. [Exercises](#exercises)

---

## Project Structure

```
01-hello-cli/
├── Cargo.toml       # Package manifest (name, version, dependencies)
├── GUIDE.md         # This learning guide
└── src/
    └── main.rs      # Application entry point
```

Every Rust project managed by Cargo follows this layout. `Cargo.toml` is the manifest
that describes your package, and `src/main.rs` is the default entry point for a binary
(executable) crate.

---

## How to Run

```bash
# Build and run with no arguments
cargo run

# Pass an argument after `--` (the separator tells Cargo
# that everything after it is for YOUR program, not for Cargo)
cargo run -- Morgan

# Build a release (optimized) binary
cargo build --release

# Run the compiled binary directly
./target/debug/hello_cli
./target/debug/hello_cli Morgan
```

---

## Code Walkthrough

Here is the full source in `src/main.rs`:

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let name = &args[1];
        println!("Hello, {}!", name);
    } else {
        println!("Hello, world!");
    }
}
```

### Line-by-line breakdown

### `use std::env;`

`use` brings items into scope so you can refer to them by their short name. Here we
import the `env` module from the **standard library** (`std`). This module provides
functions for interacting with the environment — including reading command-line arguments.

### `fn main() { ... }`

Every Rust binary needs a `main` function. This is where execution begins. Unlike some
languages, `main` in Rust returns `()` (the unit type, essentially "nothing") by default,
but it can also return a `Result` for error handling — you'll see that in later projects.

### `let args: Vec<String> = env::args().collect();`

This line packs several concepts together:

| Piece | What it does |
|---|---|
| `let args` | Declares a new variable binding called `args`. Variables in Rust are **immutable by default**. |
| `: Vec<String>` | A **type annotation**. `Vec<String>` is a growable array (vector) of owned strings. The annotation is needed here so `.collect()` knows what type to produce. |
| `env::args()` | Returns an **iterator** over the command-line arguments. The first element (`args[0]`) is always the program's own name/path. |
| `.collect()` | Consumes the iterator and collects all items into a collection — in this case the `Vec<String>` we specified. |

### `if args.len() > 1 { ... } else { ... }`

Standard control flow. `args.len()` returns the number of elements. We check for `> 1`
(not `> 0`) because `args[0]` is always the program name itself.

Note: `if` in Rust is an **expression**, meaning it can return a value. You could write:

```rust
let greeting = if args.len() > 1 {
    format!("Hello, {}!", &args[1])
} else {
    String::from("Hello, world!")
};
println!("{}", greeting);
```

### `let name = &args[1];`

The `&` creates a **reference** (a borrow) to the string at index 1. Rust's ownership
system ensures memory safety without a garbage collector. Key rules:

- Each value has a single **owner**.
- You can have either **one mutable reference** OR **any number of immutable references**
  at a time.
- References must always be valid (no dangling pointers).

Here we only need to read the value, so an immutable borrow (`&`) is perfect.

### `println!("Hello, {}!", name);`

`println!` is a **macro** (indicated by the `!`). Macros in Rust are expanded at compile
time. The `{}` is a placeholder that formats the value using its `Display` trait — similar
to `{}` in Python f-strings or `%s` in C.

---

## Concepts Covered

| Concept | Summary |
|---|---|
| **Cargo** | Rust's build tool and package manager. `cargo new`, `cargo run`, `cargo build`. |
| **`Cargo.toml`** | The manifest file declaring your package name, version, Rust edition, and dependencies. |
| **`fn main()`** | The entry point for binary crates. |
| **`let` bindings** | How you declare variables. Immutable by default; use `let mut` for mutable. |
| **Type annotations** | Explicit types like `Vec<String>`. Often optional thanks to type inference, but required when the compiler can't infer (e.g. with `.collect()`). |
| **`Vec<T>`** | A growable, heap-allocated array. One of the most common collections in Rust. |
| **Iterators & `.collect()`** | Lazy sequences that you can transform and collect into concrete types. |
| **References & borrowing** | `&` borrows a value without taking ownership. Central to Rust's memory safety model. |
| **`println!` macro** | Formatted printing to stdout. Uses `{}` for `Display` formatting. |
| **`if`/`else` expressions** | Control flow that can also return values. No parentheses required around the condition. |

---

## Exercises

Try these modifications to deepen your understanding. Each builds on the last.

### 1. Greet multiple names

Modify the program so it greets every argument, not just the first:

```
$ cargo run -- Alice Bob Charlie
Hello, Alice!
Hello, Bob!
Hello, Charlie!
```

**Hint:** Use a `for` loop to iterate over `&args[1..]` (a **slice** of everything after
the program name).

### 2. Add `--shout` mode

If the user passes `--shout` as the first argument, print the greeting in uppercase:

```
$ cargo run -- --shout Morgan
HELLO, MORGAN!
```

**Hint:** Look at the `.to_uppercase()` method on `String`/`&str`.

### 3. Use `mut` and reassignment

Refactor so that you build the greeting string into a `let mut` variable before printing:

```rust
let mut greeting = String::from("Hello");
// conditionally modify `greeting` ...
println!("{}", greeting);
```

This will get you comfortable with **mutable bindings**.

### 4. Return a `Result` from `main`

Change `main` to return `Result<(), Box<dyn std::error::Error>>` and use `?` to propagate
errors. For example, what happens if you try to access `args[1]` on an empty args list
using `.get(1).ok_or("No name provided")?` instead of an `if` check?

This previews Rust's error handling model, which you'll use heavily in future projects.
