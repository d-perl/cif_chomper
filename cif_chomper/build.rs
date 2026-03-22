use std::{fs::File, io::BufWriter, path::Path};

use cif_chomper_core::codegen::cif_vessel_codegen;

fn main() {
    let dst_path = Path::new("./src/__cif_vessel.rs");
    let mut file = BufWriter::new(File::create(dst_path).unwrap());

    let templ_enum = include_str!("./templ_enum.cif");

    cif_vessel_codegen(&mut file, templ_enum);

    println!("cargo::rerun-if-changed=./cif_core/cif_core.dic");
    println!("cargo::rerun-if-changed=build.rs");
}
