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
    let s = r#"save_import_details.single

    _definition.id                '_import_details.single'
    _definition.class             Attribute
    _definition.update            2017-07-21
    _description.text
;
    A Table mapping attributes defined individually in category IMPORT to
    their values; used to import definitions from other dictionaries.
;
    _name.category_id             import_details
    _name.object_id               single
    _type.purpose                 Internal
    _type.source                  Assigned
    _type.container               Table
    _type.contents                Text
    _type.indices                 ByReference
    _type.indices_referenced_id   '_import_details.single_index'
    _method.purpose               Evaluation
    _method.expression
;
    with id as import_details
    _import_details.single = {"file":id.file_id,
                              "version":id.file_version,
                              "save":id.frame_id,
                              "mode":id.mode,
                              "dupl":id.if_dupl,
                              "miss":id.if_miss}
;

save_"#;

    let (_, blocks) = save_frame(s).unwrap();
    dbg!(blocks);
}
