#![doc = include_str!("../README.md")]

mod tests;

use proc_macro2::TokenStream;


pub fn derive_macro_impl(input: TokenStream) -> TokenStream {
    TokenStream::new()
}
