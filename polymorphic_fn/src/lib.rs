extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, Block, FnArg, Ident, Token, Type, Visibility,
};
use rand::Rng;

struct PolymorphicFnInput {
    vis: Visibility,
    fn_token: Token![fn],
    name: Ident,
    paren_token: token::Paren,
    inputs: Punctuated<FnArg, Token![,]>,
    arrow_token: Token![->],
    output: Type,
    brace_token: token::Brace,
    implementations: Punctuated<Block, Token![,]>,
}

impl Parse for PolymorphicFnInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let vis = input.parse()?;
        let fn_token = input.parse()?;
        let name = input.parse()?;
        let content;
        let paren_token = syn::parenthesized!(content in input);
        let inputs = Punctuated::parse_terminated(&content)?;
        let arrow_token = input.parse()?;
        let output = input.parse()?;
        let brace_content;
        let brace_token = syn::braced!(brace_content in input);
        let implementations = Punctuated::parse_terminated(&brace_content)?;
        Ok(PolymorphicFnInput {
            vis,
            fn_token,
            name,
            paren_token,
            inputs,
            arrow_token,
            output,
            brace_token,
            implementations,
        })
    }
}

/// Defines a function with multiple implementations, where one is randomly selected at compile-time.
///
/// At compile-time, the macro randomly selects one of the provided implementation blocks and
/// generates a function containing only that implementation. Other implementations are excluded
/// from the final binary. Each compilation may select a different implementation.
///
/// # Syntax
///
/// ```rust
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
/// polymorphic_fn! {
///     pub fn compute(x: i32) -> i32 {
///         { x + 1 },
///         { x * 2 },
///         { x - 3 }
///     }
/// }
///
/// fn main() {
///     println!("Result: {}", compute(5));
/// }
/// ```
///
/// In this example, each time the code is compiled, `compute` will randomly be one of:
/// - `x + 1` (returns 6 for input 5),
/// - `x * 2` (returns 10 for input 5),
/// - `x - 3` (returns 2 for input 5).
#[proc_macro]
pub fn polymorphic_fn(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as PolymorphicFnInput);
    let implementations: Vec<Block> = input.implementations.into_iter().collect();
    if implementations.is_empty() {
        return syn::Error::new_spanned(&input.name, "At least one implementation must be provided")
        .to_compile_error()
        .into();
    }
    let mut rng = rand::thread_rng();
    let selected_index = rng.gen_range(0..implementations.len());
    let selected_block = &implementations[selected_index];
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