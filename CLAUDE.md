# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Purpose

This is a personal **learning environment** for Rust, not a production codebase. Each top-level numbered directory (`01-hello-cli`, `02-order-viewer`, ...) is an independent Cargo project that introduces new Rust concepts incrementally. The current focus is CLI tools because they need minimal framework scaffolding.

Two implications for how to work here:

- **Don't refactor across projects.** Each numbered directory is intentionally self-contained and frozen at the level of concepts it was meant to teach. Earlier projects deliberately use simpler patterns (e.g. `process::exit` instead of `?`) — preserve that, even when a "better" Rust idiom exists.
- **The `GUIDE.md` in each project is the source of truth for what concepts that project teaches.** When adding a new project or modifying an exercise, keep `GUIDE.md` and the code in sync. The `## Exercises` section at the bottom of each guide lists intentional follow-up tasks the user may work through.

## Repo Layout

```
NN-<project-name>/
├── Cargo.toml      # Per-project manifest — each project is its own crate
├── GUIDE.md        # Walkthrough + concepts covered + exercises
├── src/main.rs     # Binary entry point
└── (sample data, e.g. sample_orders.jsonl)
```

There is **no workspace `Cargo.toml`** at the repo root. Each project is built/run from its own directory.

## Common Commands

Run from inside the specific project directory:

```bash
cargo run                       # build + run with no args
cargo run -- <args>             # everything after `--` is forwarded to the binary
cargo build                     # debug build → target/debug/<crate-name>
cargo build --release           # optimized build → target/release/<crate-name>
cargo check                     # fast type-check without producing a binary
```

Project-specific examples:

```bash
# 01-hello-cli
cargo run -- Morgan

# 02-order-viewer
cargo run -- sample_orders.jsonl
```

## Adding a New Project

1. `cargo new NN-<name>` at the repo root (matches existing numbering).
2. Add a `GUIDE.md` following the structure of existing guides: Project Structure → How to Run → Code Walkthrough (line-by-line for new concepts) → Concepts Covered table → Exercises.
3. Only introduce concepts that build on previous projects — the progression is the point.
