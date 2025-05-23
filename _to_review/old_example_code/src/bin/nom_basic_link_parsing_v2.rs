#![allow(warnings)]
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

#[derive(Debug)]
enum Content {
    Text { raw_value: String },
    Link { raw_value: String },
    Placeholder,
}

fn main() {
    let source = r#"some <<link|example|https://www.example.com/>> link"#;
    let (_, parts) = parse(source).unwrap();
    dbg!(parts);
}

fn parse(source: &str) -> IResult<&str, Vec<Content>> {
    let (source, texts) = many_till(get_parts, eof)(source)?;
    Ok((source, texts.0))
}

fn get_parts(source: &str) -> IResult<&str, Content> {
    dbg!(&source);
    let (source, value) = alt((
        delimited(tag("<<"), take_until(">>"), tag(">>")).map(|s: &str| Content::Link {
            raw_value: s.to_string(),
        }),
        take_until("<<").map(|s: &str| Content::Text {
            raw_value: s.to_string(),
        }),
        rest.map(|s: &str| Content::Text {
            raw_value: s.to_string(),
        }),
    ))(source)?;
    Ok((source, value))
}

