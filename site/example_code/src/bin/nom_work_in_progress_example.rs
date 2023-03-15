#![allow(warnings)]
use nom::branch::alt;
use nom::bytes::complete::tag;
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

#[derive(Debug)]
struct Page {
    text: String,
}

fn main() {
    let mut p = Page {
        text: "".to_string(),
    };

    // NOTE: The parsing requires a space in front
    // of and behind the semaphores. It doesn't work
    // as the first or last word of the file if there's
    // no padding.

    // TODO: Cover the end of line versions.

    // // TODO: Fix this (shouldn't turn into
    // // an `em`` because the closing `_`` isn't
    // // connected to the text
    // p.text = " _alfa _ ".to_string();
    // parse(&mut p);
    // assert_eq!(" _alfa _ ".to_string(), p.text);

    println!("Testing...");

    p.text = "alfa".to_string();
    parse(&mut p);
    assert_eq!("alfa".to_string(), p.text);

    p.text = " _alfa_ ".to_string();
    parse(&mut p);
    assert_eq!(" <<em|alfa>> ".to_string(), p.text);

    p.text = " _alfa bravo_ ".to_string();
    parse(&mut p);
    assert_eq!(" <<em|alfa bravo>> ".to_string(), p.text);

    p.text = " _alfa_ bravo _charlie_ ".to_string();
    parse(&mut p);
    assert_eq!(" <<em|alfa>> bravo <<em|charlie>> ".to_string(), p.text);

    p.text = "alfa _bravo_ charlie".to_string();
    parse(&mut p);
    assert_eq!("alfa <<em|bravo>> charlie".to_string(), p.text);

    p.text = "alfa _bravo\ncharlie_ delta".to_string();
    parse(&mut p);
    assert_eq!("alfa <<em|bravo\ncharlie>> delta".to_string(), p.text);

    p.text = "delta".to_string();
    parse(&mut p);
    assert_eq!("delta".to_string(), p.text);

    p.text = " *delta* ".to_string();
    parse(&mut p);
    assert_eq!(" <<strong|delta>> ".to_string(), p.text);

    p.text = " *delta echo* ".to_string();
    parse(&mut p);
    assert_eq!(" <<strong|delta echo>> ".to_string(), p.text);

    p.text = " *delta* echo *foxtrot* ".to_string();
    parse(&mut p);
    assert_eq!(
        " <<strong|delta>> echo <<strong|foxtrot>> ".to_string(),
        p.text
    );

    p.text = "delta *echo* foxtrot".to_string();
    parse(&mut p);
    assert_eq!("delta <<strong|echo>> foxtrot".to_string(), p.text);

    p.text = "delta *echo\nfoxtrot* golf".to_string();
    parse(&mut p);
    assert_eq!("delta <<strong|echo\nfoxtrot>> golf".to_string(), p.text);

    println!("Process complete");

    // let mut p = Page {
    //     text: "alfa _bravo_ *charlie* _delta_ *echo* foxtrot".to_string(),
    // };
    // parse(&mut p);
    // dbg!(p);
}

fn parse(p: &mut Page) {
    let (_, strongs) = many_till(preprocess_strongs, eof)(p.text.as_str()).unwrap();
    p.text = strongs.0.concat().concat();
    let (_, ems) = many_till(preprocess_ems, eof)(p.text.as_str()).unwrap();
    p.text = ems.0.concat().concat();
}

fn preprocess_ems(source: &str) -> IResult<&str, Vec<&str>> {
    let (source, value) = alt((
        pair(
            take_until(" _"),
            delimited(tag(" _"), take_until("_ "), tag("_ ")),
        ),
        tuple((rest, rest)),
    ))(source)?;
    if value.1.len() > 0 {
        Ok((source, vec![value.0, " <<em|", value.1, ">> "]))
    } else {
        Ok((source, vec![value.0]))
    }
}

fn preprocess_strongs(source: &str) -> IResult<&str, Vec<&str>> {
    let (source, value) = alt((
        pair(
            take_until(" *"),
            delimited(tag(" *"), take_until("*"), tag("* ")),
        ),
        tuple((rest, rest)),
    ))(source)?;
    if value.1.len() > 0 {
        Ok((source, vec![value.0, " <<strong|", value.1, ">> "]))
    } else {
        Ok((source, vec![value.0]))
    }
}

// #[derive(Debug)]
// struct Shuttle<'a> {
//     text: &'a str,
// }

// fn main() {
//     let mut shuttle = Shuttle {
//         text: r#"the _quick_ *brown* _fox_ *jumps* over"#,
//     };
//     {
//         let tmp_shuttle = &mut shuttle;
//         let (_, _) = parse(&mut tmp_shuttle).unwrap();
//     }
//     dbg!(&shuttle.text);
// }

// fn parse<'a>(shuttle: &'a mut Shuttle<'a>) -> IResult<&'a str, &'a str> {
//     let (_, _) = many_till(preprocess_strong, eof)(shuttle.text)?;
//     // shuttle.text = "asdf".to_string();
//     Ok(("", ""))
// }

// fn preprocess_strong(source: &str) -> IResult<&str, Vec<&str>> {
//     // dbg!(source);
//     let (source, value) = alt((
//         pair(
//             take_until("*"),
//             delimited(char('*'), take_until("*"), char('*')),
//         ),
//         tuple((rest, rest)),
//     ))(source)?;
//     // let mut output_value = String::from(value.0);
//     // if value.1 != "" {
//     //     output_value.push_str("<<strong|");
//     //     output_value.push_str(value.1);
//     //     output_value.push_str(">>");
//     // }
//     // Ok((source, output_value))
//     if value.1.len() > 0 {
//         Ok((source, vec![value.0, "<<strong|", value.1, ">>"]))
//     } else {
//         Ok((source, vec![value.0]))
//     }
// }

//fn parse(source: &str) -> IResult<&str, &str> {
//    let (_source, strong_texts) = many_till(preprocess_strong, eof)(source)?;
//    let tmp_vec = strong_texts.0.concat();
//    let tmp_string = tmp_vec.join("");
//    let tmp_str = tmp_string.as_str();
//    let (_source, em_texts) = many_till(preprocess_strong, eof)(tmp_str)?;
//    dbg!(em_texts);
//    // let new_source = strong_texts.0.concat().join("").to_string().as_str();
//    // dbg!(new_source);
//    // let next_source = texts.0.join("").as_str();
//    // let (source, texts2) = many_till(preprocess_strong, eof)(next_source)?;
//    //Ok((source, texts.0.join("")))
//    Ok(("", "asdf"))
//}

// fn preprocess_em(source: &str) -> IResult<&str, String> {
//     dbg!(source);
//     let (source, value) = alt((
//         pair(
//             take_until("_"),
//             delimited(char('_'), take_until("_"), char('_')),
//         ),
//         tuple((rest, rest)),
//     ))(source)?;
//     let mut output_value = String::from(value.0);
//     if value.1.len() > 0 {
//         output_value.push_str("<<strong|");
//         output_value.push_str(value.1);
//         output_value.push_str(">>");
//     }
//     Ok((source, output_value))
// }

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
