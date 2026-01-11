// use cif_chomper_core::model::Model;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
// use std::collections::HashMap;
use std::fs;
use std::path::Path;
use syn::File;
use syn::Ident;

struct Item {
    name: String, // "_cell.angle_alpha" -> angle_alpha
    ty: String,   // "Option<f64>" if SU
}

struct Category {
    name: String, // "Cell"
    items: Vec<Item>,
}

// struct SaveData {}
// struct SaveCategory {}
//
// enum SaveFrame {
//     Data(SaveData),
//     Category(SaveCategory),
// }

// fn convert_model_to_tbl(model: Model) -> HashMap<(&str, &str), SaveFrame> {
//     todo!();
// }

fn generate_struct(category: &Category) -> TokenStream {
    let struct_name = Ident::new(&category.name, Span::call_site());

    let fields: Vec<TokenStream> = category
        .items
        .iter()
        .map(|item| {
            let field_name = Ident::new(&item.name, Span::call_site());
            let ty: syn::Type = syn::parse_str(&item.ty).unwrap();
            quote! {
                pub #field_name: #ty
            }
        })
        .collect();

    quote! {
        #[derive(Debug, Clone)]
        pub struct #struct_name {
            #(#fields),*
        }
    }
}

fn cif_vessel_codegen() -> String {
    // let ddl = include_str!("../cif_core/cif_core.dic");
    // let ddl = include_str!("./cif_core_example.dic");
    // let model = cif2_file(ddl).expect("parse cif_core");
    // let tbl = convert_model_to_tbl(model);
    // TODO: get a second table and have parent points its to children
    // TODO: dfs the tree from head, codegen the struct along the traversal.
    let cell = Category {
        name: "Cell".to_string(),
        items: vec![
            Item {
                name: "angle_alpha".to_string(),
                ty: "f64".to_string(),
            },
            Item {
                name: "angle_alpha_su".to_string(),
                ty: "Option<f64>".to_string(),
            },
        ],
    };

    let tokens = generate_struct(&cell);
    let file: File = syn::parse2(tokens).unwrap();

    prettyplease::unparse(&file)
}

fn main() {
    let dest_path = Path::new("./src/__cif_vessel.rs");
    let cif_vessel_src = cif_vessel_codegen();
    fs::write(dest_path, cif_vessel_src).unwrap();

    println!("cargo::rerun-if-changed=./cif_core/cif_core.dic");
    println!("cargo::rerun-if-changed=build.rs");
}
