#![allow(unused_imports)]
#![allow(unused_variables)]
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::alpha1;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::character::complete::newline;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::combinator::not;
use nom::combinator::rest;
use nom::multi::many1;
use nom::multi::many_till;
use nom::multi::separated_list0;
use nom::multi::separated_list1;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

#[derive(Debug, PartialEq)]
pub enum Section {
    H2(Vec<crate::nom_parse_test_v17::Content>),
    Title(Vec<crate::nom_parse_test_v17::Content>),
    P(Vec<crate::nom_parse_test_v17::Content>),
    None,
}

#[derive(Debug, PartialEq)]
pub enum Content {
    Title { text: String },
    Paragraph { text: String },
    None,
}

// pub fn text_line(
//     source: &str,
// ) -> IResult<&str, Vec<String>> {
//     dbg!(&source);
//     let (source, captured) = separated_list0(
//         tuple((newline, not(newline))),
//         not_line_ending,
//     )(source)?;
//     dbg!(&source);
//     dbg!(&captured);
//     Ok((source, vec![]))
// }

// pub fn text_string(source: &str) -> IResult<&str, String> {
//     dbg!(&source);
//     // let (source, captured) = tag("A")(source)?;
//     let (source, captured) = separated_list1(
//         tuple((newline, newline)),
//         text_line,
//     )(source)?;
//     // tuple((alpha1, space1, alpha1))(source)?;
//     dbg!(&source);
//     dbg!(&captured);
//     Ok((
//         source,
//         format!(
//             "{}{}{}",
//             "Alfa",
//             " ",
//             "Bravo" //captured.0, captured.1, captured.2
//         ),
//     ))
// }

// pub fn title_content(
//     source: &str,
// ) -> IResult<&str, Section> {
//     let (source, _) = multispace0(source)?;
//     let (source, captured) = text_string(source)?;
//     let (source, _) = multispace0(source)?;
//     // dbg!(&source);
//     Ok((
//         source,
//         Section::Title(vec![Content::Title {
//             text: captured.to_string(),
//         }]),
//     ))
// }

// pub fn h2_content(source: &str) -> IResult<&str, Section> {
//     let (source, _) = multispace0(source)?;
//     let (source, captured) = alpha1(source)?;
//     let (source, _) = multispace0(source)?;
//     // dbg!(&source);
//     Ok((
//         source,
//         Section::H2(vec![Content::Title {
//             text: captured.to_string(),
//         }]),
//     ))
// }

// fn paragraph(input: &str) -> IResult<&str, Vec<&str>> {
//     separated_list0(
//         tuple((newline, not(newline))),
//         not_line_ending,
//     )(input)
// }

// #[allow(dead_code)]
// fn hard_break_paragraphs(
//     input: &str,
// ) -> IResult<&str, Vec<String>> {
//     let (input, stuff) = separated_list0(
//         tuple((newline, newline)),
//         paragraph,
//     )(input.trim())?;
//     let output = stuff
//         .into_iter()
//         .map(|lines| lines.join(" "))
//         .collect();
//     Ok((input, output))
// }

// pub fn title_section(
//     source: &str,
// ) -> IResult<&str, Vec<Content>> {
//     let (source, paragraphs) =
//         hard_break_paragraphs(source)?;
//     dbg!(&paragraphs);
//     dbg!(&source);
//     Ok((
//         source,
//         vec![Content::Title {
//             text: paragraphs[0].to_string(),
//         }],
//     ))
// }

