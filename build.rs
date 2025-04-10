use rand::Rng;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    let mut rng = rand::thread_rng();
    let out_dir = env::var_os("OUT_DIR").unwrap();
    
    // Generate the polymorphic implementations infrastructure
    generate_polymorphic_framework(&mut rng, &out_dir);
    
    // Generate example factorial implementations
    generate_factorial_examples(&mut rng, &out_dir);
}

fn generate_polymorphic_framework(_rng: &mut rand::rngs::ThreadRng, out_dir: &std::ffi::OsStr) {
    let dest_path = Path::new(out_dir).join("polymorphic_impls.rs");
    
    // Core framework for polymorphic function generation
    let content = r#"// Auto-generated polymorphic implementation framework

// Helper function to create a random polymorphic implementation when requested
// This will be specialized for each specific function type at compile time
// through type inference and monomorphization
pub fn create_polymorphic_implementation<I, O>() -> Box<dyn super::PolymorphicFunction<I, O>> {
    // This is a placeholder - when users implement their own polymorphic functions,
    // they will provide implementations of this function in their own modules
    panic!("No polymorphic implementation available for this type signature. 
                    You need to implement your own polymorphic functions.");
}

// Common obfuscation utilities for use in generated code

/// Generates random whitespace to obfuscate code formatting
pub fn obfuscate_whitespace(code: &str) -> String {
    let mut obfuscated = code.to_string();
    
    if rand::thread_rng().gen_bool(0.5) {
        obfuscated = obfuscated.replace(" ", "  ");
    }
    
    if rand::thread_rng().gen_bool(0.3) {
        obfuscated = obfuscated.lines()
            .map(|line| format!("{}    ", line))
            .collect::<Vec<_>>()
            .join("\n");
    }
    
    obfuscated
}

/// Adds random bogus comments to code
pub fn add_random_comments(code: &str) -> String {
    let comments = vec![
        "// Processing data",
        "// Optimized path",
        "// Handle edge case",
        "// Core algorithm implementation",
        "// Compute result",
    ];
    
    let mut lines: Vec<&str> = code.lines().collect();
    let comment_count = rand::thread_rng().gen_range(1..5);
    
    for _ in 0..comment_count {
        if !lines.is_empty() {
            let line_idx = rand::thread_rng().gen_range(0..lines.len());
            let comment = comments[rand::thread_rng().gen_range(0..comments.len())];
            lines.insert(line_idx, comment);
        }
    }
    
    lines.join("\n")
}

/// Generate a random variable name
pub fn generate_random_name(min_len: usize, max_len: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    let len = rand::thread_rng().gen_range(min_len..=max_len);
    
    let mut name = String::with_capacity(len);
    for _ in 0..len {
        let idx = rand::thread_rng().gen_range(0..CHARSET.len());
        name.push(CHARSET[idx] as char);
    }
    name
}

/// Adds bogus code that doesn't affect functionality
pub fn add_bogus_code(code: &str) -> String {
    let bogus_statements = vec![
        "    let _v = 0;".to_string(),
        "    let _unused = 42;".to_string(),
        "    let _tmp = if true { 1 } else { 0 };".to_string(),
        "    let _ = {}; // No-op".to_string(),
    ];
    
    let mut lines: Vec<&str> = code.lines().collect();
    let bogus_count = rand::thread_rng().gen_range(1..4);
    
    for _ in 0..bogus_count {
        if lines.len() > 2 {
            let line_idx = rand::thread_rng().gen_range(1..lines.len());
            let stmt_idx = rand::thread_rng().gen_range(0..bogus_statements.len());
            lines.insert(line_idx, &bogus_statements[stmt_idx]);
        }
    }
    
    lines.join("\n")
}

/// Trait for defining a polymorphic implementation generator
pub trait PolymorphicGenerator {
    fn generate(&self) -> String;
    fn name(&self) -> &'static str;
}

/// Helper for registering polymorphic implementations
#[macro_export]
macro_rules! register_polymorphic_implementation {
    ($input_type:ty, $output_type:ty, $generators:expr) => {
        // Define a specialized create_polymorphic_implementation function
        pub fn create_polymorphic_implementation() -> Box<dyn super::PolymorphicFunction<$input_type, $output_type>> {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let generators = $generators;
            let idx = rng.gen_range(0..generators.len());
            let code = generators[idx].generate();
            
            // Here we would dynamically compile and load the code
            // For now, we'll just select from a set of predefined implementations
            todo!("Dynamic code generation not yet implemented")
        }
    };
}
"#;
    
    fs::write(dest_path, content).unwrap();
}

fn generate_factorial_examples(rng: &mut rand::rngs::ThreadRng, out_dir: &std::ffi::OsStr) {
    let dest_path = Path::new(out_dir).join("factorial_examples.rs");
    
    // Select which implementation to use
    let impl_choice = rng.gen_range(0..5);
    
    // Generate random variable and function names to obfuscate code
    let fn_name = "factorial_inner".to_string(); // Use a fixed name that main.rs expects
    let var_acc = generate_random_name(rng, 3, 7);
    let var_i = generate_random_name(rng, 2, 5);
    let var_n = generate_random_name(rng, 2, 5);
    let var_result = generate_random_name(rng, 3, 8);
    let helper_fn = generate_random_name(rng, 4, 9);
    
    // Generate the actual implementation based on random selection
    let factorial_implementation = match impl_choice {
        0 => generate_recursive_factorial(&fn_name, &var_n),
        1 => generate_iterative_factorial(&fn_name, &var_n, &var_result, &var_i),
        2 => generate_fold_factorial(&fn_name, &var_n, &var_acc),
        3 => generate_functional_factorial(&fn_name, &var_n),
        _ => generate_tailrec_factorial(&fn_name, &var_n, &helper_fn, &var_acc),
    };
    
    // Apply random obfuscation techniques
    let obfuscated_implementation = apply_obfuscation(factorial_implementation, rng);
    
    // Define polymorphic factorial structs
    let struct_names = [
        "RecursiveFactorial",
        "IterativeFactorial",
        "FoldFactorial",
        "FunctionalFactorial",
        "TailRecursiveFactorial"
    ];
    
    // Combine all implementations into a comprehensive example module
    let mut example_impls = String::new();
    
    // Add imports
    example_impls.push_str("use crate::polymorphic::PolymorphicFunction;\n\n");
    
    // Add the factorial struct implementations
    for (i, name) in struct_names.iter().enumerate() {
        // Generate a unique implementation for each struct
        let impl_fn = match i {
            0 => generate_recursive_factorial("execute_impl", "n"),
            1 => generate_iterative_factorial("execute_impl", "n", "result", "i"),
            2 => generate_fold_factorial("execute_impl", "n", "acc"),
            3 => generate_functional_factorial("execute_impl", "n"),
            _ => generate_tailrec_factorial("execute_impl", "n", "helper", "acc"),
        };
        
        example_impls.push_str(&format!(
r#"
pub struct {0} {{}}

impl {0} {{
    pub fn new() -> Self {{
        {0} {{}}
    }}
    
    {1}
}}

impl PolymorphicFunction<u64, u64> for {0} {{
    fn execute(&self, input: u64) -> u64 {{
        self.execute_impl(input)
    }}
    
    fn name(&self) -> &'static str {{
        "{0}"
    }}
}}
"#,
            name,
            impl_fn.replace(&format!("pub fn execute_impl"), "fn execute_impl")
        ));
    }
    
    // Add a function to get all implementations
    example_impls.push_str(&format!(
r#"
/// Returns all available factorial implementations
pub fn all_factorial_implementations() -> Vec<Box<dyn PolymorphicFunction<u64, u64>>> {{
    vec![
        Box::new(RecursiveFactorial::new()),
        Box::new(IterativeFactorial::new()),
        Box::new(FoldFactorial::new()),
        Box::new(FunctionalFactorial::new()),
        Box::new(TailRecursiveFactorial::new()),
    ]
}}

/// Create a polymorphic factorial implementation (selected at build time)
pub fn create_factorial() -> Box<dyn PolymorphicFunction<u64, u64>> {{
    // Select one implementation based on build-time randomness
    match {0} {{
        0 => Box::new(RecursiveFactorial::new()),
        1 => Box::new(IterativeFactorial::new()),
        2 => Box::new(FoldFactorial::new()),
        3 => Box::new(FunctionalFactorial::new()),
        _ => Box::new(TailRecursiveFactorial::new()),
    }}
}}

// Original generated factorial implementation
{1}
"#,
        impl_choice,
        obfuscated_implementation
    ));
    
    // Write the example implementations
    fs::write(dest_path, example_impls).unwrap();
}

