#![allow(warnings)]
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::bytes::complete::take_until1;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::combinator::not;
use nom::combinator::rest;
use nom::IResult;

struct Parser {}

impl Parser {
    pub fn get_hashtag<'a>(
        &'a self, s: &'a str,
    ) -> IResult<&str, &str> {
        dbg!(s);
        let (s, _) = multispace0(s)?;
        let (s, grabbed) = take_until("#+")(s)?;
        // let (data, _) = tag("-> ")(data)?;
        // let (data, token) = alphanumeric1(data)?;
        // let (data, _) = not_line_ending(data)?;
        // let (data, _) = line_ending(data)?;
        // let (data, _) = line_ending(data)?;
        // let (data, content) = alt((
        //     take_until1("\n\n-> "),
        //     rest,
        // ))(data)?;
        // Ok((data, (token, content)))

        Ok((s, grabbed))
    }
}

fn main() {
    let p = Parser {};
    let input = r#"#+TITLE: this is the title"#;

    let response = p.get_hashtag(input);

    dbg!(response);
}
