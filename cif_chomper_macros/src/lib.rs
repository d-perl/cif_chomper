use cif_chomper_macros_core::{ModelMacroInput, make_model_core};
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn make_model(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as ModelMacroInput);
    make_model_core(input).into()
}
