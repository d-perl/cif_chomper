use crate::model::{RawDataBlock, RawDataItem, RawModel};
use const_str::to_char_array;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{is_not, tag, tag_no_case, take_until, take_while1},
    character::complete::{char, line_ending, not_line_ending, space0, space1},
    combinator::{eof, not, opt, peek},
    error::Error,
    multi::{many0, many1, separated_list1},
    sequence::terminated,
};

const NON_BLANK: &str = " \t\r\n";
fn non_blank(c: char) -> bool {
    !const { to_char_array!(NON_BLANK) }.contains(&c)
}
static RESTRICT: &str = " \t\r\n[]{}";
fn restrict_char(c: char) -> bool {
    non_blank(c) && !const { to_char_array!(RESTRICT) }.contains(&c)
}
static LEAD: &str = " \t\r\n[]{}\"#$\\_";


macro_rules! reserved_word {
    ($n:ident, $tag:literal, $fun:ident, $a:ident, $t:ty) => {
        fn $n($a: $t) -> IResult<$t, $t> {
            $fun($tag)($a)
        }
    };
}
macro_rules! res_word {
    ($n:ident, $tag:literal) => {
        reserved_word!($n, $tag, tag, input, &str);
    };
}
macro_rules! res_word_nocase {
    ($n:ident, $tag:literal) => {
        reserved_word!($n, $tag, tag_no_case, input, &str);
    };
}

fn text_delim(input: &str) -> IResult<&str, &str> {
    tag(";")(line_ending(input)?.0)
}
fn comment(input: &str) -> IResult<&str, &str> {
    let (inp, _) = tag("#")(input)?;
    terminated(not_line_ending, line_ending).parse(inp)
}
fn comment_or_eol(input: &str) -> IResult<&str, &str> {
    alt((comment, line_ending)).parse(input)
}
fn wspace_to_eol(input: &str) -> IResult<&str, &str> {
    let (inp, _) = space0(input)?;
    comment_or_eol(inp)
}
fn wspace_any(input: &str) -> IResult<&str, &str> {
    let (inp, _) = many0(wspace_to_eol).parse(input)?;
    space0(inp)
}
fn wspace_lines(input: &str) -> IResult<&str, &str> {
    let (inp, _) = opt(comment).parse(input)?;
    let (inp, _) = space0(inp)?;
    let (inp, _) = line_ending(inp)?;
    let (inp, _) = many0(wspace_to_eol).parse(inp)?;
    Ok((inp, ""))
}
fn wspace(input: &str) -> IResult<&str, &str> {
    let (inp, _) = alt((space1, line_ending)).parse(input)?;
    wspace_any(inp)
}
fn non_blank_chars(input: &str) -> IResult<&str, &str> {
    take_while1(non_blank).parse(input)
}
fn text_content(input: &str) -> IResult<&str, &str> {
    alt((take_until("\n;"), take_until("\r\n;"), take_until("\r;"))).parse(input)
}
fn text_field(input: &str) -> IResult<&str, &str> {
    let (inp, _) = text_delim(input)?;
    let (inp, txt) = text_content(inp)?;
    let (inp, _) = text_delim(inp)?;
    Ok((inp, txt))
}

res_word!(magic_code, r"#\#CIF_2.0");
res_word_nocase!(data_token, "data_");
res_word_nocase!(save_token, "save_");
res_word_nocase!(loop_token, "loop_");
res_word_nocase!(global_token, "global_");
res_word_nocase!(stop_token, "stop_");

res_word!(quote_3_delim, "\"\"\"");
res_word!(apostrophe_3_delim, "'''");

