use cif_chomper_core::parser::cif2_file;
// use cif_chomper_core::parser::{block, save_frame};

fn main() {
    // let raw_content = include_str!("../../cif_core/cif_core.dic");

    let raw_content = include_str!("../../cif_chomper/cif_core_example.dic");
    let model = cif2_file(raw_content).expect("parse fail");
    dbg!(model);
    // let content = model.content;
    // dbg!(model.heading);
    // dbg!(content);
}
