use nom::bytes::complete::take_till;
use nom::IResult;

fn main() {
    assert_eq!(Ok(("&b*c", "a")), find_things("a&b*c"));
    assert_eq!(Ok(("*b&c", "a")), find_things("a*b&c"));
    assert_eq!(Ok(("", "abc")), find_things("abc"));
}

fn find_things(source: &str) -> IResult<&str, &str> {
    let (remainder, value) = take_till(|c| c == '*' || c == '&')(source)?;
    Ok((remainder, value))
}
