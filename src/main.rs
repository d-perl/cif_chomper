use log::LevelFilter;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_till1},
    character::complete::{line_ending, not_line_ending, space0},
    combinator::eof,
    multi::many1,
    sequence::terminated,
};
use rstest::rstest;
use std::error::Error;
mod logging;

fn main() -> Result<(), Box<dyn Error>> {
    log::set_logger(&logging::LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Debug))
        .expect("Failed to set up logger!");

    Ok(())
}

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

fn eol_or_eof(input: &str) -> IResult<&str, &str> {
    alt((line_ending, eof)).parse(input)
}
fn text_delim(input: &str) -> IResult<&str, &str> {
    alt((eol_or_eof, tag(";"))).parse(input)
}
fn comment(input: &str) -> IResult<&str, &str> {
    let val = tag("#")(input)?;
    terminated(not_line_ending, eol_or_eof).parse(val.0)
}
fn comment_or_eol(input: &str) -> IResult<&str, &str> {
    alt((comment, eol_or_eof)).parse(input)
}
fn wspace_to_eol(input: &str) -> IResult<&str, &str> {
    let val = space0(input)?;
    comment_or_eol(val.0)
}
fn wspace_any(input: &str) -> IResult<&str, &str> {
    let val = many1(wspace_to_eol).parse(input)?;
    space0(val.0)
}
fn non_blank_chars(input: &str) -> IResult<&str, &str> {
    take_till1(|c| c == ' ' || c == '\t' || c == '\r' || c == '\n')(input)
}
res_word!(magic_code, r"#\#CIF_2.0");
res_word_nocase!(data_token, "data_");
res_word_nocase!(save_token, "save_");
res_word_nocase!(loop_token, "loop_");
res_word_nocase!(global_token, "global_");
res_word_nocase!(stop_token, "stop_");

#[rstest]
#[case(
    comment,
    "#Asdiuybe9oniudbnfv   sieucvbn98\n",
    "Asdiuybe9oniudbnfv   sieucvbn98"
)]
#[case(
    comment,
    "#Asdiuybe9oniudbnfv   sieucvbn98",
    "Asdiuybe9oniudbnfv   sieucvbn98"
)]
#[case(data_token, "DaTa_asdh8907hiuh", "asdh8907hiuh")]
#[case(non_blank_chars, "asdhb87^TG*(&^Gsd78a6g   ", "asdhb87^TG*(&^Gsd78a6g")]
#[case(non_blank_chars, "asdhb87^TG*(&^Gsd78a6g", "asdhb87^TG*(&^Gsd78a6g")]
#[case(non_blank_chars, "öµ\tyy7893h4", "öµ")]
fn test_parser_components(
    #[case] func: fn(&str) -> IResult<&str, &str>,
    #[case] input: &str,
    #[case] expected: &str,
) {
    let test = func(input);
    assert!(test.is_ok());
    let res = test.unwrap();
    println!("e: {:?} - f: {:?}", expected, res);
    assert!(res.0 == expected || res.1 == expected)
}
