use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_till;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::error::Error;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

#[derive(Debug, PartialEq)]
enum Wrapper {
    Page { children: Vec<Section> },
}

#[derive(Debug, PartialEq)]
enum Section {
    Title { children: Vec<Block> },
    Paragraphs { children: Vec<Block> },
}

#[derive(Debug, PartialEq)]
enum Block {
    P { children: Vec<Content> },
}

#[derive(Debug, PartialEq)]
enum Content {
    Link { text: String, url: String },
    Text { text: String },
    Space,
}

// #[derive(Debug, PartialEq)]
// enum Trigger {
//     Title,
//     Paragraphs,
// }

fn main() {
    let lines = vec![
        "-> title",
        "",
        "quick <<link|example.com|brown>> fox",
        "",
        "-> p",
        "",
        "the book cover",
        "",
        "random string",
        "with content",
    ]
    .join("\n");
    let source = lines.as_str();
    let expected = Wrapper::Page {
        children: vec![
            Section::Title {
                children: vec![Block::P {
                    children: vec![
                        Content::Text {
                            text: "quick".to_string(),
                        },
                        Content::Space,
                        Content::Link {
                            text: "brown".to_string(),
                            url: "example.com".to_string(),
                        },
                        Content::Space,
                        Content::Text {
                            text: "fox".to_string(),
                        },
                    ],
                }],
            },
            Section::Paragraphs {
                children: vec![
                    Block::P {
                        children: vec![
                            Content::Text {
                                text: "the".to_string(),
                            },
                            Content::Space,
                            Content::Text {
                                text: "book".to_string(),
                            },
                            Content::Space,
                            Content::Text {
                                text: "cover".to_string(),
                            },
                        ],
                    },
                    Block::P {
                        children: vec![
                            Content::Text {
                                text: "random".to_string(),
                            },
                            Content::Space,
                            Content::Text {
                                text: "string".to_string(),
                            },
                            Content::Space,
                            Content::Text {
                                text: "with".to_string(),
                            },
                            Content::Space,
                            Content::Text {
                                text: "content".to_string(),
                            },
                        ],
                    },
                ],
            },
        ],
    };
    let result = parse(source).unwrap().1;
    assert_eq!(expected, result);
    println!("Process complete");
}

fn parse(source: &str) -> IResult<&str, Wrapper> {
    dbg!(source);
    let (_, sections) = many_till(section, eof)(source)?;
    let expected = Wrapper::Page {
        children: sections.0,
    };
    Ok(("", expected))
}

fn section(source: &str) -> IResult<&str, Section> {
    dbg!(source);
    let (source, _) = multispace0(source)?;
    let (remainder, sec) = alt((
        tuple((tag("-> title\n\n"), alt((take_until("\n\n-> "), rest)))).map(|t| {
            let (_, b) = many_till(block, eof)(t.1).unwrap();
            Section::Title { children: b.0 }
        }),
        tuple((tag("-> p\n\n"), alt((take_until("\n\n-> "), rest)))).map(|t| {
            let (_, b) = many_till(block, eof)(t.1).unwrap();
            Section::Paragraphs { children: b.0 }
        }),
        // tag("-> p\n\n").map(|_| Trigger::Paragraphs),
    ))(source)?;

    // .map(|(a, b)| match b {
    //     Trigger::Title => {
    //         let (remainder, blocks) = many_till(block, eof)(a).unwrap();
    //         (remainder, Section::Title { children: blocks.0 })
    //     }
    //     Trigger::Paragraphs => {
    //         let (remainder, blocks) = many_till(block, eof)(a).unwrap();
    //         (remainder, Section::Paragraphs { children: blocks.0 })
    //     }
    // })?;

    Ok((remainder, sec))
}

// fn section(source: &str) -> IResult<&str, Section> {
//     dbg!(source);
//     let (remainder, sec) = alt((
//         tag("-> title\n\n").map(|_| Trigger::Title),
//         tag("-> p\n\n").map(|_| Trigger::Paragraphs),
//     ))(source)
//     .map(|(a, b)| match b {
//         Trigger::Title => {
//             let (remainder, blocks) = many_till(block, eof)(a).unwrap();
//             (remainder, Section::Title { children: blocks.0 })
//         }
//         Trigger::Paragraphs => {
//             let (remainder, blocks) = many_till(block, eof)(a).unwrap();
//             (remainder, Section::Paragraphs { children: blocks.0 })
//         }
//     })?;
//     Ok((remainder, sec))
// }

fn block(source: &str) -> IResult<&str, Block> {
    dbg!(source);
    let (remainder, content) = many_till(content, alt((tag("\n\n"), eof)))(source)?;
    Ok((
        remainder,
        Block::P {
            children: content.0,
        },
    ))
}

fn content(source: &str) -> IResult<&str, Content> {
    dbg!(source);
    let (remainder, content) = alt((
        tuple((
            // I'm not sure if this is the right way to
            // setup the type as &str, but it works so far.
            tag_no_case::<&str, &str, Error<&str>>("<<link|"),
            take_until("|"),
            tag("|"),
            take_until(">>"),
            tag(">>"),
        ))
        .map(|t| Content::Link {
            url: t.1.to_string(),
            text: t.3.to_string(),
        }),
        multispace1.map(|_| Content::Space),
        take_till(|c| c == ' ' || c == '\n' || c == '\t').map(|t: &str| Content::Text {
            text: t.to_string(),
        }),
    ))(source)?;
    Ok((remainder, content))
}
