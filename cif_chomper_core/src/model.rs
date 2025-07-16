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
pub enum RawDataItemContent<'a> {
    Empty,
    Str(&'a str),
    List(Vec<RawDataItemContent<'a>>),
    Table(Vec<(RawDataItemContent<'a>, RawDataItemContent<'a>)>),
}

#[derive(Debug, PartialEq)]
pub enum RawDataItem<'a> {
    SaveFrame(Vec<RawDataItem<'a>>),
    Data {
        name: &'a str,
        value: RawDataItemContent<'a>,
    },
    Loop {
        names: Vec<&'a str>,
        values: Vec<RawDataItemContent<'a>>,
    },
}
