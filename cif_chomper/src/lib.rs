pub mod vessel {
    include!("./__cif_vessel.rs");
}

mod _vessel_be_like;

// use vessel::Cell;

pub type Result<T> = std::result::Result<T, CifParserError>;

#[derive(Debug)]
pub enum CifParserError {
    Io(std::io::Error),
    // WrapperError(CextxyzError),
    // InvalidValue(&'static str),
}

impl std::fmt::Display for CifParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CifParserError::Io(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for CifParserError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CifParserError::Io(error) => Some(error),
            // _ => None,
        }
    }
}

// pub fn read_structure<R>(rd: &mut R) -> Result<Structure> {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use cif_chomper_core::model::Model;
    use cif_chomper_core::parser::cif2_file;
    use std::sync::OnceLock;

    static DDL: &str = include_str!("../../cif_core/ddl.dic");
    static DDL_MODEL: OnceLock<Model<'static>> = OnceLock::new();

    pub fn ddl_model() -> &'static Model<'static> {
        DDL_MODEL.get_or_init(|| cif2_file(DDL).unwrap())
    }

    const DICT: &str = include_str!("../../cif_core/cif_core.dic");
    static DICT_MODEL: OnceLock<Model<'static>> = OnceLock::new();

    pub fn dict_model() -> &'static Model<'static> {
        DICT_MODEL.get_or_init(|| cif2_file(DICT).unwrap())
    }

    #[test]
    fn test_load_ddl_str() {
        assert!(DDL.len() > 100);
    }

    #[test]
    fn test_load_ddl_model() {
        println!("{:?}", ddl_model().content[0].heading);
        println!("{:?}", ddl_model().content[0].content.len());
        let content = &ddl_model().content;
        assert!(content[0].content.len() > 5);
    }

    #[test]
    fn test_ddl_model_content() {
        let content = &ddl_model().content;
        dbg!(&ddl_model().heading);
        for block in &content.as_slice()[0..1] {
            block.content.iter().for_each(|i| {
                println!("{i}");
            });
        }
    }

    #[test]
    fn test_dict_model_content() {
        let content = &dict_model().content;
        dbg!(&dict_model().heading);
        for block in &content.as_slice()[0..1] {
            block.content.iter().for_each(|i| {
                println!("{i}");
            });
        }
    }

    // TODO: can have a round-trip test ddl00 -> model00 -> ddl01 -> model00 -> ddl02
    // test model00 print the same as model00 (test parsing)
    // test ddl01 the same as ddl02 (test cif writing)
}
