/// Corresponds to the hierarchy expressed by the CIF 2.0 / DDLm syntax, without
/// any parsing or interpretation of data
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

// XXX: not fully sync with grammar def, which is:
//
// ```cif
// block-content = wspace, ( data | save-frame ) ;
// save-frame = save-heading, { frame-content }, wspace, save-token ;
// save-heading = save-token, container-code;
// frame-content = wspace, data ;
// container-code = non-blank-char, { non-blank-char } ;
// data = ( data-name, wspace-data-value ) | data-loop ;
// data-loop = loop-token, wspace, data-name, { wspace, data-name },
//   wspace-data-value, { wspace-data-value } ;
// ```
//

#[derive(Debug, PartialEq)]
pub enum RawDataItem<'a> {
    SaveFrame {
        name: &'a str,
        content: Vec<RawDataItem<'a>>,
    },
    Data {
        name: &'a str,
        value: RawDataItemContent<'a>,
    },
    Loop {
        names: Vec<&'a str>,
        values: Vec<RawDataItemContent<'a>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum RawDataItemContent<'a> {
    Empty,
    Str(&'a str),
    List(Vec<RawDataItemContent<'a>>),
    Table(Vec<(RawDataItemContent<'a>, RawDataItemContent<'a>)>),
}

