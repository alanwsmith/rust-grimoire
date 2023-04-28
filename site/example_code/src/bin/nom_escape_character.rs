#![allow(warnings)]
use nom::branch::alt;
use nom::bytes::complete::escaped;
use nom::bytes::complete::escaped_transform;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_till;
use nom::bytes::complete::take_until;
use nom::character::complete::anychar;
use nom::character::complete::none_of;
use nom::character::complete::one_of;
use nom::combinator::eof;
use nom::combinator::not;
use nom::combinator::opt;
use nom::combinator::rest;
use nom::combinator::value;
use nom::multi::many0;
use nom::multi::many_till;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

fn main() {
    //test1();
    test2();
    dbg!("done");
}

// fn test1() {
//     let source = "Lift|the|stone";
//     let expected = vec!["Lift", "the", "stone"];
//     let result = parse_string(source, "|");
//     assert_eq!(expected, result.unwrap().1);
// }

fn test2() {
    let source = "Dip\\|the|pail";
    let expected = vec!["Dip|the", "pail"];
    let result = parse_string(source, "|");
    assert_eq!(expected, result.unwrap().1);
}

// fn parse_string<'a>(source: &'a str, separator: &'a str) -> IResult<&'a str, Vec<&'a str>> {
//     let (remainder, _) = opt(tag(separator))(source)?;
//     let (_, items) = separated_list0(tag(separator), is_not(separator))(remainder)?;
//     Ok(("", items))
// }

fn parse_string<'a>(source: &'a str, separator: &'a str) -> IResult<&'a str, Vec<&'a str>> {
    let (remainder, _) = opt(tag(separator))(source)?;
    // let (_, items) = separated_list0(tag(separator), is_not(separator))(remainder)?;
    // let (_, items) = take_till(tag("|"))(source)?;
    // let (_, items) = tuple((none_of("\\"), escaped(anychar, '\\', one_of("|"))))(source)?;
    // separated_list0(tuple((not(tag("\\")), tag(separator))), is_not(separator))(remainder)?;

    // let (_, items) = tuple((none_of("\\"), escaped(anychar, '\\', one_of("|"))))(source)?;
    // let (_, items) = escaped(none_of("|"), '\\', tag("|"))(source)?;

    // let (_, items) = many_till(
    //     alt((
    //         escaped_transform(none_of("\\|"), '\\', value("|", tag("|"))).map(|x| x.to_string()),
    //         tuple((take_until("|"), tag("|"))).map(|x: (&str, &str)| x.0.to_string()),
    //         rest.map(|x| "a".to_string()),
    //     )),
    //     eof,
    // )(source)?;

    let (_, items) = many_till(
        alt((
            tuple((
                escaped_transform(none_of("\\|"), '\\', value("|", tag("|"))),
                tag("|"),
            ))
            .map(|x| x.0.to_string()),
            tuple((take_until("|"), tag("|"))).map(|x: (&str, &str)| x.0.to_string()),
            rest.map(|x: &str| x.to_string()),
        )),
        eof,
    )(source)?;

    dbg!(items);

    // many_till(tuple((
    //     take_until("|")
    //)), eof)(source)?;

    Ok(("", vec![]))
}
