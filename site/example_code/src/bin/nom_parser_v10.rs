use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_till;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace1;
use nom::combinator::eof;
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
    let (source, _) = tag("-> title\n\n")(source)?;
    let (_, blocks) = many_till(block, eof)(source)?;
    Ok(("", Section::Title { children: blocks.0 }))
}

fn block(source: &str) -> IResult<&str, Block> {
    let (_, content) = many_till(content, eof)(source)?;
    Ok((
        "",
        Block::P {
            children: content.0,
        },
    ))
}

fn content(source: &str) -> IResult<&str, Content> {
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
