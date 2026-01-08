/// Corresponds to the hierarchy expressed by the CIF 2.0 / `DDLm` syntax, without
/// any parsing or interpretation of data
#[derive(Debug, PartialEq)]
pub struct Model<'a> {
    pub heading: &'a str,
    pub content: Vec<Block<'a>>,
}

#[derive(Debug, PartialEq)]

pub struct Block<'a> {
    pub heading: &'a str,
    pub content: Vec<BlockItem<'a>>,
}

// Sync layout with grammar def
//
// ```cif
// block = block-heading, { block-content } ;
// block-content = wspace, ( data | save-frame ) ;
// save-frame = save-heading, { frame-content }, wspace, save-token ;
// save-heading = save-token, container-code;
// frame-content = wspace, data ;
// container-code = non-blank-char, { non-blank-char } ;
// data = ( data-name, wspace-data-value ) | data-loop ;
// data-loop = loop-token, wspace, data-name, { wspace, data-name },
//   wspace-data-value, { wspace-data-value } ;
//
// wspace-data-value =
//     (   wspace, nospace-value )
//   | ( [ wspace-lines ], inline-wspace, { inline-wspace }, wsdelim-string )
//   | (   wspace-lines, wsdelim-string-sol )
//   | ( [ wspace ], [ comment ], text-field ) ;
//
// nospace-value =
//     quoted-string
//   | triple-quoted-string
//   | list
//   | table ;
// ```
//
#[derive(Debug, PartialEq)]
pub enum BlockItem<'a> {
    SaveFrame {
        heading: &'a str,
        content: Vec<DataItem<'a>>,
    },
    Data(DataItem<'a>),
}

#[derive(Debug, PartialEq)]
pub enum DataItem<'a> {
    Data {
        name: &'a str,
        value: DataValue<'a>,
    },
    DataLoop {
        names: Vec<&'a str>,
        values: Vec<DataValue<'a>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum DataValue<'a> {
    Empty,
    Str(&'a str),
    List(Vec<DataValue<'a>>),
    Table(Vec<(&'a str, DataValue<'a>)>),
}
