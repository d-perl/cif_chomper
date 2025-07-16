use cif_chomper_macros::make_model;
const DICT: &str = include_str!("../../cif_core/cif_core.dic");

#[test]
fn it_works() {
    make_model!(StructName);
    assert_eq!(answer(), 42);
    assert_eq!(StructName { x: 6 }.x, 6);
}
