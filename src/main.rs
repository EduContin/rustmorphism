use polymorphic_framework::PolymorphicFunction;
use rand::Rng;

// Uncomment this when we add the 'examples' feature to Cargo.toml
// use polymorphic_framework::examples;

fn main() {
    println!("Demonstrating the Polymorphic Framework");

    // We'll use our custom example directly since the examples module is feature-gated
    let factorial_impls = custom_example::get_factorial_implementations();
    // Demo 1: Build-time polymorphism with the factorial example
    // Select factorial implementation based on build-time feature flags
    let factorial = {
        #[cfg(feature = "recursive")]
        {
            println!("Using recursive factorial (selected at build time via feature flag)");
            &custom_example::get_factorial_implementations()[0]  // RecursiveFactorial
        }
        #[cfg(feature = "iterative")]
        {
            println!("Using iterative factorial (selected at build time via feature flag)");
            &custom_example::get_factorial_implementations()[1]  // IterativeFactorial
        }
        #[cfg(feature = "functional")]
        {
            println!("Using functional factorial (selected at build time via feature flag)");
            &custom_example::get_factorial_implementations()[2]  // FunctionalFactorial
        }
        #[cfg(not(any(feature = "recursive", feature = "iterative", feature = "functional")))]
        {
            println!("No specific feature selected, using default factorial implementation");
            &custom_example::get_factorial_implementations()[0]  // Default to first implementation
        }
    };


    // We'll use our custom example directly since the examples module is feature-gated
    let factorial_impls = custom_example::get_factorial_implementations();
    // Demo 1: Build-time polymorphism with the factorial example
    // Select factorial implementation based on build-time feature flags
    let factorial = {
        #[cfg(feature = "recursive")]
        {
            println!("Using recursive factorial (selected at build time via feature flag)");
            &custom_example::get_factorial_implementations()[0]  // RecursiveFactorial
        }
        #[cfg(feature = "iterative")]
        {
            println!("Using iterative factorial (selected at build time via feature flag)");
            &custom_example::get_factorial_implementations()[1]  // IterativeFactorial
        }
        #[cfg(feature = "functional")]
        {
            println!("Using functional factorial (selected at build time via feature flag)");
            &custom_example::get_factorial_implementations()[2]  // FunctionalFactorial
        }
        #[cfg(not(any(feature = "recursive", feature = "iterative", feature = "functional")))]
        {
            println!("No specific feature selected, using default factorial implementation");
            &custom_example::get_factorial_implementations()[0]  // Default to first implementation
        }
    };


    // We'll use our custom example directly since the examples module is feature-gated
    let factorial_impls = custom_example::get_factorial_implementations();
    // Demo 1: Build-time polymorphism with the factorial example
    // Select factorial implementation based on build-time feature flags
    let factorial = {
        #[cfg(feature = "recursive")]
        {
            println!("Using recursive factorial (selected at build time via feature flag)");
            &custom_example::get_factorial_implementations()[0]  // RecursiveFactorial
        }
        #[cfg(feature = "iterative")]
        {
            println!("Using iterative factorial (selected at build time via feature flag)");
            &custom_example::get_factorial_implementations()[1]  // IterativeFactorial
        }
        #[cfg(feature = "functional")]
        {
            println!("Using functional factorial (selected at build time via feature flag)");
            &custom_example::get_factorial_implementations()[2]  // FunctionalFactorial
        }
        #[cfg(not(any(feature = "recursive", feature = "iterative", feature = "functional")))]
        {
            println!("No specific feature selected, using default factorial implementation");
            &custom_example::get_factorial_implementations()[0]  // Default to first implementation
        }
    };


    // We'll use our custom example directly since the examples module is feature-gated
    let factorial_impls = custom_example::get_factorial_implementations();
    // Demo 1: Build-time polymorphism with the factorial example
    // Select factorial implementation based on build-time feature flags
    let factorial = {
        #[cfg(feature = "recursive")]
        {
            println!("Using recursive factorial (selected at build time via feature flag)");
            &custom_example::get_factorial_implementations()[0]  // RecursiveFactorial
        }
        #[cfg(feature = "iterative")]
        {
            println!("Using iterative factorial (selected at build time via feature flag)");
            &custom_example::get_factorial_implementations()[1]  // IterativeFactorial
        }
        #[cfg(feature = "functional")]
        {
            println!("Using functional factorial (selected at build time via feature flag)");
            &custom_example::get_factorial_implementations()[2]  // FunctionalFactorial
        }
        #[cfg(not(any(feature = "recursive", feature = "iterative", feature = "functional")))]
        {
            println!("No specific feature selected, using default factorial implementation");
            &custom_example::get_factorial_implementations()[0]  // Default to first implementation
        }
    };


    // We'll use our custom example directly since the examples module is feature-gated
    let factorial_impls = custom_example::get_factorial_implementations();
    // Demo 1: Build-time polymorphism with the factorial example
    // Select factorial implementation based on build-time feature flags
    let factorial = {
        #[cfg(feature = "recursive")]
        {
            println!("Using recursive factorial (selected at build time via feature flag)");
            &custom_example::get_factorial_implementations()[0]  // RecursiveFactorial
        }
        #[cfg(feature = "iterative")]
        {
            println!("Using iterative factorial (selected at build time via feature flag)");
            &custom_example::get_factorial_implementations()[1]  // IterativeFactorial
        }
        #[cfg(feature = "functional")]
        {
            println!("Using functional factorial (selected at build time via feature flag)");
            &custom_example::get_factorial_implementations()[2]  // FunctionalFactorial
        }
        #[cfg(not(any(feature = "recursive", feature = "iterative", feature = "functional")))]
        {
            println!("No specific feature selected, using default factorial implementation");
            &custom_example::get_factorial_implementations()[0]  // Default to first implementation
        }
    };

    println!("======================================");
    
    // Create a factorial function chosen at build time
    let n = 10;
    
    // We'll use our custom example directly since the examples module is feature-gated
    let factorial_impls = custom_example::get_factorial_implementations();
    // Demo 1: Build-time polymorphism with the factorial example
    // Select factorial implementation based on build-time feature flags
    let factorial = {
        #[cfg(feature = "recursive")]
        {
            println!("Using recursive factorial (selected at build time via feature flag)");
            &custom_example::get_factorial_implementations()[0]  // RecursiveFactorial
        }
        #[cfg(feature = "iterative")]
        {
            println!("Using iterative factorial (selected at build time via feature flag)");
            &custom_example::get_factorial_implementations()[1]  // IterativeFactorial
        }
        #[cfg(feature = "functional")]
        {
            println!("Using functional factorial (selected at build time via feature flag)");
            &custom_example::get_factorial_implementations()[2]  // FunctionalFactorial
        }
        #[cfg(not(any(feature = "recursive", feature = "iterative", feature = "functional")))]
        {
            println!("No specific feature selected, using default factorial implementation");
            &custom_example::get_factorial_implementations()[0]  // Default to first implementation
        }
    };

    let result = factorial.execute(n);
    println!("Factorial of {} = {} (using {} algorithm)", n, result, factorial.name());
    
    // Demo 2: Runtime polymorphism with all available implementations
    println!("\nDemo 2: Runtime polymorphism with factorial");
    
    // We'll run this multiple times to demonstrate different implementations being chosen
    for i in 1..=5 {
        // We need to do this manually since we can't use &impl Trait with Boxes
        let idx = rand::thread_rng().gen_range(0..factorial_impls.len());
        let implementation = &factorial_impls[idx];
        let result = implementation.execute(n);
        let algorithm = implementation.name();
        println!("Run {}: Factorial of {} = {} (using {} algorithm)", i, n, result, algorithm);
    }
    
    // Demo 3: Building custom polymorphic functions
    println!("\nDemo 3: Guide to creating your own polymorphic functions");
    println!("This framework allows you to create your own polymorphic functions:");
    println!("1. Define multiple implementations with different algorithms");
    println!("2. Create structs that implement the PolymorphicFunction trait");
    println!("3. Use runtime_polymorphic() for runtime selection");
    println!("4. Follow the factorial example for build-time polymorphism");
    
    // Demonstrate string length polymorphism
    let text = "Hello, polymorphic world!";
    let length_impls = custom_example::all_implementations();
    
    // We need to do this manually since we can't use &impl Trait with Boxes
    let idx = rand::thread_rng().gen_range(0..length_impls.len());
    let implementation = &length_impls[idx];
    let length = implementation.execute(text);
    let algorithm = implementation.name();
    println!("\nDemo 4: String length polymorphism");
    println!("Length of '{}' = {} (using {} algorithm)", text, length, algorithm);
}

