# rustmorphism

[![Crates.io](https://img.shields.io/crates/v/rustmorphism.svg)](https://crates.io/crates/rustmorphism)
[![Documentation](https://docs.rs/rustmorphism/badge.svg)](https://docs.rs/rustmorphism)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/yourusername/rustmorphism/actions/workflows/ci.yml/badge.svg)](https://github.com/yourusername/rustmorphism/actions)

> **Compile-time polymorphism for Rust functions: deterministic, zero-cost, and ergonomic.**

`rustmorphism` is a procedural macro crate that lets you define multiple implementations for a function, with one being deterministically selected at compile time. This enables A/B testing, feature comparison, and binary size optimization—all with zero runtime overhead.

---

## Table of Contents

- [Features](#features)
- [Motivation](#motivation)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage](#usage)
  - [Basic Example](#basic-example)
  - [Binary Size Optimization](#binary-size-optimization)
  - [Implementation A/B Testing](#implementation-ab-testing)
- [How It Works](#how-it-works)
- [Controlling Selection](#controlling-selection)
- [FAQ](#faq)
- [Contributing](#contributing)
- [License](#license)

---

## Features

- **Multiple implementations:** Define several function bodies in a single macro call.
- **Deterministic selection:** One implementation is chosen at compile time, based on build metadata.
- **Zero runtime overhead:** Unused implementations are not included in the final binary.
- **Great for A/B testing:** Try out different algorithms or optimizations with no runtime cost.
- **Compile-time binary size optimization:** Select smaller or larger implementations as needed.

---

## Motivation

Rust does not natively support function-level compile-time polymorphism. `rustmorphism` fills this gap, making it easy to:

- Experiment with different algorithms or optimizations.
- Reduce binary size by excluding unused code.
- Perform deterministic A/B testing across builds.

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rustmorphism = "0.1.0"
```

---

## Quick Start

```rust
use rustmorphism::polymorphic_fn;

polymorphic_fn! {
    pub fn choose(x: i32) -> i32 {
        { x + 1 },         // Implementation 1
        { x * 2 },         // Implementation 2
        { x * x }          // Implementation 3
    }
}

fn main() {
    println!("Result: {}", choose(5));
}
```

---

## Usage

### Basic Example

```rust
use rustmorphism::polymorphic_fn;

polymorphic_fn! {
    pub fn calculate(x: i32) -> i32 {
        { x + 1 },         // Implementation 1
        { x * 2 },         // Implementation 2
        { x * x }          // Implementation 3
    }
}

fn main() {
    let result = calculate(5);
    println!("Result: {}", result); // Will print result from one of the implementations
}
```

### Binary Size Optimization

```rust
use rustmorphism::polymorphic_fn;

fn small_impl(data: &[u8]) -> u64 {
    data.iter().map(|&b| b as u64).sum()
}

fn large_impl(data: &[u8]) -> u64 {
    static LARGE_DATA: [u8; 1_000_000] = [0; 1_000_000];
    data.iter().zip(LARGE_DATA.iter())
        .map(|(&a, &b)| (a as u64) * (b as u64 + 1))
        .sum()
}

polymorphic_fn! {
    pub fn process_data(data: &[u8]) -> u64 {
        { small_impl(data) },
        { large_impl(data) }
    }
}
```

### Implementation A/B Testing

```rust
use rustmorphism::polymorphic_fn;

polymorphic_fn! {
    pub fn sort_algorithm<T: Ord + Copy>(data: &mut [T]) {
        { 
            // Implementation 1: Quick sort
            data.sort_unstable();
        },
        { 
            // Implementation 2: Insertion sort
            for i in 1..data.len() {
                let mut j = i;
                while j > 0 && data[j-1] > data[j] {
                    data.swap(j-1, j);
                    j -= 1;
                }
            }
        }
    }
}
```

---

## How It Works

The `polymorphic_fn!` macro uses compile-time hashing to select one implementation from the provided alternatives. The selection is based on:

- Function name
- Build timestamp
- Package name and version
- Target architecture and profile
- (Optional) Build counter

This ensures the selection is consistent for a given build, but can change between builds, allowing for different implementations to be tested over time.

---

## Controlling Selection

You can influence which implementation is selected by:

- Forcing a rebuild (e.g., `cargo clean`).
- Using a build script with `cargo:rerun-if-changed=nonexistent-file` to ensure the build script runs every time.
- Setting the `FORCE_REBUILD` environment variable before building.
- Incrementing a build counter in your build script.

---

## FAQ

**Q: Is there any runtime overhead?**  
A: No. Only the selected implementation is compiled into the binary.

**Q: Can I use this for benchmarking or fuzzing?**  
A: Yes! This is a great way to compare different algorithms or code paths.

**Q: How do I ensure a specific implementation is chosen?**  
A: The selection is deterministic but based on build metadata. For full control, you can patch the macro or use environment variables as entropy.

---

## Contributing

Contributions, issues, and feature requests are welcome!  
Feel free to check the [issues page](https://github.com/yourusername/rustmorphism/issues) or submit a pull request.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Links

- [API Documentation](https://docs.rs/rustmorphism)
- [Crates.io](https://crates.io/crates/rustmorphism)
- [Issue Tracker](https://github.com/yourusername/rustmorphism/issues) 