use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::multi::many0;
use nom::IResult;

fn main() {
    let source = ">> target1\n>> target2\nexcluded text";
    let expected: Vec<&str> = vec!["target1", "target2"];
    let (remainder, extraction) = parse(source).unwrap();
    dbg!(&extraction);
    dbg!(&remainder);
    assert_eq!(expected, extraction);
}

fn parse(source: &str) -> IResult<&str, Vec<&str>> {
    let (source, values) = many0(get_part)(source)?;
    Ok((source, values))
}

fn get_part(source: &str) -> IResult<&str, &str> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag(">> ")(source)?;
    let (source, value) = not_line_ending(source)?;
    Ok((source, value))
}
