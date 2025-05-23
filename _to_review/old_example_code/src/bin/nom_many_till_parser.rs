#![allow(warnings)]
use nom::multi::many0;
//use nom::multi::many1;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::IResult;
use nom::Parser;

#[derive(Debug, PartialEq)]
enum Widget {
    A,
    B,
    C,
}

fn main() {
    let source = "a b a c a b";
    let expected: IResult<&str, Vec<Widget>> = Ok((
        "b",
        vec![Widget::A, Widget::B, Widget::A, Widget::C, Widget::A],
    ));
    let result = parse(source);
    assert_eq!(expected, result);
    println!("Process complete")
}

fn parse(source: &str) -> IResult<&str, Vec<Widget>> {
    let (a, b) = many0(crawler)(source)?;
    dbg!(source);
    Ok((a, b))
}

fn crawler(source: &str) -> IResult<&str, Widget> {
    let (a, b) = alt((
        tag("a ").map(|x| Widget::A),
        tag("b ").map(|x| Widget::B),
        tag("c ").map(|x| Widget::C),
    ))(source)?;
    Ok((a, b))
}