// Helper functions for generating different factorial implementations
fn generate_random_name(rng: &mut rand::rngs::ThreadRng, min_len: usize, max_len: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    let len = rng.gen_range(min_len..=max_len);
    
    let mut name = String::with_capacity(len);
    for _ in 0..len {
        let idx = rng.gen_range(0..CHARSET.len());
        name.push(CHARSET[idx] as char);
    }
    name
}

fn generate_recursive_factorial(fn_name: &str, var_n: &str) -> String {
    format!(
        r#"
pub fn {}({}: u64) -> u64 {{
    match {} {{
        0 | 1 => 1,
        _ => {} * {}({} - 1)
    }}
}}
"#,
        fn_name, var_n, var_n, var_n, fn_name, var_n
    )
}

fn generate_iterative_factorial(fn_name: &str, var_n: &str, var_result: &str, var_i: &str) -> String {
    format!(
        r#"
pub fn {}({}: u64) -> u64 {{
    let mut {}: u64 = 1;
    for {} in 1..={} {{
        {} *= {};
    }}
    {}
}}
"#,
        fn_name, var_n, var_result, var_i, var_n, var_result, var_i, var_result
    )
}

fn generate_fold_factorial(fn_name: &str, var_n: &str, var_acc: &str) -> String {
    format!(
        r#"
pub fn {}({}: u64) -> u64 {{
    (1..={}).fold(1, |{}, x| {} * x)
}}
"#,
        fn_name, var_n, var_n, var_acc, var_acc
    )
}

