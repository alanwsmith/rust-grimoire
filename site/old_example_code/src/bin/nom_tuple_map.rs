use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::sequence::tuple;
use nom::IResult;

fn main() {
    let (a, b) = parse("alfa bravo charlie").unwrap();
    dbg!(a);
    dbg!(b);
}

fn parse(source: &str) -> IResult<&str, &str> {
    let (a, b) = tuple((tag("alfa"), multispace1, tag("bravo")))(source).map(|(x, y)| (x, y.2))?;
    Ok((a, b))
}
