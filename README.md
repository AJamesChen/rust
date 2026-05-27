# Rust Practice

This repository contains Rust practice code organized as a Cargo workspace.
The current workspace member is `codility`, a library crate for Codility-style
programming exercises.

## Repository Layout

```text
.
├── Cargo.toml              # Workspace manifest
├── codility/
│   ├── Cargo.toml          # Codility practice crate
│   └── src/
│       ├── lib.rs
│       ├── lesson01_iterations/
│       │   ├── mod.rs
│       │   └── binary_gap.rs
│       └── lesson02_arrays/
│           ├── mod.rs
│           ├── cyclic_rotation.rs
│           └── odd_occurrences_in_array.rs
└── README.md
```

## Crates

### `codility`

Solutions for Codility lessons, grouped by lesson module.

Implemented exercises:

- Lesson 01: Iterations
  - Binary Gap
- Lesson 02: Arrays
  - Cyclic Rotation
  - Odd Occurrences In Array

## Requirements

- Rust toolchain with Cargo
- Edition: Rust 2024

Install Rust with [rustup](https://rustup.rs/) if it is not already available.

## Common Commands

Run the full test suite:

```sh
cargo test
```

Check that the workspace builds:

```sh
cargo check
```

Format the code:

```sh
cargo fmt
```

Run Clippy lints:

```sh
cargo clippy --all-targets --all-features
```

## CI/CD

GitHub Actions runs the `Rust CI` workflow on pushes and pull requests. The
workflow installs the stable Rust toolchain with `rustfmt` and `clippy`, caches
Cargo dependencies, then runs:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo build --workspace --verbose
cargo test --workspace --verbose
```

## Adding Exercises

Add new Codility exercises under `codility/src`, grouped by lesson. Expose each
new module from its parent `mod.rs`, and include focused unit tests next to the
solution.
