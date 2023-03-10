#![allow(unused_imports)]
use nom::branch::alt;
use nom::branch::Alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until1;
use nom::character::complete::alphanumeric1;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::IResult;

fn main() {
    let s = r#"-> title

This Is The Title

-> c

This is some content"#;
    match get_stuff(s) {
        Ok((a, b)) => {
            dbg!(b);
            match get_stuff(a) {
                Ok((c, d)) => {
                    dbg!(d);
                }
                Err(e) => {
                    dbg!(e);
                }
            }
        }
        Err(e) => {
            dbg!(e);
        }
    };
}

fn get_stuff(
    data: &str,
) -> IResult<&str, (&str, &str)> {
    let (data, _) = multispace0(data)?;
    let (data, _) = tag("-> ")(data)?;
    let (data, token) = alphanumeric1(data)?;
    dbg!(token);
    let (data, _) = multispace1(data)?;

    let (data, content) =
        alt((take_until1("\n\n-> "), rest))(data)?;

    dbg!(data);
    dbg!(content);

    Ok((data, (token, content)))
}