fn triple_dquote_string(input: &str) -> IResult<&str, &str> {
    let (inp, _) = quote_3_delim(input)?;
    let (inp, txt) = take_until("\"\"\"")(inp)?;
    let (inp, _) = quote_3_delim(inp)?;
    Ok((inp, txt))
}
fn triple_apo_string(input: &str) -> IResult<&str, &str> {
    let (inp, _) = apostrophe_3_delim(input)?;
    let (inp, txt) = take_until("'''")(inp)?;
    let (inp, _) = apostrophe_3_delim(inp)?;
    Ok((inp, txt))
}
fn triple_quoted_string(input: &str) -> IResult<&str, &str> {
    alt((triple_dquote_string, triple_apo_string)).parse(input)
}
fn single_squote_string(input: &str) -> IResult<&str, &str> {
    let (inp, _) = char('\'')(input)?;
    let (inp, value) = take_while1(|c| c != '\'').parse(inp)?;
    let (inp, _) = char('\'')(inp)?;
    Ok((inp, value))
}
fn single_dquote_string(input: &str) -> IResult<&str, &str> {
    let (inp, _) = char('"')(input)?;
    let (inp, value) = take_while1(|c| c != '"').parse(inp)?;
    let (inp, _) = char('"')(inp)?;
    Ok((inp, value))
}
fn single_quoted_string(input: &str) -> IResult<&str, &str> {
    alt((single_dquote_string, single_squote_string)).parse(input)
}
fn not_token(input: &str) -> IResult<&str, ()> {
    not(data_token).parse(input)?;
    not(save_token).parse(input)?;
    not(loop_token).parse(input)?;
    not(global_token).parse(input)?;
    not(stop_token).parse(input)
}
fn wsdelim_string(input: &str) -> IResult<&str, &str> {
    not_token(input)?;
    peek(is_not(LEAD)).parse(input)?;
    take_while1(restrict_char).parse(input)
}
fn wsdelim_string_sol(input: &str) -> IResult<&str, &str> {
    not_token(input)?;
    peek(is_not(LEAD)).parse(input)?;
    if peek(char::<&str, Error<&str>>(';')).parse(input).is_ok() {
        let (inp, _) = char(';')(input)?;
        space1(inp)
    } else {
        take_while1(restrict_char).parse(input)
    }
}
fn data_name(input: &str) -> IResult<&str, &str> {
    peek(char('_')).parse(input)?;
    let (inp, name) = non_blank_chars(input)?;
    Ok((inp, name))
}
fn list_values_start(input: &str) -> IResult<&str, &str> {
    let p1 = |inp| nospace_value(wspace_any(inp)?.0);
    let p2 = |inp| {
        let (inp_, _) = wspace_any(inp)?;
        let (inp_, _) = opt(comment).parse(inp_)?;
        text_field(inp_)
    };
    let p3 = |inp| {
        let (inp_, _) = wspace_any(inp)?;
        wsdelim_string(inp_)
    };
    let p4 = |inp| {
        let (inp_, _) = many1(wspace_to_eol).parse(inp)?;
        wsdelim_string_sol(inp_)
    };
    alt((p1, p2, p3, p4)).parse(input)
}
fn list(input: &str) -> IResult<&str, &str> {
    let (inp, _) = char('[')(input)?;
    let (inp, _) = opt(list_values_start).parse(inp)?;
    let (inp, _) = many0(wspace_data_value).parse(inp)?;
    let (inp, _) = wspace_any(space0(inp)?.0)?;
    let (inp, _) = char(']')(inp)?;
    Ok((inp, ""))
}
fn table(input: &str) -> IResult<&str, &str> {
    let wspace_tentry = |inp| table_entry(space0(inp)?.0);
    // TODO: replace with separated_list (0)
    let (inp, _) = char('{')(input)?;
    let (inp, _) = opt(wspace_any).parse(inp)?;
    let (inp, entry_1) = opt(table_entry).parse(inp)?;
    if entry_1.is_none() {
        let (inp, _) = char('}')(inp)?;
        return Ok((inp, ""));
    }
    let (inp, entries) = many0(wspace_tentry).parse(inp)?;
    let (inp, _) = space0(inp)?;
    let (inp, _) = char('}')(inp)?;
    Ok((inp, ""))
}
fn nospace_value(input: &str) -> IResult<&str, &str> {
    alt((single_quoted_string, triple_quoted_string, list, table)).parse(input)
}
fn wspace_dv_1(input: &str) -> IResult<&str, &str> {
    nospace_value(wspace(input)?.0)
}
fn wspace_dv_2(input: &str) -> IResult<&str, &str> {
    let (inp, _) = opt(wspace_lines).parse(input)?;
    let (inp, _) = space1(inp)?;
    wsdelim_string(inp)
}
fn wspace_dv_3(input: &str) -> IResult<&str, &str> {
    wsdelim_string_sol(wspace_lines(input)?.0)
}
fn wspace_dv_4(input: &str) -> IResult<&str, &str> {
    text_field(opt(comment).parse(opt(space0).parse(input)?.0)?.0)
}
fn wspace_data_value(input: &str) -> IResult<&str, &str> {
    alt((wspace_dv_1, wspace_dv_2, wspace_dv_3, wspace_dv_4)).parse(input)
}
fn table_entry(input: &str) -> IResult<&str, &str> {
    let (inp, key) = alt((single_quoted_string, triple_quoted_string)).parse(input)?;
    let (inp, _) = char(':')(inp)?;
    let (inp, value) = alt((nospace_value, wsdelim_string, wspace_data_value)).parse(inp)?;
    Ok((inp, "table entry"))
}
fn data_loop(input: &str) -> IResult<&str, RawDataItem> {
    let (inp, _) = loop_token(input)?;
    let (inp, _) = wspace(inp)?;
    let (inp, names) = separated_list1(wspace, data_name).parse(inp)?;
    //TODO: n values should be multiple of labels
    let (inp, values) = many1(wspace_data_value).parse(inp)?;
    Ok((inp, RawDataItem::Loop { names, values }))
}
fn data(input: &str) -> IResult<&str, RawDataItem> {
    let data_item = |inp| {
        let (inp_, name) = data_name(inp)?;
        let (inp_, value) = wspace_data_value(inp_)?;
        Ok((inp_, RawDataItem::Data { name, value }))
    };
    alt((data_item, data_loop)).parse(input)
}
fn container_code(input: &str) -> IResult<&str, &str> {
    take_while1(non_blank).parse(input)
}
fn frame_content(input: &str) -> IResult<&str, RawDataItem> {
    let (inp, _) = wspace(input)?;
    data(inp)
}
fn save_heading(input: &str) -> IResult<&str, &str> {
    let (inp, _) = save_token(input)?;
    container_code(inp)
}
fn save_frame(input: &str) -> IResult<&str, RawDataItem> {
    let (inp, _) = save_heading(input)?;
    let (inp, data) = many0(frame_content).parse(inp)?;
    let (inp, _) = wspace(inp)?;
    let (inp, _) = save_token(inp)?;
    Ok((inp, RawDataItem::SaveFrame(data)))
}
fn block_content(input: &str) -> IResult<&str, RawDataItem> {
    let (inp, _) = wspace(input)?;
    let (inp, cont) = alt((data, save_frame)).parse(inp)?;
    Ok((inp, cont))
}
fn data_heading(input: &str) -> IResult<&str, &str> {
    let (inp, _) = data_token(input)?;
    container_code(inp)
}
fn data_block(input: &str) -> IResult<&str, RawDataBlock> {
    let (inp, heading) = data_heading(input)?;
    let (inp, content) = many0(block_content).parse(inp)?;
    Ok((inp, RawDataBlock { heading, content }))
}
fn file_heading(input: &str) -> IResult<&str, &str> {
    let (inp, _) = opt(char('\u{FEFF}')).parse(input)?;
    let (inp, code) = magic_code(inp)?;
    let (inp, _) = space0(inp)?;
    Ok((inp, code))
}
fn file_content(input: &str) -> IResult<&str, Vec<RawDataBlock>> {
    let (inp, _) = line_ending(input)?;
    let (inp, _) = wspace_any(inp)?;
    let (inp, blocks) = separated_list1(wspace, data_block).parse(inp)?;
    Ok((inp, blocks))
}
pub fn cif2_file(input: &str) -> Result<RawModel, &str> {
    let (inp, heading) = file_heading(input).map_err(|_| "heading")?;
    let (inp, content) = file_content(inp).map_err(|_| "content")?;
    let (inp, _) = wspace_any(inp).map_err(|_| "trailing wspace")?;
    let (inp, _) = opt(comment)
        .parse(inp)
        .map_err(|_| "trailing opt comment")?;
    eof::<&str, ()>(inp).map_err(|_| "expected eof")?;
    Ok(RawModel { heading, content })
}

mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        comment,
        "#Asdiuybe9oniudbnfv   sieucvbn98\n",
        "Asdiuybe9oniudbnfv   sieucvbn98",
        true
    )]
    #[case(data_token, "DaTa_asdh8907hiuh", "asdh8907hiuh", true)]
    #[case(
        non_blank_chars,
        "asdhb87^TG*(&^Gsd78a6g   ",
        "asdhb87^TG*(&^Gsd78a6g",
        true
    )]
    #[case(
        non_blank_chars,
        "asdhb87^TG*(&^Gsd78a6g",
        "asdhb87^TG*(&^Gsd78a6g",
        true
    )]
    #[case(non_blank_chars, "öµ\tyy7893h4", "öµ", true)]
    #[case(non_blank_chars, "  ashd87", "", false)]
    #[case(text_delim, "\n;abc", "abc", true)]
    #[case(text_delim, "\n;abc123\nxyz987\n;abc", "abc123\nxyz987\n;abc", true)]
    #[case(text_content, "abc123\nxyz987\n;abc", "\n;abc", true)]
    #[case(text_field, "\n;abc123\nxyz987\n;abc", "abc", true)]
    #[case(triple_quoted_string, "\"\"\"asdf  7' \n\t \"\"\"abc", "abc", true)]
    #[case(triple_quoted_string, "\"\"\"asdf  7' \n\t \"\"a\"abc", "", false)]
    #[case(triple_quoted_string, "'''asdf  7' \n\t '''abc", "abc", true)]
    #[case(data_name, "_cif_field_item qwe rty", "_cif_field_item", true)]
    #[case(data_name, "cif_field_item qwe rty", "", false)]
    #[case(nospace_value, "'Lebedev, O. I.'\n", "Lebedev, O. I.", true)]
    #[case(single_quoted_string, "'Lebedev, O. I.'\n", "Lebedev, O. I.", true)]
    fn test_parser_components(
        #[case] func: fn(&str) -> IResult<&str, &str>,
        #[case] input: &str,
        #[case] expected: &str,
        #[case] good: bool,
    ) {
        let test = func(input);
        dbg!(&test);
        if good {
            assert!(test.is_ok());
            let res = test.unwrap();
            println!("e: {:?} - f: {:?}", expected, res);
            assert!(res.0 == expected || res.1 == expected)
        } else {
            assert!(test.is_err())
        }
    }

    #[rstest]
    #[case(
        "loop_
