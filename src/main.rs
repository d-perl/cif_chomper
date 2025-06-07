use log::{LevelFilter, debug};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_till},
    character::complete::{line_ending, not_line_ending},
    combinator::{eof, opt},
    multi::SeparatedList0,
    sequence::terminated,
};
use rstest::rstest;
use std::{any::Any, error::Error};
mod logging;

fn main() -> Result<(), Box<dyn Error>> {
    log::set_logger(&logging::LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Debug))
        .expect("Failed to set up logger!");

    Ok(())
}

fn eol_or_eof(input: &str) -> IResult<&str, &str> {
    alt((line_ending, eof)).parse(input)
}
fn parse_comment(input: &str) -> IResult<&str, &str> {
    let val = tag("#")(input)?;
    terminated(not_line_ending, eol_or_eof).parse(val.0)
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

res_word!(magic_code, r"#\#CIF_2.0");
res_word_nocase!(data_token, "data_");
res_word_nocase!(save_token, "save_");
res_word_nocase!(loop_token, "loop_");
res_word_nocase!(global_token, "global_");
res_word_nocase!(stop_token, "stop_");

#[rstest]
#[case(
    "#Asdiuybe9oniudbnfv   sieucvbn98\n",
    "Asdiuybe9oniudbnfv   sieucvbn98"
)]
#[case("#Asdiuybe9oniudbnfv   sieucvbn98", "Asdiuybe9oniudbnfv   sieucvbn98")]
fn test_parse_comment(#[case] input: &str, #[case] expected: &str) {
    let test = parse_comment(input);
    println!("############ {:?}", test);
    assert!(test.is_ok());
    assert_eq!(test.unwrap().1, expected);
}

#[rstest]
#[case("DaTa_asdh8907hiuh", "asdh8907hiuh")]
fn test_data_resword(#[case] input: &str, #[case] expected: &str) {
    let test = data_token(input);
    println!("############ {:?}", test);
    assert!(test.is_ok());
    assert_eq!(test.unwrap().0, expected);
}
