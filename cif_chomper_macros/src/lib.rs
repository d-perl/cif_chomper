use cif_chomper_core::model::{RawDataBlock, RawDataItem, RawModel};
use cif_chomper_core::parser::cif2_file;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Pat};

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
