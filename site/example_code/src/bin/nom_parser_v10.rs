#![allow(dead_code, unused_imports, unused_variables)]
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::IResult;

#[derive(Debug, PartialEq)]
enum Wrapper {
    Page { children: Vec<Section> },
}

#[derive(Debug, PartialEq)]
enum Section {
    Title { children: Vec<Block> },
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

fn main() {
    let lines = vec!["-> title", "", "quick <<link|example.com|brown>> fox"].join("\n");
    let source = lines.as_str();
    let expected = Wrapper::Page {
        children: vec![Section::Title {
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
        }],
    };
    let result = parse(source).unwrap().1;
    assert_eq!(expected, result);
}

fn parse(source: &str) -> IResult<&str, Wrapper> {
    let (_, sections) = many_till(section, eof)(source)?;
    let expected = Wrapper::Page {
        children: sections.0,
    };
    Ok(("", expected))
}

fn section(source: &str) -> IResult<&str, Section> {
    let (_, blocks) = many_till(block, eof)(source)?;
    Ok(("", Section::Title { children: blocks.0 }))
}

fn block(source: &str) -> IResult<&str, Block> {
    let (_, content) = many_till(content, eof)(source)?;
    Ok((
        "",
        Block::P {
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
        },
    ))
}

fn content(source: &str) -> IResult<&str, Content> {
    let (a, b) = alt((tag(" "), tag("\n\n"), rest))(source)?;
    Ok((
        a,
        Content::Text {
            text: "quick".to_string(),
        },
    ))
}
