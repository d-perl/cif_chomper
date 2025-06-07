use log::LevelFilter;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_till1, take_until, take_while, take_while1},
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

fn non_blank(c: char) -> bool {
    ![' ', '\t', '\r', '\n'].contains(&c)
}
fn restrict_char(c: char) -> bool {
    non_blank(c) && !['[', ']', '{', '}'].contains(&c)
}
fn lead_char(c: char) -> bool {
    restrict_char(c) && !['"', '#', '$', '\'', '_'].contains(&c)
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
    tag(";")(line_ending(input)?.0)
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
    take_while1(non_blank)(input)
}
fn text_content(input: &str) -> IResult<&str, &str> {
    alt((take_until("\n;"), take_until("\r\n;"), take_until("\r;"))).parse(input)
}
fn text_field(input: &str) -> IResult<&str, &str> {
    text_delim(text_content(text_delim(input)?.0)?.0)
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
    "Asdiuybe9oniudbnfv   sieucvbn98",
    true
)]
#[case(
    comment,
    "#Asdiuybe9oniudbnfv   sieucvbn98",
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