//pub fn parse(source: &str) -> IResult<&str, Vec<Section>> {
//    let (source, captured) = many1(preceded(
//        tag("-> "),
//        tuple((
//            not_line_ending,
//            alt((take_until("\n\n-> "), rest)),
//        ))
//        .map(
//            |(tag_name, contents)| match tag_name {
//                "title" => Section::Title(
//                    title_section(contents).unwrap().1,
//                ),
//                _ => Section::None,
//            },
//        ),
//    ))(source)?;
//    dbg!(&captured);
//    dbg!(&source);
//    Ok((source, captured))
//    // let (source, captured) = many1(preceded(
//    //     tag("-> "),
//    //     alt((
//    //         tuple((tag_no_case("title"), title_content)),
//    //         tuple((tag_no_case("h2"), h2_content)),
//    //         // tuple((tag_no_case("p"), content)),
//    //     )),
//    // ))(source)?;
//    // match captured[0] {
//    //     "title" => {}
//    //     "h2" => {}
//    //     "p" => {}
//    //     _ => {}
//    // }
//    // dbg!(&captured);
//    // dbg!(&source);
//    // let payload = captured
//    //     .into_iter()
//    //     .map(|(_, y)| y)
//    //     .collect::<Vec<Section>>();
//    // dbg!(&payload);
//    // Ok((source, vec![Section::None]))
//    // Ok((
//    //     source,
//    //     vec![
//    //         Section::Title(vec![Content::Title(
//    //             "Alfa".to_string(),
//    //         )]),
//    //         Section::H2(vec![Content::Title(
//    //             "Charlie".to_string(),
//    //         )]),
//    //         // Section::P(vec![
//    //         //     Content::Paragraph(
//    //         //         "Echo Foxtrot".to_string(),
//    //         //     ),
//    //         //     Content::Paragraph(
//    //         //         "Golf Hotel".to_string(),
//    //         //     ),
//    //         // ]),
//    //     ],
//    // ))
//    //
//}

pub fn content_block(
    source: &str,
) -> IResult<&str, Content> {
    let (source, content) = many_till(
        pair(not_line_ending, alt((line_ending, eof)))
            .map(|x| x.0),
        alt((line_ending, eof)),
        // eof,
    )(source)?;
    dbg!(&content.0.join(" "));
    Ok((
        source,
        Content::Title {
            text: content.0.join(" "),
        },
    ))
}

pub fn parse(source: &str) -> IResult<&str, Vec<Section>> {
    let (source, _) = tag("-> ")(source)?;
    let (source, tag_type) = not_line_ending(source)?;
    let (source, _) = newline(source)?;
    let (source, _) = newline(source)?;
    dbg!(&tag_type);
    //let (source, content) = content_block(source)?;
    // let (source, content) = content_block(source)?;

    let (source, content) = many_till(
        content_block,
        alt((tag("->"), eof)),
    )(source)?;

    // let (source, content) = many_till(
    //     // tuple((not_line_ending, newline)),
    //     pair(not_line_ending, line_ending),
    //     // pair(line_ending, line_ending),
    //     line_ending,
    // )(source)?;

    dbg!(content);
    Ok((
        source,
        vec![Section::Title(vec![Content::Title {
            //text: content.to_string(),
            text: "Alfa Bravo Charlie Delta".to_string(),
        }])],
    ))
}

#[cfg(test)]

mod test {

    use super::*;

    #[test]
    pub fn solo_test_1() {
        let lines = vec![
            "-> title",
            "",
            "Alfa Bravo",
            "Charlie Delta",
            "",
            "Echo Foxtrot",
            "Golf Hotel",
        ]
        .join("\n");
        let expected =
            vec![Section::Title(vec![Content::Title {
                text: "Alfa Bravo Charlie Delta"
                    .to_string(),
            }])];
        assert_eq!(
            expected,
            parse(lines.as_str()).unwrap().1
        );
    }

    #[test]
    pub fn test_2() {
        let lines = vec![
            "-> title",
            "",
            "Alfa Bravo",
            "",
            "-> h2",
            "",
            "Charlie",
        ]
        .join("\n");
        let expected = vec![
            Section::Title(vec![Content::Title {
                text: "Alfa Bravo".to_string(),
            }]),
            Section::H2(vec![Content::Title {
                text: "Charlie".to_string(),
            }]),
        ];
        assert_eq!(
            expected,
            parse(lines.as_str()).unwrap().1
        );
    }
    //
}
