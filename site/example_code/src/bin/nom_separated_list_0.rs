#![allow(unused_imports)]
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::alpha1;
use nom::multi::separated_list0;
use nom::IResult;

fn main() {
    assert_eq!(split("a|b|c"), Ok(("", vec!["a", "b", "c"])))
}

// This doesn't work

fn split(source: &str) -> IResult<&str, Vec<&str>> {
    let (remainder, mut parts) = separated_list0(tag("|"), take_until("|"))(source)?;
    parts.push(remainder);
    Ok(("", parts))
}
