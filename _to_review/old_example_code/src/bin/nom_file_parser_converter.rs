#![allow(warnings)]
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

fn main() {
    let source = "alfa <b>bravo</b> - <b>charlie</b> delta";
    let expected = "alfa *bravo* - *charlie* delta";
    let (_, result) = convert(source).unwrap();
    assert_eq!(expected, result);

    let source = "echo <i>foxtrot</i> <b>golf</b> hotel <i>india</i>";
    let expected = "echo _foxtrot_ *golf* hotel _india_";
    let (_, result) = convert(source).unwrap();
    assert_eq!(expected, result);

    let source = "<i>juliett</i> <b>kilo</b> lima <i>mike</i>";
    let expected = "_juliett_ *kilo* lima _mike_";
    let (_, result) = convert(source).unwrap();
    assert_eq!(expected, result);
}

fn convert(source: &str) -> IResult<&str, String> {
    let (_, results) = many_till(grinder, eof)(source)?;
    Ok(("", results.0.join("")))
}

fn grinder(source: &str) -> IResult<&str, String> {
    let (active_string, preface) = alt((take_until("<"), rest))(source)?;
    let (next_up, completed) = alt((
        tuple((tag("<b>"), take_until("</b>"), tag("</b>"))).map(|x| format!("*{}*", &x.1)),
        tuple((tag("<i>"), take_until("</i>"), tag("</i>"))).map(|x| format!("_{}_", &x.1)),
        rest.map(|x: &str| x.to_string()),
    ))(active_string)?;
    Ok((next_up, format!("{}{}", preface, completed)))
}
