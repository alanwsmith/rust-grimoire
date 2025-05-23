use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::combinator::rest;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

fn main() {
    let source = r#"alfa
bravo #findme charlie
delta"#;
    let value = parse(source, "#findme").unwrap().1;
    dbg!(value);
}

fn parse<'a>(source: &'a str, target: &'a str) -> IResult<&'a str, Option<String>> {
    let (remainder, captured) = alt((
        tuple((
            take_until(target),
            tag(target),
            multispace1,
            not_line_ending,
        ))
        .map(|x| x.3),
        rest,
    ))(source)?;
    if remainder.is_empty() {
        Ok(("", None))
    } else {
        Ok(("", Some(captured.to_string())))
    }
}
