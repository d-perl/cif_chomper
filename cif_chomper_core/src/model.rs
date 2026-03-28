/* Corresponds to the hierarchy expressed by the CIF 2.0 / `DDLm` syntax, without any parsing or interpretation of data

```ext-EBNF
block = block-heading, { block-content } ;
block-content = wspace, ( data | save-frame ) ;
save-frame = save-heading, { frame-content }, wspace, save-token ;
save-heading = save-token, container-code;
frame-content = wspace, data ;
container-code = non-blank-char, { non-blank-char } ;
data = ( data-name, wspace-data-value ) | data-loop ;
data-loop = loop-token, wspace, data-name, { wspace, data-name },
  wspace-data-value, { wspace-data-value } ;

wspace-data-value =
    (   wspace, nospace-value )
  | ( [ wspace-lines ], inline-wspace, { inline-wspace }, wsdelim-string )
  | (   wspace-lines, wsdelim-string-sol )
  | ( [ wspace ], [ comment ], text-field ) ;

nospace-value =
    quoted-string
  | triple-quoted-string
  | list
  | table ;
```
*/

/// Full model parsed vessel
#[derive(Debug, PartialEq)]
pub struct Model<'a> {
    pub heading: &'a str,
    pub content: Vec<Block<'a>>,
}

impl std::fmt::Display for Model<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Model - {}", self.heading)?;
        for (i, b) in self.content.iter().enumerate() {
            writeln!(f, "--- < block {i} ---")?;
            write!(f, "{b}")?;
            writeln!(f, "--- block {i} > ---")?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct Block<'a> {
    pub heading: &'a str,
    pub content: Vec<BlockItem<'a>>,
}

impl std::fmt::Display for Block<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Block - {}", self.heading)?;
        for (i, bi) in self.content.iter().enumerate() {
            writeln!(f, "- < item {i} -")?;
            write!(f, "{bi}")?;
            writeln!(f, "- item {i} > -")?;
        }
        Ok(())
    }
}

impl Block<'_> {
    pub fn find_single_value(&'_ self, name: &'_ str) -> Option<DataValue<'_>> {
        for item in self.content.iter() {
            match item {
                BlockItem::SaveFrame { heading, content } => todo!(),
                BlockItem::Data(data_item) => todo!(),
            }
        }
        None
    }
}

#[derive(Debug, PartialEq)]
pub enum BlockItem<'a> {
    SaveFrame {
        heading: &'a str,
        content: Vec<DataItem<'a>>,
    },
    Data(DataItem<'a>),
}

impl std::fmt::Display for BlockItem<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockItem::SaveFrame { heading, content } => {
                writeln!(f, "SaveFrame: {heading}")?;
                for d in content {
                    writeln!(f, "\t{d}")?;
                }
                writeln!(f, "End SaveFrame {heading}")?;
                Ok(())
            }
            BlockItem::Data(data_item) => write!(f, "{data_item}"),
        }
    }
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

impl std::fmt::Display for DataItem<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataItem::Data { name, value } => {
                writeln!(f, "Data:")?;
                writeln!(f, "\t{name}: \t{value}")?;
                Ok(())
            }
            DataItem::DataLoop { names, values } => {
                writeln!(f, "DataLoop:")?;
                writeln!(f, "\t{}", names.join(", "))?;
                let n_cols = names.len();
                for (i, value) in values.iter().enumerate() {
                    if (i + 1) % n_cols == 0 {
                        writeln!(f, "{value};")?;
                    } else {
                        write!(f, "{value}, ")?;
                    }
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DataValue<'a> {
    Empty,
    Str(&'a str),
    List(Vec<DataValue<'a>>),
    Table(Vec<(&'a str, DataValue<'a>)>),
}

impl std::fmt::Display for DataValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataValue::Empty => write!(f, "<Empty>"),
            DataValue::Str(s) => write!(f, "<str \"{s}\">"),
            DataValue::List(data_values) => {
                let vs = data_values
                    .iter()
                    .map(|v| format!("{v}"))
                    .collect::<Vec<_>>();
                write!(f, "[{}]", vs.join(", "))
            }
            DataValue::Table(items) => {
                let vt = items
                    .iter()
                    .map(|(k, v)| format!("{k}: {v}"))
                    .collect::<Vec<_>>();
                write!(f, "{{ {} }}", vt.join("; "))
            }
        }
    }
}
