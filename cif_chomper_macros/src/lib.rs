use cif_chomper_core::model::{RawDataBlock, RawDataItem, RawModel};
use cif_chomper_core::parser::cif2_file;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Pat};

// make struct out of given text
struct Creator(Pat);

impl Parse for Creator {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        todo!()
    }
}
