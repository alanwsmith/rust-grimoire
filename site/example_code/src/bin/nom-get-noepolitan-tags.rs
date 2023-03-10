#![allow(unused_imports)]
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until1;
use nom::character::complete::alphanumeric1;
use nom::character::complete::multispace1;
use nom::IResult;

// This is the initail test that just
// gets the first tag. The v2 file
// is aimed at getting repated tags

fn main() {
    let s = r#"-> title

This Is The Title

-> c

This is some content"#;
    match get_stuff(s) {
        Ok((_a, b)) => {
            dbg!(b);
        }
        Err(e) => {
            dbg!(e);
        }
    };
}

fn get_stuff(data: &str) -> IResult<&str, &str> {
    let (data, _) = tag("-> ")(data)?;
    let (data, token) = alphanumeric1(data)?;
    dbg!(token);
    let (data, _) = multispace1(data)?;
    let (data, content) =
        take_until1("\n\n-> ")(data)?;

    dbg!(data);
    dbg!(content);

    Ok(("", ""))
}
