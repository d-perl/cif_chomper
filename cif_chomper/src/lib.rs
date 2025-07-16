use cif_chomper_macros::make_model;

#[cfg(test)]
mod macro_tests {
    use super::*;

    #[test]
    fn it_works() {
        make_model!("test");
        assert_eq!(answer(), 42);
    }
}
