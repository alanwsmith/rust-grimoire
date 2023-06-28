use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::delimited;
use nom::IResult;

fn main() {
    let source = r#"some <<link|example|https://www.example.com/>> link"#;
    let (_, parts) = parse(source).unwrap();
    dbg!(parts);
}

fn parse(source: &str) -> IResult<&str, Vec<String>> {
    let (source, texts) = many_till(get_parts, eof)(source)?;
    Ok((source, texts.0))
}

fn get_parts(source: &str) -> IResult<&str, String> {
    dbg!(&source);
    let (source, value) = alt((
        delimited(tag("<<"), take_until(">>"), tag(">>")),
        take_until("<<"),
        rest,
    ))(source)?;
    Ok((source, value.to_string()))
}
