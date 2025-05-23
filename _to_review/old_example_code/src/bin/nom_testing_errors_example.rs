use nom::bytes::complete::take_until;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::Err;
use nom::IResult;

fn main() {
    assert_eq!(parse("alfa-bravo"), Ok(("", "alfa")));
    assert_eq!(
        parse("alfa-charlie"),
        Err(Err::Error(Error::new("alfa-charlie", ErrorKind::TakeUntil)))
    );
}

fn parse(source: &str) -> IResult<&str, &str> {
    let (_, value) = take_until("-bravo")(source)?;
    Ok(("", value))
}
