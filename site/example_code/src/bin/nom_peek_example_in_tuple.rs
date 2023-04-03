use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::character::complete::multispace1;
use nom::character::complete::none_of;
use nom::character::complete::one_of;
use nom::combinator::eof;
use nom::combinator::peek;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

fn main() {
    let text = vec![
        "- item alfa",
        "line bravo",
        "",
        "paragraph charlie",
        "with lines",
        "",
        "and another",
        "",
        "- item delta",
        "line echo",
        "",
        "# Header Foxtrot",
        "",
    ];
    let expected = vec![
        "-> list",
        "",
        "- item alfa",
        "line bravo",
        "",
        "-> p",
        "",
        "paragraph charlie",
        "with lines",
        "",
        "and another",
        "",
        "-> list",
        "",
        "- item delta",
        "line echo",
        "",
        "-> h1",
        "",
        "Header Foxtrot",
        "",
    ];
    let source = text.join("\n");
    let response = parse(source.as_str());
    println!("{}", response);
    assert_eq!(expected.join("\n"), response);
    println!("Test complete");
}

pub fn parse<'a>(source: &str) -> String {
    let mut payload = ("".to_string(), Area::Base);
    let mut output = "".to_string();
    let prepend_newline = format!("\n{}", source);
    let mut source = prepend_newline.as_str();
    loop {
        (source, payload) = crawler(source, payload.1).unwrap();
        output.push_str(payload.0.as_str());
        if source == "" {
            break;
        }
    }
    output
}

pub fn crawler(source: &str, area: Area) -> IResult<&str, (String, Area)> {
    match area {
        Area::Base => {
            let (a, b) = alt((
                // Clear any leading text so this always
                // starts on a new line
                multispace1.map(|_| (format!(""), Area::Base)),
                // Start a list if you find one
                tuple((tag("- "), take(1u32)))
                    .map(|x| (format!("-> list\n\n- {}", x.1), Area::List)),
                // Header
                tuple((tag("# "), take(1u32))).map(|x| (format!("-> h1\n\n{}", x.1), Area::List)),
                // This is what closes the processing
                eof.map(|_| (format!(""), Area::Base)),
                // Anything else is the start of a paragraph
                take(1u32).map(|x| (format!("-> p\n\n{}", x), Area::Paragraph)),
            ))(source)?;
            Ok((a, b))
        }
        Area::List => {
            let (a, b) = alt((
                tuple((tag("\n\n"), peek(none_of("-")))).map(|_| (format!("\n\n"), Area::Base)),
                tag("\n- ").map(|_| (format!("- "), Area::List)),
                take(1u32).map(|x| (format!("{}", x), Area::List)),
            ))(source)?;
            Ok((a, b))
        }
        Area::Paragraph => {
            let (a, b) = alt((
                // this could be done to take into account the `.`` after
                // the number with a more complex parser
                tuple((tag("\n\n"), peek(one_of("-#*1234567890"))))
                    .map(|_| (format!("\n\n"), Area::Base)),
                take(1u32).map(|x| (format!("{}", x), Area::Paragraph)),
            ))(source)?;
            Ok((a, b))
        }
    }
}

#[derive(Debug)]
pub enum Area {
    Base,
    List,
    Paragraph,
}
