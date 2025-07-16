use std::cell::LazyCell;

use cif_chomper_core::model::{RawDataBlock, RawDataItem, RawModel};
use cif_chomper_core::parser::cif2_file;
use proc_macro2::Span;
use quote::quote;
use syn::parse::{Parse, ParseStream};
extern crate proc_macro;
use proc_macro2::{Literal, TokenStream};

const DDL: &str = include_str!("../../cif_core/ddl.dic");
const DDL_MODEL: LazyCell<RawModel> = LazyCell::new(|| cif2_file(DDL).unwrap());
const DICT: &str = include_str!("../../cif_core/cif_core.dic");
const DICT_MODEL: LazyCell<RawModel> = LazyCell::new(|| cif2_file(DICT).unwrap());

#[derive(Debug)]
pub struct ModelMacroInput {
    name: syn::Ident,
}

impl Parse for ModelMacroInput {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Ident) {
            Ok(ModelMacroInput {
                name: input.parse()?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

pub fn make_model_core(input: ModelMacroInput) -> TokenStream {
    let name = input.name;
    quote! {
        struct #name {
            x: usize
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_load_ddl_str() {
        assert!(DDL.len() > 100);
    }

    #[test]
    fn test_load_ddl_model() {
        println!("{:?}", DDL_MODEL.content[0].heading);
        println!("{:?}", DDL_MODEL.content[0].content.len());
        assert!(DDL_MODEL.content[0].content.len() > 5);
    }

    #[test]
    fn test_ddl_model_content() {}
}
