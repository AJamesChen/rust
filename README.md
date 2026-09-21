# Rust Practice

This repository contains Rust practice code organized as a Cargo workspace.
It includes host-side programming exercises and embedded Rust examples.

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
├── embedded-rust/
│   ├── ch2-app/            # no_std Cortex-M application examples
│   └── ch2-app2/           # STM32F3DISCOVERY quickstart application
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

### `ch2-app`

A `no_std` Cortex-M application based on the Knurling embedded app template.
It contains small examples for `defmt` logging, formatting, panic handling,
bitfields, and stack-overflow behaviour.

### `ch2-app2`

A second Cortex-M quickstart application configured for the STM32F3DISCOVERY
board and its STM32F303VC microcontroller.

## Requirements

- Rust toolchain with Cargo, rustfmt, and Clippy
- The `thumbv7em-none-eabihf` Cortex-M compilation target
- Edition: Rust 2024

Install Rust with [rustup](https://rustup.rs/) if it is not already available.

## Common Commands

Run the host-side tests:

```sh
cargo test -p codility
```

Check the embedded examples without requiring target hardware:

```sh
cargo check -p ch2-app -p ch2-app2 --target thumbv7em-none-eabihf
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

GitHub Actions runs the `Rust CI` workflow on pushes and pull requests. It tests
the host crate normally and checks the embedded crate for a Cortex-M target
without trying to execute target tests on the CI host.

```sh
cargo fmt --all -- --check
cargo clippy -p codility --all-targets --all-features -- -D warnings
cargo clippy -p ch2-app -p ch2-app2 --target thumbv7em-none-eabihf --lib --bins --all-features -- -D warnings
cargo check -p ch2-app -p ch2-app2 --target thumbv7em-none-eabihf
cargo test -p codility --verbose
```

## Adding Exercises

Add new Codility exercises under `codility/src`, grouped by lesson. Expose each
new module from its parent `mod.rs`, and include focused unit tests next to the
solution.
