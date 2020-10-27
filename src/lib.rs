extern crate proc_macro;

use proc_macro::TokenStream;

/// Returns the original token stream.
#[proc_macro_attribute]
pub fn is(_attributes: TokenStream, input: TokenStream) -> TokenStream {
    input
}

/// Returns an empty token stream.
#[proc_macro_attribute]
pub fn not(_attributes: TokenStream, _input: TokenStream) -> TokenStream {
    TokenStream::new()
}
