use nom::branch::alt;
use nom::bytes::complete::escaped_transform;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::none_of;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::combinator::value;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

fn main() {
    test1();
    test2();
    println!("done");
}

fn test1() {
    let source = "Lift|the|stone|up|high";
    let expected = vec!["Lift", "the", "stone", "up", "high"];
    let result = split_on_separator_with_escapes(source, "|");
    assert_eq!(expected, result.unwrap().1);
}

fn test2() {
    let source = "Dip\\|the|pail|in\\|the|water";
    let expected = vec!["Dip|the", "pail", "in|the", "water"];
    let result = split_on_separator_with_escapes(source, "|");
    assert_eq!(expected, result.unwrap().1);
}

fn split_on_separator_with_escapes<'a>(
    source: &'a str,
    separator: &'a str,
) -> IResult<&'a str, Vec<String>> {
    let mut separator_with_escape = String::from("\\");
    separator_with_escape.push_str(separator);
    let (_, items) = many_till(
        alt((
            tuple((
                escaped_transform(
                    none_of(separator_with_escape.as_str()),
                    '\\',
                    value(separator, tag(separator)),
                ),
                tag(separator),
            ))
            .map(|x| x.0.to_string()),
            tuple((take_until(separator), tag(separator))).map(|x: (&str, &str)| x.0.to_string()),
            rest.map(|x: &str| x.to_string()),
        )),
        eof,
    )(source)?;
    Ok(("", items.0))
}
