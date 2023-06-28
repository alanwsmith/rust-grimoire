#![allow(warnings)]
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::bytes::complete::take_till;
use nom::bytes::complete::take_until;
use nom::character::complete::alpha1;
use nom::character::complete::anychar;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::newline;
use nom::character::complete::not_line_ending;
use nom::character::complete::space0;
use nom::combinator::eof;
use nom::combinator::not;
use nom::combinator::opt;
use nom::combinator::peek;
use nom::combinator::rest;
use nom::multi::many0;
use nom::multi::many_till;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

// Work in progress

// // this one works at the baseline
// pub fn get_paragraphs(
//     source: &str,
// ) -> IResult<&str, (Vec<&str>, &str)> {
//     let count: usize = 1;
//     let (a, b) = many_till(
//         alt((
//             tuple((take_until("\n\n"), tag("\n\n")))
//                 .map(|x| x.0),
//             rest,
//         )),
//         eof,
//     )(source.trim())?;
//     dbg!(&a);
//     dbg!(&b);
//     Ok((a, b))
// }

// // this assemble one paragraph
// pub fn get_paragraphs(
//     source: &str,
// ) -> IResult<&str, String> {
//     let (a, b) = many_till(
//         anychar.map(|x| if x == '\n' { ' ' } else { x }),
//         tuple((tag("\n"), tag("\n"))).map(|x| ""),
//     )(source.trim())
//     .map(|(x, y)| (x, y.0.iter().collect()))?;
//     dbg!(&a);
//     dbg!(&b);
//     Ok((a, b))
// }

// this is basic working
//pub fn get_paragraphs(
//    source: &str,
//) -> IResult<&str, (Vec<(Vec<char>, &str)>, &str)> {
//    let (a, b) = many_till(
//        // tuple((line_ending, line_ending)),
//        many_till(
//            anychar
//                .map(|x| if x == '\n' { ' ' } else { x }),
//            alt((
//                tuple((tag("\n"), tag("\n"))).map(|x| ""),
//                eof,
//            )),
//        ),
//        alt((tag("\n\n"), eof)), // )(source.trim())
//    )(source.trim())?;
//    //.map(|(x, y)| (x, y.0.iter().collect()))?;
//    // .map(|(x, y)| (x, y.iter().collect()))?;
//    dbg!(&a);
//    dbg!(&b);
//    Ok((a, b))
//}

// // this collects the paragraphs
// pub fn get_paragraphs(
//     source: &str,
// ) -> IResult<&str, (Vec<(Vec<char>, &str)>, &str)> {
//     let (a, b) = many_till(
//         // tuple((line_ending, line_ending)),
//         many_till(
//             anychar
//                 .map(|x| if x == '\n' { ' ' } else { x }),
//             alt((
//                 tuple((tag("\n"), tag("\n"))).map(|x| ""),
//                 eof,
//             )),
//         )
//         .map(|(x, y)| {
//             dbg!(&x.iter().collect::<String>());
//             (x, y)
//         }),
//         alt((tag("\n\n"), eof)),
//     )(source.trim())?;
//     Ok((a, b))
// }

// this is working with most stuff, might
// be final, need to check.
pub fn get_paragraphs(
    source: &str,
) -> IResult<&str, Vec<String>> {
    let (a, b) = many_till(
        many_till(
            anychar
                .map(|x| if x == '\n' { ' ' } else { x }),
            alt((
                tuple((tag("\n"), tag("\n"))).map(|x| ""),
                eof,
            )),
        )
        .map(|(x, y)| {
            // dbg!(&x.iter().collect::<String>());
            (x.iter().collect::<String>(), y)
        }),
        alt((tag("\n\n"), eof)),
    )(source.trim())?;
    let results: Vec<_> =
        b.clone().0.into_iter().map(|x| x.0).collect();
    // dbg!(&results);
    Ok((a, results))
}

// pub fn get_paragraphs(
//     source: &str,
// ) -> IResult<&str, (Vec<&str>, &str)> {
//     let count: usize = 1;
//     let (a, b) = many_till(
//         alt((
//             tuple((take_until("\n\n"), tag("\n")))
//                 .map(|x| x.0),
//             rest,
//         )),
//         eof,
//     )(source.trim())?;
//     dbg!(&a);
//     dbg!(&b);
//     Ok((a, b))
// }

