// Polymorphic Framework for Rust
// A framework for creating truly polymorphic code that changes with each build

pub mod polymorphic {
    use rand::Rng;

    // Include the generated polymorphic code
    include!(concat!(env!("OUT_DIR"), "/polymorphic_impls.rs"));
    
    // Public API for the polymorphism framework
    
    /// Trait that needs to be implemented by any function that wants to be polymorphic
    pub trait PolymorphicFunction<I, O> {
        fn execute(&self, input: I) -> O;
        fn name(&self) -> &'static str;
    }
    
    /// Factory for creating polymorphic implementations
    pub struct PolymorphicFactory;
    
    impl PolymorphicFactory {
        /// Create a new instance of a polymorphic function implementation
        /// The specific implementation is selected at build time
        pub fn create<I, O>() -> Box<dyn PolymorphicFunction<I, O>> {
            // This function is implemented in the generated code
            create_polymorphic_implementation()
        }
    }
    
    /// Runtime polymorphism - selects an implementation at runtime
    /// For direct use with trait objects, see the example in main.rs
    /// This function is useful only with non-boxed trait implementations.
    pub fn runtime_polymorphic<I, O, F>(input: I, implementations: &[F]) -> (O, &'static str) 
    where 
        F: PolymorphicFunction<I, O>,
    {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..implementations.len());
        let implementation = &implementations[idx];
        let result = implementation.execute(input);
        (result, implementation.name())
    }
}

// Re-export the main traits and structs for convenience
pub use polymorphic::{PolymorphicFunction, PolymorphicFactory};

// Example factorial implementations are provided for demonstration
// but users will define their own polymorphic implementations
#[cfg(feature = "examples")]
pub mod examples {
    // Factorial example implementations will be included here
    include!(concat!(env!("OUT_DIR"), "/factorial_examples.rs"));
} 