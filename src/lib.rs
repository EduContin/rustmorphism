extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Block, FnArg, Ident, Token, Type, Visibility,
};

struct PolymorphicFnInput {
    vis: Visibility,
    fn_token: Token![fn],
    name: Ident,
    inputs: Punctuated<FnArg, Token![,]>,
    output: Type,
    implementations: Punctuated<Block, Token![,]>,
}

impl Parse for PolymorphicFnInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let vis = input.parse()?;
        let fn_token = input.parse()?;
        let name = input.parse()?;
        let content;
        let _paren_token = syn::parenthesized!(content in input);
        let inputs = Punctuated::parse_terminated(&content)?;
        let _arrow_token = input.parse::<Token![->]>()?;
        let output = input.parse()?;
        let brace_content;
        let _brace_token = syn::braced!(brace_content in input);
        let implementations = Punctuated::parse_terminated(&brace_content)?;
        Ok(PolymorphicFnInput {
            vis,
            fn_token,
            name,
            inputs,
            output,
            implementations,
        })
    }
}

/// Defines a function with multiple implementations, where one is deterministically selected at compile-time.
///
/// At compile-time, the macro selects one of the provided implementation blocks based on a hash of
/// source location and other build identifiers. Other implementations are excluded from the final binary.
///
/// # Syntax
///
/// ```text
/// polymorphic_fn! {
///     [vis] fn name([args]) -> return_type {
///         { implementation1 },
///         { implementation2 },
///         ...
///     }
/// }
/// ```
///
/// - `[vis]`: Optional visibility (e.g., `pub`), defaults to private if omitted.
/// - `name`: The function name.
/// - `[args]`: Comma-separated argument list (e.g., `x: i32`).
/// - `return_type`: The return type of the function (e.g., `i32`).
/// - `{ implementation }`: A block containing an implementation; at least one must be provided.
///
/// # Example
///
/// ```rust
/// use rustmorphism::polymorphic_fn;
///
/// polymorphic_fn! {
///     pub fn compute(x: i32) -> i32 {
///         { x + 1 },
///         { x * 2 },
///         { x - 3 }
///     }
/// }
///
/// println!("Result: {}", compute(5));
/// ```
#[proc_macro]
pub fn polymorphic_fn(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as PolymorphicFnInput);
    let implementations: Vec<&Block> = input.implementations.iter().collect();
    if implementations.is_empty() {
        return syn::Error::new_spanned(
            &input.name,
            "At least one implementation must be provided",
        )
        .to_compile_error()
        .into();
    }
    let selected_index = deterministic_selection(&input.name, implementations.len());
    let selected_block = implementations[selected_index];
    let vis = &input.vis;
    let fn_token = &input.fn_token;
    let name = &input.name;
    let inputs = &input.inputs;
    let output = &input.output;
    let output = quote! {
        #vis #fn_token #name(#inputs) -> #output #selected_block
    };
    output.into()
}

/// Deterministically selects an implementation based on build identifiers
fn deterministic_selection(name: &Ident, count: usize) -> usize {
    // Create a hasher for deterministic selection
    let mut hasher = DefaultHasher::new();
    // Hash the function name
    name.to_string().hash(&mut hasher);
    // Use various environment variables for entropy
    if let Ok(timestamp) = std::env::var("BUILD_TIMESTAMP") {
        timestamp.hash(&mut hasher);
    }
    if let Ok(package) = std::env::var("CARGO_PKG_NAME") {
        package.hash(&mut hasher);
    }
    if let Ok(version) = std::env::var("CARGO_PKG_VERSION") {
        version.hash(&mut hasher);
    }
    if let Ok(target) = std::env::var("TARGET") {
        target.hash(&mut hasher);
    }
    if let Ok(profile) = std::env::var("PROFILE") {
        profile.hash(&mut hasher);
    }
    // If we need more entropy, we can use a counter that increments on every build
    // through the build.rs mechanism
    if let Ok(counter) = std::env::var("BUILD_COUNTER") {
        counter.hash(&mut hasher);
    }
    // Get the hash value and select an implementation
    let hash = hasher.finish();
    (hash as usize) % count
}
