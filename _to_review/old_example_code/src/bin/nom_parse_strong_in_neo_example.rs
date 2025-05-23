// #![allow(warnings)]
use nom::branch::alt;
// use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::char;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::delimited;
use nom::sequence::pair;
// use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;
// use nom::Parser;

fn main() {
    let source = r#"the _quick_ brown _fox_ jumps"#;
    let (_, value) = parse(source).unwrap();
    dbg!(value);
}

fn parse(source: &str) -> IResult<&str, String> {
    let (source, texts) = many_till(parse_strong, eof)(source)?;
    Ok((source, texts.0.join("")))
}

fn parse_strong(source: &str) -> IResult<&str, String> {
    dbg!(source);
    let (source, value) = alt((
        pair(
            take_until("_"),
            delimited(char('_'), take_until("_"), char('_')),
        ),
        tuple((rest, rest)),
    ))(source)?;
    let mut output_value = String::from(value.0);
    if value.1 != "" {
        output_value.push_str("<<strong|");
        output_value.push_str(value.1);
        output_value.push_str(">>");
    }
    Ok((source, output_value))
}

// fn parse_strong(source: &str) -> IResult<&str, String> {
//     // dbg!("rrrrrrrrrrrrrrrrr");
//     dbg!(source);
//     let (source, value) = alt((
//         // take_until("_")
//         pair(
//             take_until("_"),
//             delimited(char('_'), take_until("_"), char('_')),
//         ), //
//         // Gotta be a better way to do this than `rest, rest`
//         // but it's working. Haven't figured out if it's
//         // possible to do this stuff in the .map style yet.
//         tuple((rest, rest)),
//     ))(source)?;
//     // dbg!("xxxxxxxxxxxxxxxx");
//     // dbg!(&source);
//     // dbg!(&value);
//     // Ok((source, value.1))
//     let mut output_value = String::from(value.0);
//     if value.1 != "" {
//         output_value.push_str("<<strong|");
//         output_value.push_str(value.1);
//         output_value.push_str(">>");
//     }
//     Ok((source, output_value))
// }

// fn parse_strong(source: &str) -> IResult<&str, String> {
//     dbg!("rrrrrrrrrrrrrrrrr");
//     dbg!(source);
//     let (source, value) = alt((
//         // take_until("_")
//         pair(
//             take_until("_"),
//             delimited(char('_'), take_until("_"), char('_')),
//         ), //
//         // Gotta be a better way to do this than `rest, rest`
//         // but it's working. Haven't figured out if it's
//         // possible to do this stuff in the .map style yet.
//         tuple((rest, rest)),
//     ))(source)?;
//     // dbg!("xxxxxxxxxxxxxxxx");
//     // dbg!(&source);
//     dbg!(&value);
//     // Ok((source, value.1))
//     let mut output_value = String::from(value.0);
//     output_value.push_str(value.1);
//     Ok((source, output_value))
// }

// fn parse_strong(source: &str) -> IResult<&str, &str> {
//     dbg!("rrrrrrrrrrrrrrrrr");
//     dbg!(source);
//     let (source, value) = alt((
//         // take_until("_")
//         pair(
//             take_until("_"),
//             delimited(char('_'), take_until("_"), char('_')),
//         ), //
//         // Gotta be a better way to do this than `rest, rest`
//         // but it's working
//         tuple((rest, rest)),
//     ))(source)?;
//     // dbg!("xxxxxxxxxxxxxxxx");
//     // dbg!(&source);
//     dbg!(&value);
//     // Ok((source, value.1))
//     Ok((source, value.1))
// }

// fn parse_strong(source: &str) -> IResult<&str, &str> {
//     let (_, value) = pair(
//         take_until("_"),
//         delimited(char('_'), take_until("_"), char('_')),
//     )(source)?;
//     Ok((source, value.1))
// }

// #[derive(Debug)]
// enum Content {
//     Text { raw_value: String },
//     Link { raw_value: String },
//     Placeholder,
// }

// fn main() {
//     let source = r#"the _quick_ *brown* _fox_ *jumps* over"#;
//     // let (_, parts) = parse(source).unwrap();
//     // dbg!(parts);
// }

// fn parse(source: &str) -> IResult<&str, Vec<String>> {
//     let (source, texts) = many_till(get_parts, eof)(source)?;
//     Ok((source, texts.0))
// }

// fn get_parts(source: &str) -> IResult<&str, String> {
//     let (source, value) = alt((preceded(take_until("_"), tag("_")), rest))(source)?;
//     dbg!(&source);
//     Ok((source, value.to_string()))
// }

// fn get_parts(source: &str) -> IResult<&str, Content> {
//     dbg!(&source);
//     let (source, value) = alt((
//         delimited(tag("<<"), take_until(">>"), tag(">>")).map(|s: &str| Content::Link {
//             raw_value: s.to_string(),
//         }),
//         take_until("<<").map(|s: &str| Content::Text {
//             raw_value: s.to_string(),
//         }),
//         rest.map(|s: &str| Content::Text {
//             raw_value: s.to_string(),
//         }),
//     ))(source)?;
//     Ok((source, value))
// }
