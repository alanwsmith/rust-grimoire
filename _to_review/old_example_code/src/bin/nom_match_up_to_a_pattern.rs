use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

// This is the baseline for matching
// part of a string up until another
// pattern. Right now it's a dupliate
// of `nom_get_lines_after_a_tag`` which
// is the first place I got it working.
// The next step is to make it more generic
// and put it into the grimoire

fn main() {
    basic_test();
    clear_lines_after();
    capture_multiple_lines();
}

fn parse(source: &str) -> IResult<&str, String> {
    let (a, _) = take_until("-> title")(source)?;
    let (a, _) = tag("-> title")(a)?;
    let (a, _) = multispace0(a)?;
    let (a, b) = many_till(
        take(1u32),
        alt((tuple((line_ending, line_ending)).map(|_| ""), eof)),
    )(a)?;
    Ok((a, b.0.join("")))
}

fn basic_test() {
    let lines = ["prelude to ignore", "", "-> title", "", "alfa bravo"];
    let source = lines.join("\n");
    let (_, b) = parse(source.as_str()).unwrap();
    assert_eq!("alfa bravo", b);
}

fn clear_lines_after() {
    let lines = ["-> title", "", "charlie delta", "", "echo foxtrot"];
    let source = lines.join("\n");
    let (_, b) = parse(source.as_str()).unwrap();
    assert_eq!("charlie delta", b);
}

fn capture_multiple_lines() {
    let lines = ["-> title", "", "golf hotel", "india juliette", "", ""];
    let source = lines.join("\n");
    let (_, b) = parse(source.as_str()).unwrap();
    assert_eq!("golf hotel\nindia juliette", b);
}