fn generate_functional_factorial(fn_name: &str, var_n: &str) -> String {
    format!(
        r#"
pub fn {}({}: u64) -> u64 {{
    (1..={}).product()
}}
"#,
        fn_name, var_n, var_n
    )
}

fn generate_tailrec_factorial(fn_name: &str, var_n: &str, helper_fn: &str, var_acc: &str) -> String {
    format!(
        r#"
pub fn {}({}: u64) -> u64 {{
    fn {}({}: u64, {}: u64) -> u64 {{
        match {} {{
            0 | 1 => {},
            _ => {}({} - 1, {} * {})
        }}
    }}
    {}({}, 1)
}}
"#,
        fn_name, var_n, helper_fn, var_n, var_acc, var_n, var_acc, helper_fn, var_n, var_acc, var_n, helper_fn, var_n
    )
}

fn apply_obfuscation(implementation: String, rng: &mut rand::rngs::ThreadRng) -> String {
    let mut obfuscated = implementation;
    
    // Apply random whitespace variations
    if rng.gen_bool(0.5) {
        obfuscated = obfuscated.replace(" ", "  ");
    }
    
    // Add random comments
    if rng.gen_bool(0.7) {
        let comments = vec![
            "// This performs the calculation",
            "// Optimized path",
            "// Handle edge case",
            "// Process input",
            "// Return result",
        ];
        
        for _ in 0..rng.gen_range(1..4) {
            let comment = comments[rng.gen_range(0..comments.len())];
            let lines: Vec<&str> = obfuscated.lines().collect();
            if !lines.is_empty() {
                let line_idx = rng.gen_range(0..lines.len());
                let mut new_lines = lines.clone();
                new_lines.insert(line_idx, comment);
                obfuscated = new_lines.join("\n");
            }
        }
    }
    
    // Add bogus variable declarations
    if rng.gen_bool(0.6) {
        let var_declarations = vec![
            format!("    let _v{} = 0;", rng.gen_range(1..100)),
            format!("    let _unused{} = {};", rng.gen_range(1..100), rng.gen_range(1..1000)),
            format!("    let _tmp{} = if true {{ 1 }} else {{ 0 }};", rng.gen_range(1..100)),
        ];
        
        let idx = rng.gen_range(0..var_declarations.len());
        let declaration = &var_declarations[idx];
        let lines: Vec<&str> = obfuscated.lines().collect();
        if lines.len() > 2 {
            let line_idx = rng.gen_range(2..lines.len());
            let mut new_lines = lines.clone();
            new_lines.insert(line_idx, declaration);
            obfuscated = new_lines.join("\n");
        }
    }
    
    obfuscated
} 