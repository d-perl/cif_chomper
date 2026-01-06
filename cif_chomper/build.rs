use std::fs;
use std::path::Path;

fn cif_vessel_codegen() -> &'static str {
    let ddl = include_str!("../cif_core/cif_core.dic");
    let source = "pub fn message() -> &'static str {
        \"Hello, World!ii\"
    }
    ";
    source
}

fn main() {
    let dest_path = Path::new("./src/__cif_vessel.rs");
    let cif_vessel_src = cif_vessel_codegen();
    fs::write(dest_path, cif_vessel_src).unwrap();

    println!("cargo::rerun-if-changed=./cif_core/cif_core.dic");
    println!("cargo::rerun-if-changed=build.rs");
}