// pub fn get_paragraphs(
//     source: &str,
// ) -> IResult<&str, (&str, &str, &str)> {
//     let count: usize = 1;
//     let (a, b) =
//         tuple((tag("a"), space0, tag("b")))("a b".trim())?;
//     dbg!(&a);
//     dbg!(&b);
//     Ok((a, b))
// }

// pub fn get_paragraphs(
//     source: &str,
// ) -> IResult<&str, Vec<(&str, Option<&str>)>> {
//     let count: usize = 1;
//     let (a, b) = separated_list1(
//         tuple((is_a("\n"), is_a("\n"))),
//         tuple((is_not("\n"), opt(is_not("\n")))),
//     )(source.trim())?;
//     dbg!(&a);
//     dbg!(&b);
//     Ok((a, b))
// }

//pub fn get_paragraphs(
//    source: &str,
//) -> IResult<&str, Vec<&str>> {
//    let count: usize = 1;
//    let (a, b) = many0(alt((
//        tuple((is_not("\n"), take(count), newline)).map(|x| x.0),
//        //tuple((is_a("\n"), line_ending)).map(|x| x.0),
//        // tuple((is_not("\n"), line_ending)).map(|x| x.0),
//        // is_not("\n"),
//        is_not("x"),
//    )))(source.trim())?;
//    dbg!(&a);
//    dbg!(&b);
//    Ok((a, b))
//}

// pub fn get_paragraphs(
//     source: &str,
// ) -> IResult<&str, (Vec<&str>, &str)> {
//     let (a, b) = many_till(
//         tuple((not_line_ending, line_ending)).map(|x| x.0),
//         // not_line_ending,
//         // tuple((line_ending, line_ending)).map(|x| x.0),
//         line_ending,
//     )(source.trim())?;
//     dbg!(&a);
//     dbg!(&b);
//     Ok((a, b))
// }

// pub fn get_paragraphs(
//     source: &str,
// ) -> IResult<&str, Vec<&str>> {
//     let (a, b) = many0(
//         tuple((
//             not_line_ending,
//             line_ending,
//             peek(line_ending),
//         ))
//         .map(|x| x.0),
//     )(source.trim())?;
//     dbg!(&a);
//     dbg!(&b);
//     Ok((a, b))
// }

// pub fn get_paragraphs(
//     source: &str,
// ) -> IResult<&str, Vec<String>> {
//     let (a, b) = many0(
//         tuple((
//             many0(
//                 tuple((
//                     not_line_ending,
//                     line_ending,
//                     peek(not_line_ending),
//                 ))
//                 .map(|x| "".to_string()),
//             ),
//             not_line_ending,
//         ))
//         .map(|x| "".to_string()),
//     )(source.trim())?;
//     dbg!(&a);
//     dbg!(&b);
//     Ok((a, b))
// }

// pub fn get_paragraphs(
//     source: &str,
// ) -> IResult<&str, (Vec<(&str)>, &str)> {
//     let (a, b) = many_till(
//         not_line_ending, line_ending
//     )(
//         source.trim(),
//     )?;
//     dbg!(&a);
//     dbg!(&b);
//     Ok((a, b))
// }

// pub fn get_paragraphs(
//     source: &str,
// ) -> IResult<&str, (Vec<(String)>, &str)> {
//     let (a, b) = many_till(
//         alt((
//             tuple((not_line_ending, line_ending))
//                 .map(|x| "".to_string()),
//             rest.map(|x| "".to_string()),
//         )),
//         tuple((line_ending, space0, line_ending))
//             .map(|x| ""),
//     )(source.trim())?;
//     dbg!(&a);
//     dbg!(&b);
//     Ok((a, b))
// }

// pub fn get_paragraphs(
//     source: &str,
// ) -> IResult<&str, Vec<&str>> {
//     let (a, b) = separated_list1(
//         tuple((line_ending, space0, line_ending))
//             .map(|x| "".to_string()),
//         take_until("\n"), // tuple((
//                           //     not_line_ending,
//                           //     line_ending,
//                           //     space0,
//                           //     line_ending,
//                           // ))
//                           // .map(|x| "".to_string()),
//                           // rest, // many_till(
//                           //     not_line_ending,
//                           //     tuple((line_ending, space0, line_ending)),
//                           // )
//                           // .map(|x| "".to_string()),
//     )(source.trim())?;
//     dbg!(&a);
//     dbg!(&b);
//     Ok((a, b))
// }