_symmetry_equiv_pos_as_xyz
x,y,z
x,-y+1/4,-z+1/4
-x+1/4,y,-z+1/4
-x,-z+1/2,-y+1/2
-x,z+1/4,y+1/4
x+3/4,z+1/4,-y+1/2
x+3/4,-z+1/2,y+1/4
loop_
_atom_site_label,",
        RawDataItem::Loop{ 
            names: vec!["_symmetry_equiv_pos_as_xyz"], 
            values: vec!["x,y,z","x,-y+1/4,-z+1/4","-x+1/4,y,-z+1/4","-x,-z+1/2,-y+1/2","-x,z+1/4,y+1/4","x+3/4,z+1/4,-y+1/2","x+3/4,-z+1/2,y+1/4",]
        },
        true
    )]
    fn test_parser_loop(#[case] input: &str, #[case] expected: RawDataItem, #[case] good: bool) {
        let test = data_loop(input);
        dbg!(&test);
        if good {
            assert!(test.is_ok());
            let res = test.unwrap();
            println!("e: {:?} - f: {:?}", expected, res);
            assert!(res.1 == expected)
        } else {
            assert!(test.is_err())
        }
    }

    #[rstest]
    #[case(
        "\n'Lebedev, O. I.'\n'Millange, F.'\n'Serre, C.'\n'Van Tendeloo, G.'\n'F\\'erey, G.'\n_publ_section_title\n;\nFirst direct imaging of giant pores of the Metal-Organic Framework MIL-101\n;\n_journal_name_full               'Chemistry of Materials'\n_journal_paper_doi               10.1021/cm051870o\n_journal_year                    2005\n_chemical_formula_sum            'C H Cr O'\n_space_group_IT_number           227\n_symmetry_cell_setting           cubic\n_symmetry_space_group_name_Hall  '-F 4vw 2vw 3'\n_symmetry_space_group_name_H-M   'F d -3 m :2'\"",
        "Lebedev, O. I.",
        true
    )]
    #[case("\n#'F\\'erey, G.'\n_publ_section_title abcde", "", false)]
    #[case(
        "
x,y,z
x,-y+1/4,-z+1/4
-x+1/4,y,-z+1/4
-x,-z+1/2,-y+1/2",
        "x,y,z",
        true
    )]
    fn test_wspace_data_value(#[case] input: &str, #[case] expected: &str, #[case] good: bool) {
        let test = wspace_data_value(input);
        dbg!(&test);
        if good {
            assert!(test.is_ok());
            let res = test.unwrap();
            println!("e: {:?} - f: {:?}", expected, res);
            assert!(res.0 == expected || res.1 == expected)
        } else {
            assert!(wspace_dv_1(input).is_err());
            assert!(wspace_dv_2(input).is_err());
            assert!(wspace_dv_3(input).is_err());
            assert!(wspace_dv_4(input).is_err());
        }
    }

    #[rstest]
    #[case("\n#'F\\'erey, G.'\n_publ_section_title abcde", "", false)]
    #[case("\nx,y,z\nx,-y+1/4,-z+1/4", "\nx,-y+1/4,-z+1/4", true)]
    fn test_wspace_delim_sol(#[case] input: &str, #[case] expected: &str, #[case] good: bool) {
        if good {
            let (inp, _) = wspace_lines(input).unwrap();
            let (inp, val) = wsdelim_string_sol(inp).unwrap();
        } else {
            let (inp, _) = wspace_lines(input).unwrap();
            let t = peek(is_not::<&str, &str, Error<&str>>(LEAD)).parse(inp);
            assert!(wsdelim_string_sol(inp).is_err());
        }
    }
}