// Custom factorial implementation for demo purposes
mod custom_example {
    use polymorphic_framework::PolymorphicFunction;
    
    // Example of a string length calculator with different implementations
    
    // Implementation 1: Direct length calculation
    pub struct DirectLength;
    
    impl DirectLength {
        pub fn new() -> Self {
            DirectLength
        }
    }
    
    impl PolymorphicFunction<&str, usize> for DirectLength {
        fn execute(&self, input: &str) -> usize {
            input.len()
        }
        
        fn name(&self) -> &'static str {
            "DirectLength"
        }
    }
    
    // Implementation 2: Character counting
    pub struct CharCount;
    
    impl CharCount {
        pub fn new() -> Self {
            CharCount
        }
    }
    
    impl PolymorphicFunction<&str, usize> for CharCount {
        fn execute(&self, input: &str) -> usize {
            input.chars().count()
        }
        
        fn name(&self) -> &'static str {
            "CharCount"
        }
    }
    
    // Implementation 3: Recursive counting
    pub struct RecursiveCount;
    
    impl RecursiveCount {
        pub fn new() -> Self {
            RecursiveCount
        }
    }
    
    impl PolymorphicFunction<&str, usize> for RecursiveCount {
        fn execute(&self, input: &str) -> usize {
            if input.is_empty() {
                0
            } else {
                1 + self.execute(&input[1..])
            }
        }
        
        fn name(&self) -> &'static str {
            "RecursiveCount"
        }
    }
    
    // Function to get all implementations
    pub fn all_implementations() -> Vec<Box<dyn PolymorphicFunction<&'static str, usize>>> {
        vec![
            Box::new(DirectLength::new()),
            Box::new(CharCount::new()),
            Box::new(RecursiveCount::new()),
        ]
    }
    
    // FACTORIAL IMPLEMENTATIONS
    
    // Implementation 1: Recursive
    pub struct RecursiveFactorial;
    
    impl RecursiveFactorial {
        pub fn new() -> Self {
            RecursiveFactorial
        }
    }
    
    impl PolymorphicFunction<u64, u64> for RecursiveFactorial {
        fn execute(&self, input: u64) -> u64 {
            match input {
                0 | 1 => 1,
                n => n * self.execute(n - 1)
            }
        }
        
        fn name(&self) -> &'static str {
            "RecursiveFactorial"
        }
    }
    
    // Implementation 2: Iterative
    pub struct IterativeFactorial;
    
    impl IterativeFactorial {
        pub fn new() -> Self {
            IterativeFactorial
        }
    }
    
    impl PolymorphicFunction<u64, u64> for IterativeFactorial {
        fn execute(&self, input: u64) -> u64 {
            let mut result = 1;
            for i in 2..=input {
                result *= i;
            }
            result
        }
        
        fn name(&self) -> &'static str {
            "IterativeFactorial"
        }
    }
    
    // Implementation 3: Functional
    pub struct FunctionalFactorial;
    
    impl FunctionalFactorial {
        pub fn new() -> Self {
            FunctionalFactorial
        }
    }
    
    impl PolymorphicFunction<u64, u64> for FunctionalFactorial {
        fn execute(&self, input: u64) -> u64 {
            (1..=input).product()
        }
        
        fn name(&self) -> &'static str {
            "FunctionalFactorial"
        }
    }
    
    // Function to get all factorial implementations
    pub fn get_factorial_implementations() -> Vec<Box<dyn PolymorphicFunction<u64, u64>>> {
        vec![
            Box::new(RecursiveFactorial::new()),
            Box::new(IterativeFactorial::new()),
            Box::new(FunctionalFactorial::new()),
        ]
    }
}