// pub fn get_paragraphs(
//     source: &str,
// ) -> IResult<&str, Vec<&str>> {
//     let (a, b) = separated_list1(
//         tuple((line_ending, space0, line_ending)),
//         not(tuple((
//             line_ending,
//             space0,
//             line_ending,
//         ))), // take_until("\n"),
//     )(source.trim())?;
//     dbg!(&a);
//     dbg!(&b);
//     Ok((a, b))
// }

// pub fn get_paragraphs(
//     source: &str,
// ) -> IResult<&str, (Vec<&str>, &str)> {
//     let (a, b) = many_till(
//         alt((
//             tuple((
//                 take_until("\n"),
//                 tag("\n"),
//                 space0,
//                 line_ending,
//             ))
//             .map(|x| x.0),
//             rest,
//         )),
//         eof,
//     )(source.trim())?;
//     dbg!(&a);
//     dbg!(&b);
//     Ok((a, b))
// }

// -> p
//
// first thing is here
// with another line
//
// second and more
//
// -> todo
//
// [] this is
//
// [] asfdasdf
// asdfasdf
//
// [] asdfasdf
// asdf

// <p>thing is here with line</p>
// <p>and more</p>
// <ul>
// <li><input type="check" checked>this is</li>
// <li>asfdasdf asdfasdf</li>
// <li>asdfasdf asdf</li>
// </ul>

// pub fn get_paragraphs(
//     source: &str,
// ) -> IResult<&str, Option<Vec<&str>>> {
//     let (a, b) = separated_list1(
//         tag("\n\n"),
//         alt((take_until("\n\n"), rest)),
//     )(source)?;
//     dbg!(&b);
//     dbg!(&a);
//     Ok(("", Some(vec![])))
// }

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    pub fn single_line() {
        let source = vec!["Cap the jar"].join("\n");
        assert_eq!(
            get_paragraphs(source.as_str()).unwrap().1,
            vec!["Cap the jar"]
        );
    }

    #[test]
    pub fn multi_line() {
        let source =
            vec!["Heave the line", "", "Heavy black lines"]
                .join("\n");
        assert_eq!(
            get_paragraphs(source.as_str()).unwrap().1,
            vec!["Heave the line", "Heavy black lines"]
        );
    }

    #[test]
    pub fn three_lines() {
        let source = vec![
            "Drive the nail",
            "",
            "Open your book",
            "",
            "Tack it down",
        ]
        .join("\n");
        assert_eq!(
            get_paragraphs(source.as_str()).unwrap().1,
            vec![
                "Drive the nail",
                "Open your book",
                "Tack it down",
            ]
        );
    }

    #[test]
    pub fn leading_white_space() {
        let source = vec![
            "",
            "Shut the hatch",
            "",
            "Slide the catch",
            "",
            "Open the desk",
        ]
        .join("\n");
        assert_eq!(
            get_paragraphs(source.as_str()).unwrap().1,
            vec![
                "Shut the hatch",
                "Slide the catch",
                "Open the desk"
            ]
        );
    }

    #[test]
    pub fn trailing_white_space() {
        let source = vec![
            "The first scores",
            "",
            "Hang tinsel",
            "",
            "Heave the line",
            "",
            "The port side",
            "",
            "Help the weak",
            "",
        ]
        .join("\n");
        assert_eq!(
            get_paragraphs(source.as_str()).unwrap().1,
            vec![
                "The first scores",
                "Hang tinsel",
                "Heave the line",
                "The port side",
                "Help the weak",
            ]
        );
    }

    #[test]
    pub fn solo_multiple_trailing_white_space() {
        let source = vec![
            "",
            "",
            "",
            "Add salt",
            "Fasten two pins",
            "",
            "Fry the egg",
            "Fly by night",
            "Waste little time",
            "",
            "Set the lamp",
            "",
            "Beat the dust",
            "",
            "",
            "",
        ]
        .join("\n");
        assert_eq!(
        get_paragraphs(source.as_str()).unwrap().1,
             vec![
                 "Add salt Fasten two pins",
                 "Fry the egg Fly by night Waste little time",
                 "Set the lamp",
                 "Beat the dust"
             ]
         );
    }
}
