use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until1;
use nom::character::complete::alphanumeric1;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::combinator::rest;
use nom::IResult;

// GOAL of this version is to grab multiple
// sections via a loop

fn main() {
    let mut s = r#"-> title

This Is The Title

-> c

This is some content

With a second line

-> h3

This is another header"#;

    while let Ok((next, (token, content))) = get_stuff(s)
    {
        dbg!(token);
        dbg!(content);
        // dbg!(next);
        s = next;
    }
}

fn get_stuff(data: &str) -> IResult<&str, (&str, &str)> {
    let (data, _) = multispace0(data)?;
    let (data, _) = tag("-> ")(data)?;
    let (data, token) = alphanumeric1(data)?;
    // dbg!(token);
    let (data, _) = multispace1(data)?;

    let (data, content) =
        alt((take_until1("\n\n-> P"), rest))(data)?;
    // dbg!(data);
    // dbg!(content);

    Ok((data, (token, content)))
}
