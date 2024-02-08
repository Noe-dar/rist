use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{Ident, LitInt};

use crate::common::get_instruction_table;

pub(crate) fn expand(opcode: Ident) -> syn::Result<TokenStream> {
    let instruction_table = get_instruction_table()?;

    let instruction = instruction_table
        .get(&opcode.to_string())
        .ok_or(syn::Error::new(Span::call_site(), "unknown instruction"))?;

    Ok(LitInt::new(&instruction.mask, Span::call_site()).into_token_stream())
}
