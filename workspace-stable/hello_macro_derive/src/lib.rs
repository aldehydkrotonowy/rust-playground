/*
The next step is to define the procedural macro. At the time of this writing,
procedural macros need to be in their own crate. Eventually, this restriction
might be lifted. The convention for structuring crates and macro crates is as
follows: for a crate named foo, a custom derive procedural macro crate
is called foo_derive.
*/

// compiler’s API that allows us to read and manipulate
// Rust code from our code.
extern crate proc_macro;

use proc_macro::TokenStream;

// The syn crate parses Rust code from a string into a data structure
// that we can perform operations on
// use syn;

// The quote crate turns syn data structures back into Rust code.
use quote::quote;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    // build the trait implementation
    // outer function is usually the same
    // inner function will by different depending on
    // needs
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    gen.into() // into TokenStream
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
