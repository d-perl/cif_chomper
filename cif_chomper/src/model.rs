/// Corresponds to the hierarchy expressed by the CIF 2.0 syntax, without
/// any parsing of data
#[derive(Debug, PartialEq)]
pub struct RawModel<'a> {
    pub heading: &'a str,
    pub content: Vec<RawDataBlock<'a>>,
}

#[derive(Debug, PartialEq)]

pub struct RawDataBlock<'a> {
    pub heading: &'a str,
    pub content: Vec<RawDataItem<'a>>,
}

#[derive(Debug, PartialEq)]
<<<<<<<< HEAD:cif_chomper_core/src/model.rs
========

pub enum RawDataItemContent<'a> {
    Empty,
    Str(&'a str),
    List(Vec<&'a str>),
    Table(Vec<(&'a str, &'a str)>),
}

#[derive(Debug)]
>>>>>>>> main:cif_chomper/src/model.rs

pub enum RawDataItem<'a> {
    SaveFrame(Vec<RawDataItem<'a>>),
    Data {
        name: &'a str,
        value: &'a str,
    },
    Loop {
        names: Vec<&'a str>,
        values: Vec<&'a str>,
    },
}
