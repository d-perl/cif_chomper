pub mod vessel {
    include!("./__cif_vessel.rs");
}

mod _vessel_be_like;

use vessel::Cell;

use std::cell::LazyCell;

use cif_chomper_core::parser::cif2_file;
use cif_chomper_core::model::{Block, BlockItem, DataItem, DataValue, Model};

const DDL: &str = include_str!("../../cif_core/ddl.dic");
const DDL_MODEL: LazyCell<Model> = LazyCell::new(|| cif2_file(DDL).unwrap());
const DICT: &str = include_str!("../../cif_core/cif_core.dic");
const DICT_MODEL: LazyCell<Model> = LazyCell::new(|| cif2_file(DICT).unwrap());

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

fn match_block_item(item: &BlockItem) {
    match &item {
        BlockItem::Data(data) => match_data_item(data),
        BlockItem::SaveFrame { heading, content } => {
            println!("\n SAVE FRAME {heading} \n");
            content.iter().for_each(match_data_item);
        }
        _ => (),
    }
}

fn match_data_item(item: &DataItem) {}

#[test]
fn test_load_ddl_str() {
    assert!(DDL.len() > 100);
}

#[test]
fn test_load_ddl_model() {
    println!("{:?}", DDL_MODEL.content[0].heading);
    println!("{:?}", DDL_MODEL.content[0].content.len());
    let content = &DDL_MODEL.content;
    assert!(content[0].content.len() > 5);
}

#[test]
fn test_ddl_model_content() {
    let content = &DDL_MODEL.content;
    dbg!(&DDL_MODEL.heading);
    for block in content.as_slice()[0..1].iter() {
        block.content.iter().for_each(match_block_item);
    }
}

#[test]
fn test_dict_model_content() {
    let content = &DICT_MODEL.content;
    dbg!(&DICT_MODEL.heading);
    for block in content.as_slice()[0..1].iter() {
        block.content.iter().for_each(match_block_item);
    }
}
