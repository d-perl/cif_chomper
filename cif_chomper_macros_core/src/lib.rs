use std::cell::LazyCell;

use cif_chomper_core::parser::cif2_file;
use cif_chomper_core::raw_model::{RawDataBlock, RawDataItem, RawDataItemContent, RawModel};
use quote::quote;
use syn::parse::{Parse, ParseStream};
extern crate proc_macro;
use proc_macro2::TokenStream;

const DDL: &str = include_str!("../../cif_core/ddl.dic");
const DDL_MODEL: LazyCell<RawModel> = LazyCell::new(|| cif2_file(DDL).unwrap());
const DICT: &str = include_str!("../../cif_core/cif_core.dic");
const DICT_MODEL: LazyCell<RawModel> = LazyCell::new(|| cif2_file(DICT).unwrap());

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

fn match_data_item(data_item: &RawDataItem) {
    match &data_item {
        RawDataItem::Data { name, value } => match &value {
            RawDataItemContent::Str(v) => {
                println!("content str {name}, {v}");
            }
            _ => (),
        },
        RawDataItem::SaveFrame { name, content } => {
            println!("\n SAVE FRAME {name} \n");
            content.iter().for_each(match_data_item);
        }
        _ => (),
    }
}
fn iterate_data_block(data_block: &RawDataBlock) {
    data_block.content.iter().for_each(match_data_item);
}

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

#[cfg(test)]
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
        let content = &DDL_MODEL.content;
        assert!(content[0].content.len() > 5);
    }

    #[test]
    fn test_ddl_model_content() {
        let content = &DDL_MODEL.content;
        dbg!(&DDL_MODEL.heading);
        for data_block in content.as_slice()[0..1].iter() {
            iterate_data_block(data_block);
        }
    }

    #[test]
    fn test_dict_model_content() {
        let content = &DICT_MODEL.content;
        dbg!(&DICT_MODEL.heading);
        for data_block in content.as_slice()[0..1].iter() {
            iterate_data_block(data_block);
        }
    }
}
