use cif_chomper_core::parser::cif2_file;

fn main() {
    let raw_content = include_str!("../templ_enum.cif");
    let model = cif2_file(raw_content).expect("parse fail");
    dbg!(model);
}
