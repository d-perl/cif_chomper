use cif_chomper_core::parser::{cif2_file, data_block, save_frame};

fn main() {
    // let raw_content = include_str!("../../cif_core/cif_core.dic");

    // let raw_content = include_str!("../../cif_chomper/example_data/mil-101.cif");
    // let model = cif2_file(raw_content).expect("parse fail");
    // let content = model.content;
    // dbg!(model.heading);
    // dbg!(content);

    let s = "save_diffrn.id

    _definition.id                '_diffrn.id'
    _definition.update            2022-05-09
    _description.text
;
    Unique identifier for a diffraction data set collected under
    particular diffraction conditions.
;
    _name.category_id             diffrn
    _name.object_id               id
    _type.purpose                 Key
    _type.source                  Assigned
    _type.container               Single
    _type.contents                Word

save_";
    let (_, blocks) = save_frame(s).unwrap();
    dbg!(blocks);
}
