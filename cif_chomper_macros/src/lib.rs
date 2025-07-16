<<<<<<< HEAD
use std::cell::LazyCell;

use cif_chomper_core::model::{RawDataBlock, RawDataItem, RawModel};
use cif_chomper_core::parser::cif2_file;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Pat};

extern crate proc_macro;
use proc_macro::TokenStream;

const DDL: &str = include_str!("../../cif_core/ddl.dic");
const DDL_MODEL: LazyCell<RawModel> = LazyCell::new(|| cif2_file(DDL).unwrap());

#[proc_macro]
pub fn make_model(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

=======
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
>>>>>>> main
mod tests {
    use super::*;

    #[test]
<<<<<<< HEAD
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
=======
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
>>>>>>> main
}
