use nom::branch::alt;
use nom::bytes::complete::take_until1;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::combinator::not;
use nom::combinator::rest;
use nom::Err::Error;
use nom::IResult;

// This will process input data and
// split it on two newlines (i.e.
// `\n\n`. Extra whitespace is
// trimmed from the start and end.
// (This means extra newlines in the
// input are eliminated). The goal
// here is to setup the initial
// part of the parse I'm using for
// my neopolitan file format.

fn main() {
    println!("Starting...");

    assert_eq!(
        grab_content("alfa\n\nbravo"),
        Ok(("\n\nbravo", "alfa"))
    );

    assert_eq!(
        grab_content("\n\nbravo"),
        Ok(("", "bravo"))
    );

    assert_eq!(
        grab_content("\n\nalfa\n\nbravo"),
        Ok(("\n\nbravo", "alfa"))
    );

    assert_eq!(
        grab_content("alfa\n\n\nbravo\n\ncharlie"),
        Ok(("\n\n\nbravo\n\ncharlie", "alfa")),
        "Extra newline in front of second paragraph"
    );

    assert_eq!(
        grab_content("\n\n\nbravo\n\ncharlie"),
        Ok(("\n\ncharlie", "bravo")),
        "Extra leading newline"
    );

    assert_eq!(
        grab_content("\n\n"),
        Err(Error(nom::error::Error {
            code: nom::error::ErrorKind::Not,
            input: ""
        }))
    );

    assert_eq!(
        grab_content("alfa\n"),
        Ok(("", "alfa")),
        "Trim whitespace at end"
    );

    let mut input = r#"
The grass curled around the fence post.

Oak is strong and also gives shade.
Cut the pie into large parts.

The straw nest housed five robins.
"#;

    let target: Vec<&str> = vec![
"The grass curled around the fence post.",
"Oak is strong and also gives shade.\nCut the pie into large parts.",
"The straw nest housed five robins."
    ];

    let mut results: Vec<&str> = vec![];

    while let Ok((next, grabbed)) =
        grab_content(input)
    {
        results.push(grabbed);
        input = next;
    }

    assert_eq!(target, results);

    println!("Done.");
}

fn grab_content(data: &str) -> IResult<&str, &str> {
    let (data, _) = multispace0(data)?;
    not(eof)(data)?;
    let (data, content) =
        alt((take_until1("\n\n"), rest))(data)?;
    Ok((data, content.trim()))
}

// - Remove leading spaces
// - Error to exit at end of file
// - Otherwise take content to the next
//   newline or the rest of it
// - Trim whitespace off during return
