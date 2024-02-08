pub(crate) mod bake_opcode;
pub(crate) mod instruction_table;
pub(crate) mod common;
pub(crate) mod bake_mask;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Error, Ident};

#[proc_macro]
pub fn bake_opcode(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as Ident);

    bake_opcode::expand(input).unwrap_or_else(Error::into_compile_error).into()
}

#[proc_macro]
pub fn bake_mask(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as Ident);

    bake_mask::expand(input).unwrap_or_else(Error::into_compile_error).into()
}