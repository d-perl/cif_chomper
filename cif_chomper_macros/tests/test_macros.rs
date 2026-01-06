use cif_chomper_macros::make_model;

#[test]
fn test_macro_make_model() {
    make_model!(StructName);
    assert_eq!(StructName { x: 6 }.x, 6);
}
