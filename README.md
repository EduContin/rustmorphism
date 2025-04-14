# polymorphic_fn

[![Crates.io](https://img.shields.io/crates/v/polymorphic_fn.svg)](https://crates.io/crates/polymorphic_fn)
[![Documentation](https://docs.rs/polymorphic_fn/badge.svg)](https://docs.rs/polymorphic_fn)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A Rust procedural macro that enables functions with multiple implementations, where one is deterministically selected at compile-time.

## Features

- Define multiple function implementations in a single macro call
- One implementation is deterministically selected at compile-time
- Unused implementations are entirely removed from the binary
- Selection is based on function name, build timestamp, and other factors
- Useful for A/B testing, feature comparison, and binary size optimization

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
polymorphic_fn = "0.1.0"
```

## Usage

### Basic Example

```rust
use polymorphic_fn::polymorphic_fn;

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

### Use Cases

#### Binary Size Optimization

```rust
use polymorphic_fn::polymorphic_fn;

// Define alternative implementations with different size characteristics
fn small_implementation(data: &[u8]) -> u64 {
    // Simple, small implementation
    data.iter().map(|&b| b as u64).sum()
}

fn large_implementation(data: &[u8]) -> u64 {
    // Complex implementation with more features but larger code size
    // (e.g., includes optimizations, debug code, etc.)
    static LARGE_DATA: [u8; 1_000_000] = [0; 1_000_000];
    data.iter().zip(LARGE_DATA.iter())
        .map(|(&a, &b)| (a as u64) * (b as u64 + 1))
        .sum()
}

polymorphic_fn! {
    pub fn process_data(data: &[u8]) -> u64 {
        { small_implementation(data) },
        { large_implementation(data) }
    }
}
```

#### Implementation A/B Testing

```rust
use polymorphic_fn::polymorphic_fn;

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

## How It Works

The `polymorphic_fn` macro uses compile-time hashing to deterministically select one implementation from the provided alternatives. The selection is based on:

1. The function name
2. Build timestamp
3. Package name and version
4. Target architecture and profile
5. A build counter that increments on each build

This ensures that the selection is consistent for a given build but can change between builds, allowing for different implementations to be tested over time.

## Controlling Selection

To force a rebuild and potentially get a different implementation selected, you can:

1. Use a build script with `cargo:rerun-if-changed=nonexistent-file` to ensure the build script runs every time
2. Set the `FORCE_REBUILD` environment variable before building
3. Increment a build counter in your build script

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 