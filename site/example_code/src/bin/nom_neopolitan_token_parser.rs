#![allow(warnings)]
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::char;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::delimited;
use nom::sequence::pair;
// use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;
// use nom::Parser;

#[derive(Debug, PartialEq)]
enum RawBlock {
    Title { text: String },
    P { text: String },
    Error { text: String },
}

fn main() {
    println!("Testing...");
    let source = "-> TITLE\n\na title\n\n-> P\n\nsome\n\nparagraphs";
    let expected: Vec<RawBlock> = vec![
        RawBlock::Title {
            text: "a title".to_string(),
        },
        RawBlock::P {
            text: "some\n\nparagraphs".to_string(),
        },
    ];
    assert_eq!(expected, split_tokens(source));

    println!("Process complete");
}

fn split_tokens(source: &str) -> Vec<RawBlock> {
    let (_, tokens) = many_till(do_split, eof)(source).unwrap();
    tokens.0
}

fn do_split(source: &str) -> IResult<&str, RawBlock> {
    //dbg!(source);
    let (source, _) = multispace0(source)?;
    // dbg!(source);
    let (source, _) = tag("-> ")(source)?;
    let (source, value) = tuple((
        alt((tag("TITLE"), tag("P"))),
        tag("\n\n"),
        alt((take_until("\n\n-> "), rest)),
    ))(source)?;

    let response = match value.0 {
        "TITLE" => RawBlock::Title {
            text: value.2.to_string(),
        },
        "P" => RawBlock::P {
            text: value.2.to_string(),
        },
        _ => RawBlock::Error {
            text: value.2.to_string(),
        },
    };
    Ok((source, response))
}
