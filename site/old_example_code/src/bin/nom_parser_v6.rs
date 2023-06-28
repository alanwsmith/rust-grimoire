use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::error::Error;
use nom::multi::many_till;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

fn main() {
    let (a, b) = parse("\n#+TITLE: quick \n#+DATE: 2023-03-26\n #+TITLE: fox").unwrap();
    dbg!(&a);
    dbg!(&b);
}

fn parse(source: &str) -> IResult<&str, (Vec<String>, &str)> {
    let (c, d) = many_till(part, eof)(source)?;
    Ok((c, d))
}

fn part(source: &str) -> IResult<&str, String> {
    let (e, f) = alt((
        preceded(
            tag::<&str, &str, Error<&str>>("\n"),
            tuple((tag("#+TITLE: "), not_line_ending)),
        )
        .map(|t| format!("<title>{}</title>", t.1.trim())),
        preceded(
            tag::<&str, &str, Error<&str>>("\n"),
            tuple((tag("#+DATE: "), not_line_ending)),
        )
        .map(|t| format!("<date>{}</date>", t.1.trim())),
        take(1u32).map(|t: &str| t.to_string()),
    ))(source)?;
    Ok((e, f))
}
