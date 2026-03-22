use crate::model::BlockItem::SaveFrame;
use crate::model::DataItem;
use crate::model::DataValue;
use crate::parser::cif2_file;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use std::io::Write;
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

pub fn cif_vessel_codegen<W>(f: &mut W, content: &str)
where
    W: Write,
{
    let model = cif2_file(content).expect("parse templ_enum");

    let block = &model.content[0];
    for bitem in &block.content {
        if let SaveFrame { heading, content } = bitem {
            let inner_name = format!("__INNER_STATIC_{}", heading.to_uppercase());
            for data in content {
                match data {
                    DataItem::Data { .. } => {}
                    DataItem::DataLoop { names, values } => {
                        // state: detail enumeration
                        // construct as a phf ordered map
                        if names.len() == 2
                            && names[0] == "_enumeration_set.state"
                            && names[1] == "_enumeration_set.detail"
                        {
                            let n_pairs = values.len() / 2;
                            let mut entries = Vec::with_capacity(n_pairs);
                            for i in 0..n_pairs {
                                let DataValue::Str(k) = values[i * 2] else {
                                    panic!("must be string")
                                };
                                let DataValue::Str(v) = values[i * 2 + 1] else {
                                    panic!("must be string")
                                };
                                let v = format!("\"{v}\"");
                                entries.push((k, v));
                            }

                            let mut builder = phf_codegen::OrderedMap::new();
                            for &(key, ref value) in &entries {
                                builder.entry(key, value);
                            }

                            writeln!(
                                f,
                                "static {}: phf::OrderedMap<&'static str, &'static str> = \n{};\n",
                                inner_name,
                                builder.build()
                            )
                            .unwrap();
                        }

                        // state only enumeration
                        // construct as a phf ordered set
                        if names.len() == 1 && names[0] == "_enumeration_set.state" {
                            let n = values.len();
                            let mut entries = Vec::with_capacity(n);
                            (0..n).for_each(|i| {
                                let DataValue::Str(v) = values[i] else {
                                    panic!("must be string")
                                };
                                // let v = format!("\"{v}\"");
                                entries.push(v);
                            });

                            let mut builder = phf_codegen::OrderedSet::new();
                            for &value in &entries {
                                builder.entry(value);
                            }

                            writeln!(
                                f,
                                "static {}: phf::OrderedSet<&'static str> = \n{};\n",
                                inner_name,
                                builder.build()
                            )
                            .unwrap();
                        }

                        // index: value enumeration_default
                        // construct as a phf ordered map
                        if names.len() == 2
                            && names[0] == "_enumeration_default.index"
                            && names[1] == "_enumeration_default.value"
                        {
                            let n_pairs = values.len() / 2;
                            let mut entries = Vec::with_capacity(n_pairs);
                            for i in 0..n_pairs {
                                let DataValue::Str(k) = values[i * 2] else {
                                    panic!("must be string")
                                };
                                let DataValue::Str(v) = values[i * 2 + 1] else {
                                    panic!("must be string")
                                };
                                let v = format!("\"{v}\"");
                                entries.push((k, v));
                            }

                            let mut builder = phf_codegen::OrderedMap::new();
                            for &(key, ref value) in &entries {
                                builder.entry(key, value);
                            }

                            writeln!(
                                f,
                                "static {}: phf::OrderedMap<&'static str, &'static str> = \n{};\n",
                                inner_name,
                                builder.build()
                            )
                            .unwrap();
                        }
                    }
                }
            }
        }
    }

    // TODO: get a second table and have parent points its to children
    // TODO: dfs the tree from head, codegen the struct along the traversal.
    // let tokens = quote! {
    //     #(#tks)*
    // };
    // let file: File = syn::parse2(tokens).unwrap();
    //
    // prettyplease::unparse(&file)
    // todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
}
