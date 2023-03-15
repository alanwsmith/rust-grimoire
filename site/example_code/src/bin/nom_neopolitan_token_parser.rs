use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::space0;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::Err;
use nom::IResult;

// Note: this only requires one line to be
// above the `-> ` token. Or, more to the
// point it just looks for a newline
// before it. Ideally, it would be two,
// but that's more complicated than I
// want to put the time into for now.
//
// Two empty lines are required after
// a header. They can have whitespace
// but no other characters.

#[derive(Debug, PartialEq)]
enum RawBlock {
    Title { text: String },
    P { text: String },
    Code { text: String },
    Error { text: String },
}

fn main() {
    println!("Testing...");

    // Basic test
    let source = "-> TITLE\n\nA Title\n\n-> P\n\nsome\n\nparagraphs";
    let expected: Vec<RawBlock> = vec![
        RawBlock::Title {
            text: "A Title".to_string(),
        },
        RawBlock::P {
            text: "some\n\nparagraphs".to_string(),
        },
    ];
    assert_eq!(expected, split_tokens(source).unwrap().1);

    // Multiple paragraphs
    // White space added after headers too
    let source = r#"-> CODE  
   
some code

-> P   
   
para1

-> P

para2"#;

    let expected: Vec<RawBlock> = vec![
        RawBlock::Code {
            text: "some code".to_string(),
        },
        RawBlock::P {
            text: "para1".to_string(),
        },
        RawBlock::P {
            text: "para2".to_string(),
        },
    ];
    assert_eq!(expected, split_tokens(source).unwrap().1);

    // Error on invalid headers
    let source = "-> INVALIDHEADER\n\nis invalid";
    assert_eq!(
        Err(Err::Error(Error::new(
            "INVALIDHEADER\n\nis invalid",
            ErrorKind::Tag
        ))),
        split_tokens(source)
    );
    println!("Process complete");
}

fn split_tokens(source: &str) -> IResult<&str, Vec<RawBlock>> {
    let (_, tokens) = many_till(do_split, eof)(source)?;
    Ok(("", tokens.0))
}

fn newline_pair(source: &str) -> IResult<&str, &str> {
    let (source, _) = space0(source)?;
    let (source, _) = line_ending(source)?;
    let (source, _) = space0(source)?;
    let (source, _) = line_ending(source)?;
    Ok((source, value))
}

fn do_split(source: &str) -> IResult<&str, RawBlock> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("-> ")(source)?;
    let (source, value) = tuple((
        alt((tag("TITLE"), tag("CODE"), tag("P"))),
        newline_pair,
        alt((take_until("\n-> "), rest)),
    ))(source)?;
    let string_value = value.2.trim().to_string();
    let response = match value.0 {
        "TITLE" => RawBlock::Title { text: string_value },
        "P" => RawBlock::P { text: string_value },
        "CODE" => RawBlock::Code { text: string_value },
        _ => RawBlock::Error { text: string_value },
    };
    Ok((source, response))
}
