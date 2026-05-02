# Project 02: Order Viewer

A CLI tool that reads a JSONL file of product orders and displays them as a
color-formatted table. This project introduces file I/O, external crates,
JSON deserialization, structs, error handling with `match`, and iterators.

---

## Table of Contents

1. [Project Structure](#project-structure)
2. [How to Run](#how-to-run)
3. [The Data Format](#the-data-format)
4. [Code Walkthrough](#code-walkthrough)
5. [Concepts Covered](#concepts-covered)
6. [Exercises](#exercises)

---

## Project Structure

```
02-order-viewer/
├── Cargo.toml            # Manifest with external dependencies
├── GUIDE.md              # This learning guide
├── sample_orders.jsonl   # Sample data to test with
└── src/
    └── main.rs           # Application source
```

---

## How to Run

```bash
# Run with the included sample data
cargo run -- sample_orders.jsonl

# Run with no args to see the error message
cargo run

# Run with a file that doesn't exist to see file error handling
cargo run -- nope.jsonl
```

---

## The Data Format

JSONL (JSON Lines) is a format where each line is a standalone JSON object.
It's common for log files, data pipelines, and streaming data because you can
read and parse one line at a time without loading an entire array.

```json
{"productName": "Mechanical Keyboard", "quantity": 1, "dateOfPurchase": "2026-01-15"}
{"productName": "USB-C Hub", "quantity": 2, "dateOfPurchase": "2026-02-03"}
```

Each line has three fields:

| Field | Type | Description |
|---|---|---|
| `productName` | string | Name of the product ordered |
| `quantity` | number | How many were ordered |
| `dateOfPurchase` | string | Date in `YYYY-MM-DD` format |

---

## Code Walkthrough

### Cargo.toml — Adding Dependencies

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
colored = "3"
```

This is your first time using **external crates** (Rust's term for libraries/packages).
Crates are hosted on [crates.io](https://crates.io) and Cargo downloads them automatically.

| Crate | Purpose |
|---|---|
| `serde` | A serialization/deserialization framework. The `derive` feature lets you use `#[derive(Deserialize)]` on structs. |
| `serde_json` | A JSON parser that plugs into serde. |
| `colored` | Adds methods like `.red()`, `.bold()`, `.dimmed()` to strings for terminal color output. |

When you specify `version = "1"`, Cargo will use the latest `1.x.x` release
(following semver). The `features = ["derive"]` syntax enables optional functionality
within the crate.

---

### Imports

```rust
use std::env;
use std::fs;
use std::process;

use colored::Colorize;
use serde::Deserialize;
```

- **`std::fs`** — file system operations (new in this project)
- **`std::process`** — lets us call `process::exit(1)` to quit with an error code
- **`colored::Colorize`** — a **trait** that adds color methods to strings
- **`serde::Deserialize`** — a **derive macro** for automatic JSON-to-struct parsing

A **trait** in Rust is like an interface — it defines behavior that types can implement.
The `Colorize` trait adds methods like `.red()` and `.bold()` to `&str` and `String`.
You need to `use` a trait before you can call its methods.

---

### The `Order` Struct

```rust
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Order {
    product_name: String,
    quantity: u32,
    date_of_purchase: String,
}
```

**Structs** are Rust's way of grouping related data together (like a class without
methods, or a record/object in other languages).

| Piece | What it does |
|---|---|
| `#[derive(Deserialize)]` | An **attribute** that auto-generates code to parse JSON into this struct. Without it, you'd have to write the parsing logic by hand. |
| `#[serde(rename_all = "camelCase")]` | Tells serde that the JSON uses `camelCase` keys (`productName`) but the Rust fields use `snake_case` (`product_name`). Serde maps between them automatically. |
| `u32` | An unsigned 32-bit integer. Rust has explicit integer sizes: `i8`, `i16`, `i32`, `i64` (signed) and `u8`, `u16`, `u32`, `u64` (unsigned). |

---

### Reading the File

```rust
let contents = match fs::read_to_string(file_path) {
    Ok(text) => text,
    Err(err) => {
        eprintln!("{} Could not read '{}': {}", "Error:".red().bold(), file_path, err);
        process::exit(1);
    }
};
```

`fs::read_to_string()` returns a **`Result<String, io::Error>`**. This is Rust's primary
error handling mechanism — there are no exceptions. A `Result` is an enum with two variants:

```
Result<T, E>
├── Ok(T)    — success, contains the value
└── Err(E)   — failure, contains the error
```

The `match` expression handles both cases explicitly. The compiler **forces** you to handle
the error — you can't accidentally ignore it. This is one of Rust's biggest strengths.

`eprintln!` prints to **stderr** (not stdout), which is the convention for error messages.

---

### Parsing JSONL Line by Line

```rust
let mut orders: Vec<Order> = Vec::new();

for (line_num, line) in contents.lines().enumerate() {
    if line.trim().is_empty() {
        continue;
    }

    match serde_json::from_str::<Order>(line) {
        Ok(order) => orders.push(order),
        Err(err) => {
            eprintln!("Warning: Failed to parse line {}: {}", line_num + 1, err);
        }
    }
}
```

Key concepts here:

- **`let mut`** — declares a mutable variable. We need mutability because we `.push()`
  into the vector.
- **`.lines()`** — returns an iterator over each line of a string (splitting on `\n`).
- **`.enumerate()`** — wraps an iterator to yield `(index, value)` tuples — similar to
  Python's `enumerate()`.
- **`continue`** — skips the rest of the current loop iteration.
- **Turbofish syntax `::<Order>`** — explicitly tells `from_str` what type to deserialize
  into. Called "turbofish" because `::<>` looks like a fish.

Notice how we handle parse errors gracefully: bad lines print a warning but don't crash
the program. The remaining lines still get processed.

---

### Formatted Output with Colors

```rust
println!(
    "  {:<30} {:>8} {}",
    order.product_name.green(),
    qty_display,
    order.date_of_purchase.dimmed()
);
```

Rust's format strings support alignment and width:

| Syntax | Meaning |
|---|---|
| `{:<30}` | Left-align, padded to 30 characters wide |
| `{:>8}` | Right-align, padded to 8 characters wide |
| `{}` | Default formatting, no padding |

The `colored` crate methods (`.green()`, `.bold()`, `.dimmed()`, `.cyan()`) return a
`ColoredString` that wraps the text in ANSI escape codes. Your terminal interprets
these codes to render colors.

---

### Iterator Methods for the Summary

```rust
let total_items: u32 = orders.iter().map(|o| o.quantity).sum();
```

This is a chain of **iterator adapters**:

1. `.iter()` — creates an iterator over references to each `Order`
2. `.map(|o| o.quantity)` — transforms each order into just its quantity. The `|o|` is a
   **closure** (anonymous function). Closures in Rust use pipes `||` instead of
   parentheses for parameters.
3. `.sum()` — consumes the iterator and adds up all the values

This functional style is idiomatic Rust. The compiler optimizes iterator chains into
code as fast as a hand-written loop — there's no performance penalty.

---

## Concepts Covered

| Concept | Summary |
|---|---|
| **External crates** | Adding dependencies in `Cargo.toml`. Cargo fetches them from crates.io. |
| **Structs** | Custom data types that group fields together. |
| **`#[derive(...)]`** | Attributes that auto-generate trait implementations (like `Deserialize`). |
| **`Result<T, E>`** | Rust's error handling type. Forces you to handle success and failure explicitly. |
| **`match`** | Pattern matching — Rust's powerful equivalent to switch/case, but much more expressive. |
| **`fs::read_to_string`** | Reading an entire file into a `String`. |
| **`let mut`** | Mutable variable bindings. Required when you need to modify a value after creation. |
| **`.lines()` / `.enumerate()`** | Iterator methods for processing text line by line with indices. |
| **Turbofish `::<T>`** | Explicit type parameters on function calls. |
| **Closures `\|x\| ...`** | Anonymous functions, used heavily with iterator adapters. |
| **`.iter().map().sum()`** | Functional-style iterator chains. Zero-cost abstractions in Rust. |
| **Format string alignment** | `{:<30}` and `{:>8}` for padded, aligned output. |
| **Traits** | Shared behavior (like `Colorize`) that you `use` to call methods on types. |
| **ANSI terminal colors** | The `colored` crate adds color methods to strings. |
| **`eprintln!`** | Print to stderr for error/warning messages. |
| **`process::exit(1)`** | Exit the program with a non-zero status code to indicate failure. |

---

## Exercises

### 1. Sort by date

Print the orders sorted by `date_of_purchase` (oldest first).

**Hint:** Use `orders.sort_by(|a, b| a.date_of_purchase.cmp(&b.date_of_purchase))`.
Since dates are in `YYYY-MM-DD` format, alphabetical sorting works correctly.

### 2. Filter by product name

Add a second optional argument `--filter <term>` that only shows orders whose product
name contains the given text (case-insensitive).

```bash
cargo run -- sample_orders.jsonl --filter mouse
```

**Hint:** Look at `.to_lowercase()` and `.contains()` on strings.

### 3. Add a total cost column

Add a `pricePerUnit` field (as `f64`) to the JSON and struct. Display a "Total" column
(`quantity * price_per_unit`) and a grand total in the summary.

**Hint:** Use `{:>10.2}` in the format string to right-align with 2 decimal places.

### 4. Error handling with `?`

Refactor `main` to return `Result<(), Box<dyn std::error::Error>>` and replace the
`match` + `process::exit` calls with the `?` operator. Compare how the code reads
before and after.

### 5. Read from stdin

If no file path is given, read from **stdin** instead, so the tool works with pipes:

```bash
cat sample_orders.jsonl | cargo run
```

**Hint:** Use `std::io::{self, Read}` and `io::stdin().read_to_string(&mut buffer)`.
