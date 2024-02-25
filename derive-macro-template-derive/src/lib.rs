#![doc = include_str!("../README.md")]

use derive_macro_template_core::derive_macro_impl;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_error]
#[proc_macro_derive(DeriveMacro)]
pub fn derive_macro(input: TokenStream) -> TokenStream {
    derive_macro_impl(input.into()).into()
}