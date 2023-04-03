use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::not_line_ending;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

fn main() {
    let source = r#"alfa 
bravo
#target charlie

delta

#target echo

"#;
    let items = parse(source).unwrap().1;
    dbg!(items);
}

fn parse(source: &str) -> IResult<&str, Vec<&str>> {
    let token = "#target ";
    let (_, items) =
        many0(tuple((take_until(token), tag(token), not_line_ending)).map(|x| x.2))(source)?;
    Ok(("", items))
}
