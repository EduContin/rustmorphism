# Polymorphic Framework for Rust

This Rust project provides a powerful framework for creating **truly polymorphic code** that changes its signature and implementation with each build. Unlike simple runtime function selection, this framework generates different code at build time, resulting in unique binaries with different signatures.

## What is Code Polymorphism?

In the context of this framework, polymorphism refers to the ability of code to take on multiple forms while maintaining the same functionality. This is similar to techniques used in polymorphic malware to evade signature detection, but applied for legitimate software development purposes such as:

1. Making software harder to reverse engineer
2. Creating applications resistant to signature-based scanning
3. Demonstrating advanced code generation techniques
4. Educational purposes around code morphism and obfuscation

## Framework Features

The polymorphic framework provides:

1. **Build-time Code Generation**: Generates different Rust code for each build using the `build.rs` script.

2. **Randomized Implementation Selection**: Randomly selects from different algorithm implementations with:
   - Random variable names
   - Random function names
   - Randomized code structure
   - Different algorithm choices

3. **Obfuscation Utilities**: Built-in utilities for:
   - Adding junk code
   - Randomizing whitespace
   - Adding bogus comments
   - Altering code structure

4. **Runtime Polymorphism**: For when you want to select implementations at runtime instead of build time.

5. **Generic Trait-Based Design**: Allows for creation of polymorphic versions of any function type.

## Usage Examples

### Creating Polymorphic Functions

```rust
use polymorphic_framework::{PolymorphicFunction, runtime_polymorphic};

// Create implementations for a string length function
struct DirectLength;

impl PolymorphicFunction<&str, usize> for DirectLength {
    fn execute(&self, input: &str) -> usize {
        input.len()
    }
    
    fn name(&self) -> &'static str {
        "DirectLength"
    }
}

// Alternative implementation
struct CharCount;

impl PolymorphicFunction<&str, usize> for CharCount {
    fn execute(&self, input: &str) -> usize {
        input.chars().count()
    }
    
    fn name(&self) -> &'static str {
        "CharCount"
    }
}

// Use runtime polymorphism
fn main() {
    let implementations = vec![
        Box::new(DirectLength),
        Box::new(CharCount),
    ];
    
    let input = "Hello, world!";
    let (result, algorithm) = runtime_polymorphic(input, &implementations);
    println!("Length: {} (using {})", result, algorithm);
}
```

### Using Build-Time Polymorphism

For build-time polymorphism, follow the factorial example in the `/examples` directory. This demonstrates how to:

1. Define multiple implementations of a function
2. Have the build script randomly select one implementation
3. Generate completely different code with each build

## Understanding the Framework

The framework consists of:

- **PolymorphicFunction trait**: The core trait that all polymorphic implementations must implement
- **PolymorphicFactory**: For creating polymorphic implementations selected at build time
- **runtime_polymorphic function**: For selecting implementations at runtime
- **Obfuscation utilities**: Helper functions for code transformation

## Building and Running

To build and run the demo:

```bash
cargo build --release
cargo run --release
```

Each time you build, a new binary with a completely different implementation will be generated.

## Creating Your Own Polymorphic Framework

To create your own polymorphic functions:

1. Define several different implementations of the same function
2. Create structs that implement the `PolymorphicFunction<Input, Output>` trait
3. Use the framework's utilities to randomize and obfuscate the code
4. Use either build-time or runtime polymorphism as needed

## Technical Details

- **build.rs**: The core of the build-time polymorphism, generates unique code
- **lib.rs**: Contains the framework's public API and traits
- **No precompiled implementations**: Everything is generated at build time

## Use Cases and Applications

This framework is particularly useful for:

1. Creating software that needs to evade signature detection
2. Demonstrating polymorphic code techniques
3. Educational purposes around code generation and obfuscation
4. Creating applications that are more difficult to reverse engineer

## License

MIT

# Polymorphic Framework Testing Tools

This directory contains tools to test and visualize the effectiveness of the polymorphic framework.

## Available Scripts

### `test_polymorphism.ps1`

This script builds the project multiple times and verifies that each build produces a unique binary with different file hashes. It helps validate that the polymorphic framework is truly generating different implementations for each build.

**Usage:**
```powershell
./test_polymorphism.ps1
```

**Features:**
- Builds the project multiple times in release mode
- Calculates and compares SHA256 hashes of each build
- Measures binary size differences
- Verifies that the binaries still function correctly
- Generates a detailed report

### `analyze_binaries.ps1`

This script performs deep analysis on the generated binaries to evaluate their structural differences, providing metrics on how effectively the polymorphism is working.

**Usage:**
```powershell
./analyze_binaries.ps1
```

**Features:**
- Calculates entropy and randomness measures
- Identifies code section differences
- Computes byte distribution patterns
- Provides a polymorphism effectiveness rating

### `visualize_polymorphism.ps1`

This script creates visual representations of the differences between polymorphic builds, helping to demonstrate the effectiveness of the polymorphism techniques.

**Usage:**
```powershell
./visualize_polymorphism.ps1
```

**Features:**
- Creates side-by-side binary difference visualizations
- Generates byte frequency distribution charts
- Produces an HTML dashboard with all visualizations
- Provides statistical insights on binary differences

## Interpreting Results

### Hashes
Different SHA256 hashes indicate that the binaries are not identical. Every build should have a unique hash if polymorphism is working correctly.

### Binary Differences
The visualization tools highlight differences between builds. Look for:
- Widespread differences throughout the binary (not just in small sections)
- Different byte frequency distributions
- High difference percentages (>10% is good, >25% is excellent)

### Functionality
Despite the differences in the binaries, all builds should maintain the same functionality. The test scripts verify this by running each binary and checking the output.

## Requirements

- PowerShell 5.1 or higher
- Windows environment
- Rust and Cargo installed

## Examples

To perform a complete test and visualization:

```powershell
# Build and test multiple polymorphic versions
./test_polymorphism.ps1

# Analyze the binary structure
./analyze_binaries.ps1

# Create visualizations of the differences
./visualize_polymorphism.ps1

# View the results in your browser
start polymorphism_test/visualizations/index.html
``` 