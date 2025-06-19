/// Corresponds to the hierarchy expressed by the CIF 2.0 syntax, without
/// any parsing of data
#[derive(Debug)]
pub struct RawModel<'a> {
    pub heading: &'a str,
    pub content: Vec<RawDataBlock<'a>>,
}

#[derive(Debug)]

pub struct RawDataBlock<'a> {
    pub heading: &'a str,
}
