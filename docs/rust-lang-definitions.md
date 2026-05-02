# Rust Language Definitions

A running glossary of Rust terms and concepts.

---

## Structs

A **struct** (short for "structure") is how you define custom data types in Rust — a named
group of fields, each with a type.

```rust
struct Order {
    product_name: String,
    quantity: u32,
}
```

You can add behavior to a struct with an `impl` block:

```rust
impl Order {
    fn total_label(&self) -> String {
        format!("{} (x{})", self.product_name, self.quantity)
    }
}
```

Structs are similar to classes in other languages, but Rust has **no inheritance**. Shared
behavior is achieved through traits instead.

There are three kinds of structs:

| Kind | Example | Use case |
|---|---|---|
| Named fields | `struct Point { x: f64, y: f64 }` | Most common — named, readable fields |
| Tuple struct | `struct Color(u8, u8, u8)` | When field names aren't needed |
| Unit struct | `struct Marker;` | No data, used as a type-level marker |

---

## Traits

A **trait** defines shared behavior — a contract that types can implement. Similar to
interfaces in TypeScript/Java or protocols in Swift.

```rust
trait Printable {
    fn summary(&self) -> String;
}
```

Any type can implement a trait:

```rust
impl Printable for Order {
    fn summary(&self) -> String {
        format!("{}: {}", self.product_name, self.quantity)
    }
}
```

Key points:

- Traits define **what** something can do; structs define **what** something is.
- You must `use` a trait before calling its methods (e.g. `use colored::Colorize`).
- Traits can provide default implementations that types can override.
- Common standard library traits: `Display` (printable with `{}`), `Debug` (printable
  with `{:?}`), `Clone` (deep-copyable), `Iterator`.

Examples from the learning projects:

| Trait | Crate | What it means |
|---|---|---|
| `Deserialize` | serde | "This type can be created from JSON (or other formats)" |
| `Serialize` | serde | "This type can be written as JSON (or other formats)" |
| `Colorize` | colored | "This string can have `.red()`, `.bold()` called on it" |
| `Parser` | clap | "This struct defines a CLI argument parser" |
| `Subcommand` | clap | "This enum defines CLI subcommands" |

---

## Macros

A **macro** is code that generates other code at compile time. Macros can do things that
regular functions cannot, like accept a variable number of arguments or inspect a struct's
fields.

### Function-like macros

Called with `!` after the name. They look like function calls but are expanded by the
compiler before the code runs.

```rust
println!("Hello, {}!", name);   // formatted printing
format!("x = {}", 42);          // formatted string creation
vec![1, 2, 3];                  // create a Vec with initial values
eprintln!("error: {}", msg);    // print to stderr
```

The `!` is how you tell a macro apart from a regular function call.

### Derive macros

Used with the `#[derive(...)]` attribute on structs and enums. They inspect the type's
fields at compile time and auto-generate trait implementations.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    task: String,
    done: bool,
}
```

This single line generates all the code needed for `Todo` to be debug-printed, cloned,
serialized to JSON, and deserialized from JSON.

### How they connect

```
struct defines the data shape
    → derive macro inspects the fields at compile time
        → generates a trait implementation
            → your type now satisfies the trait
                → libraries (serde, clap, etc.) can work with it
```
