use proc_macro::TokenStream;

use quote::quote;
use syn::parse::{Parse, ParseStream};
extern crate proc_macro;

// To extract info from the dictionary to construct the model
// https://www.iucr.org/resources/cif/ddl/ddlm/docs/intro
// per data block:
// 1 - split into save frames & global attr
// 2 - extract head save frame from list of save frames
// 3 - construct a tree based on categories
// 4 - construct structs from tree depth first
// 5 - open outermost struct based on _dictionary.namespace (sanitise if necessary)
// 6 - add constant valiues from global attrs

// For each save frame:
// sort into bucket of _name.category_id

#[derive(Debug)]
struct ModelInput {
    name: syn::Ident,
}

impl Parse for ModelInput {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Ident) {
            Ok(ModelInput {
                name: input.parse()?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

#[proc_macro]
pub fn make_model(tokens: TokenStream) -> TokenStream {
    let tokens: proc_macro2::TokenStream = tokens.into();
    let input = syn::parse2::<ModelInput>(tokens);
    let name = match input {
        Ok(input) => input.name,
        Err(err) => return err.to_compile_error().into(),
    };
    let expand = quote! {
        struct #name {
            x: usize
        }
    };
    expand.into()
}
