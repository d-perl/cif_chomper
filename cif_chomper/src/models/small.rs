use std::{fs::read_to_string, io, path::PathBuf};

use cif_chomper_core::{model::Model, parser::cif2_file};

use crate::models::unit_types::{Angstroms, Degrees};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseToModelError {
    #[error("File could not be read")]
    File(#[from] io::Error),
    #[error("File did not contain a valid CIF record")]
    Syntax(&'static str),
    #[error("File did not contain the minimum required information")]
    Content(&'static str),
}

pub struct SmallCell {
    pub a: Angstroms,
    pub b: Angstroms,
    pub c: Angstroms,
    pub alpha: Degrees,
    pub beta: Degrees,
    pub gamma: Degrees,
    pub space_group_number: u8,
}

/// The minimal basic information to specify an atom in a structure
pub struct SmallAtom {
    /// Atomic number
    species: u8,
    /// Fractional coordinates x, y, z
    pos: (f64, f64, f64),
}

/// A minimal model with only the basic elements: atoms, locations, cell info
pub struct SmallCifModel {
    pub cell: SmallCell,
    pub atoms: Vec<SmallAtom>,
}

impl TryFrom<Model<'_>> for SmallCifModel {
    fn try_from(value: Model) -> Result<Self, ParseToModelError> {
        Err(ParseToModelError::Content("Missing some fields"))
    }

    type Error = ParseToModelError;
}

impl SmallCifModel {
    pub fn from_file(path: &str) -> Result<Self, ParseToModelError> {
        let file_contents = read_to_string(path).map_err(|e| ParseToModelError::File(e))?;
        let model = cif2_file(&file_contents).map_err(|e| ParseToModelError::Syntax(e))?;
        Self::try_from(model)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::small::SmallCifModel;

    #[test]
    fn test_happy_path_load() {
        let model = SmallCifModel::from_file("../../example_data/mil-101.cif").unwrap();
    }
}
