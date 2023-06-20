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
use nom::sequence::terminated;
// use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;
// use nom::Parser;
use nom::bytes::complete::take_till;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::sequence::preceded;
use nom::Err;

// TODO: combine this with the get_newlines_after_tag
// or whatever it's called

fn main() {
    let source = "alfa\n\n->\n\nbravo";
    let expected = Ok(("bravo", "alfa"));
    let result = parse(source);
    assert_eq!(expected, result);
}

fn parse(source: &str) -> IResult<&str, &str> {
    //jlet (source, value) = take_until(terminated(newline_pair, tag("-> ")))(source)?;
    // let (source, value) = tag(newline_pair)(source)?;
    let (source, value) = take_until(tag("\n"))(source)?;

    Ok((source, value))
}

fn newline_pair(source: &str) -> IResult<&str, &str> {
    let (source, value) = space0(source)?;
    let (source, value) = line_ending(source)?;
    let (source, value) = space0(source)?;
    let (source, value) = line_ending(source)?;
    Ok((source, value))
}

//fn do_split(source: &str) -> IResult<&str, RawBlock> {
//    //dbg!(source);
//    let (source, _) = multispace0(source)?;
//    // dbg!(source);
//    let (source, _) = tag("-> ")(source)?;
//    let (source, value) = tuple((
//        alt((tag("TITLE"), tag("CODE"), tag("P"))),
//        newline_pair,
//        // tag("\n\n"),
//        alt((take_until("\n\n-> "), rest)), // alt((
//                                            //     pair(take_until("\n\n"), tag("-> ")),
//                                            //     pair(rest, rest), // jtake_until("\n\n-> "),
//                                            //                       // rest,
//                                            // )),
//    ))(source)?;
//    // let response = match value.0 {
//    //     "TITLE" => RawBlock::Title {
//    //         text: value.2.to_string(),
//    //     },
//    //     "P" => RawBlock::P {
//    //         text: value.2.to_string(),
//    //     },
//    //     "CODE" => RawBlock::Code {
//    //         text: value.2.to_string(),
//    //     },
//    //     _ => RawBlock::Error {
//    //         text: value.2.to_string(),
//    //     },
//    // };
//    // Ok((source, response))
//}
