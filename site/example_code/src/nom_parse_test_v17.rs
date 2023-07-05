#![allow(unused_imports)]
#![allow(unused_variables)]
use nom::branch::alt;
use nom::bytes::complete::is_not;
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
use nom::combinator::opt;
use nom::combinator::rest;
use nom::multi::many0;
use nom::multi::many1;
use nom::multi::many_till;
use nom::multi::separated_list0;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

#[derive(Debug, PartialEq)]
pub enum Section {
    H2 {
        headline: Content,
        paragraphs: Vec<Content>,
    },
    Title {
        headline: Content,
        paragraphs: Vec<Content>,
    },
    P(Vec<crate::nom_parse_test_v17::Content>),
    None,
}

#[derive(Debug, PartialEq)]
pub enum Content {
    Headline {
        attributes: Vec<Attribute>,
        text: String,
    },
    Paragraph {
        text: String,
    },
    None,
}

#[derive(Debug, PartialEq)]
pub enum Attribute {
    Autofocus,
    ClassAttr(Vec<String>),
    DataAttr(String, String),
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

//pub fn parse(source: &str) -> IResult<&str, Vec<Section>> {
//    let (a, b) = many1(
//        delimited(tag("-> "), not_line_ending, multispace1)
//            .map(|tag_type| match tag_type {
//                "title" => {
//                    // let (c, v) =
//                    //Section::Title(vec![])
//                    Section::None
//                }
//                _ => Section::None,
//            }),
//    )(source.trim())?;
//    dbg!(&b);
//    dbg!(&a);
//    //let (source, _) = tag("-> ")(source)?;
//    //let (source, tag_type) = not_line_ending(source)?;
//    //let (source, _) = newline(source)?;
//    //let (source, _) = newline(source)?;
//    //dbg!(&tag_type);
//    ////let (source, content) = content_block(source)?;
//    //// let (source, content) = content_block(source)?;
//    // let (source, content) = many_till(
//    //     content_block,
//    //     alt((tag("->"), eof)),
//    // )(source)?;
//    // let (source, content) = many_till(
//    //     // tuple((not_line_ending, newline)),
//    //     pair(not_line_ending, line_ending),
//    //     // pair(line_ending, line_ending),
//    //     line_ending,
//    // )(source)?;
//    // dbg!(content);
//    Ok((
//        source,
//        vec![Section::Title(vec![Content::Title {
//            //text: content.to_string(),
//            text: "Alfa Bravo Charlie Delta".to_string(),
//        }])],
//    ))
//}

//#[cfg(test)]
//mod test2 {
//    // use super::*;
//    // #[test]
//    // pub fn solo_test_1() {
//    //     let lines = vec![
//    //         "-> title",
//    //         "",
//    //         "Alfa Bravo",
//    //         "Charlie Delta",
//    //         "",
//    //         "Echo Foxtrot",
//    //         "Golf Hotel",
//    //         "",
//    //         "",
//    //         "-> h2",
//    //         "",
//    //         "Whiskey Tango",
//    //         "Echo Sierra",
//    //     ]
//    //     .join("\n");
//    //     let expected =
//    //         vec![Section::Title(vec![Content::Title {
//    //             text: "Alfa Bravo Charlie Delta"
//    //                 .to_string(),
//    //         }])];
//    //     assert_eq!(
//    //         expected,
//    //         parse(lines.as_str()).unwrap().1
//    //     );
//    // }
//    // #[test]
//    // pub fn test_2() {
//    //     let lines = vec![
//    //         "-> title",
//    //         "",
//    //         "Alfa Bravo",
//    //         "",
//    //         "-> h2",
//    //         "",
//    //         "Charlie",
//    //     ]
//    //     .join("\n");
//    //     let expected = vec![
//    //         Section::Title(vec![Content::Title {
//    //             text: "Alfa Bravo".to_string(),
//    //         }]),
//    //         Section::H2(vec![Content::Title {
//    //             text: "Charlie".to_string(),
//    //         }]),
//    //     ];
//    //     assert_eq!(
//    //         expected,
//    //         parse(lines.as_str()).unwrap().1
//    //     );
//    // }
//    //
//}

// pub fn content_block(
//     source: &str,
// ) -> IResult<&str, String> {
//     let (source, content) = many_till(
//         pair(not_line_ending, alt((line_ending, eof)))
//             .map(|x| x.0),
//         alt((multispace1, eof)),
//     )(source)?;
//     Ok((source, content.0.join(" ")))
// }

// pub fn content_blocks(
//     source: &str,
// ) -> IResult<&str, Vec<String>> {
//     let (source, b) = many_till(
//         content_block,
//         alt((line_ending, eof)),
//     )(source.trim())?;
//     Ok((source, b.0))
// }

// #[cfg(test)]
// mod content_block_tests {
//     use super::*;

// #[test]
// pub fn content_block_basic() {
//     let lines = vec!["alfa bravo"].join("\n");
//     let expected = format!("alfa bravo");
//     assert_eq!(
//         expected,
//         content_block(lines.as_str()).unwrap().1
//     );
// }

// #[test]
// pub fn content_block_multi_line() {
//     let lines =
//         vec!["alfa bravo", "charlie delta"].join("\n");
//     let expected = format!("alfa bravo charlie delta");
//     assert_eq!(
//         expected,
//         content_block(lines.as_str()).unwrap().1
//     );
// }

// #[test]
// pub fn content_blocks_single_blocks() {
//     let lines = vec!["foxtrot golf", "hotel whiskey"]
//         .join("\n");
//     let expected =
//         vec![format!("foxtrot golf hotel whiskey")];
//     assert_eq!(
//         expected,
//         content_blocks(lines.as_str()).unwrap().1
//     );
// }

// #[test]
// pub fn content_blocks_multiple_blocks() {
//     let lines = vec![
//         "foxtrot golf",
//         "hotel whiskey",
//         "",
//         "alfa delta",
//         "victor foxtrot",
//     ]
//     .join("\n");
//     let expected = vec![
//         format!("foxtrot golf hotel whiskey"),
//         format!("alfa delta victor foxtrot"),
//     ];
//     assert_eq!(
//         expected,
//         content_blocks(lines.as_str()).unwrap().1
//     );
// }

// #[test]
// pub fn content_blocks_spacing_blocks() {
//     let lines = vec![
//         "",
//         "",
//         "tango alfa",
//         "whiskey oscar",
//         "    ",
//         "",
//         "",
//         "foxtrot hotel",
//         "echo charlie",
//         "",
//         "",
//         "",
//     ]
//     .join("\n");
//     let expected = vec![
//         format!("tango alfa whiskey oscar"),
//         format!("foxtrot hotel echo charlie"),
//     ];
//     assert_eq!(
//         expected,
//         content_blocks(lines.as_str()).unwrap().1
//     );
// }

//
// }

pub fn content_block(source: &str) -> IResult<&str, String> {
    let (source, content) = many_till(
        pair(not_line_ending, alt((line_ending, eof))).map(|x| x.0),
        alt((multispace1, eof)),
    )(source)?;
    Ok((source, content.0.join(" ")))
}

pub fn content_blocks(source: &str) -> IResult<&str, Vec<String>> {
    let (source, b) =
        many_till(content_block, alt((line_ending, eof)))(source.trim())?;
    Ok((source, b.0))
}

pub fn sections(source: &str) -> IResult<&str, Vec<Section>> {
    let (source, captured) = many1(preceded(
        tuple((multispace0, tag("-> "))),
        tuple((not_line_ending, alt((take_until("\n\n-> "), rest)))).map(
            |(tag_name, contents)| {
                match tag_name {
                    "title" => {
                        // dbg!(&contents);
                        // dbg!(&remainder);
                        // (
                        //     title_section(contents).unwrap().1,
                        //     remainder,
                        // )
                        title_section(contents).unwrap().1
                    }
                    "h2" => {
                        // dbg!(&contents);
                        // dbg!(&remainder);
                        // (
                        //     h2_section(contents).unwrap().1,
                        //     remainder,
                        // )
                        h2_section(contents).unwrap().1
                    }
                    _ => {
                        // dbg!(&contents);
                        Section::None
                    }
                }
            },
        ),
    ))(source)?;
    Ok((source, captured))

    //
}

pub fn paragraph_block(source: &str) -> IResult<&str, Content> {
    // dbg!(&source);
    let (source, content) = many_till(
        pair(not_line_ending, alt((line_ending, eof))).map(|x| x.0),
        alt((multispace1, eof)),
    )(source.trim())?;
    Ok((
        source,
        Content::Paragraph {
            text: (content.0.join(" ")),
        },
    ))
}

//////////////////////////////////////////////////////////////////////////
// Section Attributes
//////////////////////////////////////////////////////////////////////////

pub fn autofocus_section_attr(source: &str) -> IResult<&str, Attribute> {
    let (source, attr) = tag("autofocus")
        .parse(source)
        .map(|(x, y)| (x, Attribute::Autofocus))?;
    Ok((source, attr))
}

pub fn class_section_attr(source: &str) -> IResult<&str, Attribute> {
    let (source, values) = preceded(
        tag("class: "),
        many1(is_not(" \n").map(|x: &str| x.to_string())),
    )(source)?;
    Ok((source, Attribute::ClassAttr(values)))
}

pub fn data_section_attr(source: &str) -> IResult<&str, Attribute> {
    let (source, attr) = preceded(
        tag("data-"),
        separated_pair(
            is_not::<&str, &str, nom::error::Error<&str>>(":"),
            tag(": "),
            not_line_ending,
        ),
    )(source)?;
    Ok((
        source,
        Attribute::DataAttr(attr.0.to_string(), attr.1.to_string()),
    ))
}

pub fn section_attrs(source: &str) -> IResult<&str, Vec<Attribute>> {
    dbg!("-------------------------");
    dbg!(&source);
    let (source, attrs) = many0(preceded(
        tuple((multispace0, tag(">> "))),
        alt((
            autofocus_section_attr,
            data_section_attr,
            class_section_attr,
        )),
    ))(source.trim())?;
    Ok((source, attrs))
}

// ))

// Some(_) => {
//     Attribute::DataAttr(key.to_string(), value.to_string())
// }
// None => match key {
//     "class" => Attribute::ClassAttr(
//         value
//             .split(" ")
//             .collect::<Vec<&str>>()
//             .into_iter()
//             .map(|s| s.to_string())
//             .collect(),
//     ),
//     "data-" => Attribute::DataAttr(
//         "dolf".to_string(),
//         "asdf".to_string(),
//     ),
//     _ => Attribute::None,
// },
// }),

// tuple((
//     opt(tag::<&str, &str, nom::error::Error<&str>>("data-")),
//     is_not(":"),
//     tag(": "),
//     not_line_ending,
// ))
// .map(|(is_data, key, spacer, value)| match is_data {
//     Some(_) => {
//         Attribute::DataAttr(key.to_string(), value.to_string())
//     }
//     None => match key {
//         "class" => Attribute::ClassAttr(
//             value
//                 .split(" ")
//                 .collect::<Vec<&str>>()
//                 .into_iter()
//                 .map(|s| s.to_string())
//                 .collect(),
//         ),
//         "data-" => Attribute::DataAttr(
//             "dolf".to_string(),
//             "asdf".to_string(),
//         ),
//         _ => Attribute::None,
//     },
// }),
// )),

//}

pub fn headline_block(source: &str) -> IResult<&str, Content> {
    let (source, attrs) = section_attrs(source)?;
    let (source, content) = many_till(
        pair(not_line_ending, alt((line_ending, eof))).map(|x| x.0),
        alt((multispace1, eof)),
    )(source.trim())?;
    Ok((
        source,
        Content::Headline {
            attributes: attrs,
            text: content.0.join(" "),
        },
    ))
}

pub fn paragraph_blocks(source: &str) -> IResult<&str, Vec<Content>> {
    // dbg!(&source);
    let (source, paragraphs) = many_till(paragraph_block, eof)(source)?;
    Ok((source, paragraphs.0))
}

pub fn title_section(source: &str) -> IResult<&str, Section> {
    // dbg!(&source);
    let (source, headline) = headline_block(source)?;
    let (source, paragraphs) = paragraph_blocks(source)?;
    // dbg!(&source);
    Ok((
        source,
        Section::Title {
            headline,
            paragraphs,
        },
    ))
}

pub fn h2_section(source: &str) -> IResult<&str, Section> {
    // dbg!(&source);
    let (source, headline) = headline_block(source)?;
    let (source, paragraphs) = paragraph_blocks(source)?;
    // dbg!(&source);
    Ok((
        source,
        Section::H2 {
            headline,
            paragraphs,
        },
    ))
}

#[cfg(test)]
mod section_test {
    use super::*;

    #[test]
    pub fn title_section_basic() {
        let lines = vec!["-> title", "", "delta echo"].join("\n");
        let expected = vec![Section::Title {
            headline: Content::Headline {
                attributes: vec![],
                text: "delta echo".to_string(),
            },
            paragraphs: vec![],
        }];
        assert_eq!(expected, sections(lines.as_str()).unwrap().1);
    }

    #[test]
    pub fn title_section_with_multilines() {
        let lines = vec![
            "-> title",
            "",
            "delta echo",
            "",
            "tango sierra",
            "",
            "golf hotel",
        ]
        .join("\n");
        let expected = vec![Section::Title {
            headline: Content::Headline {
                attributes: vec![],
                text: "delta echo".to_string(),
            },
            paragraphs: vec![
                Content::Paragraph {
                    text: "tango sierra".to_string(),
                },
                Content::Paragraph {
                    text: "golf hotel".to_string(),
                },
            ],
        }];
        assert_eq!(expected, sections(lines.as_str()).unwrap().1);
    }

    #[test]
    pub fn multiple_secitons() {
        let lines = vec![
            "-> title",
            "",
            "sierra alfa",
            "",
            "-> h2",
            "",
            "echo tango",
            "",
            "",
        ]
        .join("\n");
        let expected = vec![
            Section::Title {
                headline: Content::Headline {
                    attributes: vec![],
                    text: "sierra alfa".to_string(),
                },
                paragraphs: vec![],
            },
            Section::H2 {
                headline: Content::Headline {
                    attributes: vec![],
                    text: "echo tango".to_string(),
                },
                paragraphs: vec![],
            },
        ];
        assert_eq!(expected, sections(lines.as_str()).unwrap().1);
    }

    #[test]
    pub fn solo_attributes() {
        let lines = vec![
            "-> h2",
            ">> class: hotel",
            "",
            "alfa echo",
            "bravo charlie",
            "",
            "-> h2",
            ">> data-golf: victor",
            ">> autofocus",
            "",
            "delta tango",
            "whiskey sierra",
            "",
            "",
        ]
        .join("\n");
        let expected = vec![
            Section::H2 {
                headline: Content::Headline {
                    attributes: vec![Attribute::ClassAttr(vec![
                        "hotel".to_string()
                    ])],
                    text: "alfa echo bravo charlie".to_string(),
                },
                paragraphs: vec![],
            },
            Section::H2 {
                headline: Content::Headline {
                    attributes: vec![
                        Attribute::DataAttr(
                            "golf".to_string(),
                            "victor".to_string(),
                        ),
                        Attribute::Autofocus,
                    ],
                    text: "delta tango whiskey sierra".to_string(),
                },
                paragraphs: vec![],
            },
        ];
        assert_eq!(expected, sections(lines.as_str()).unwrap().1);
    }

    //
}
