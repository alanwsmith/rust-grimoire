use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::sequence::delimited;
use nom::Err;
use nom::IResult;
// use nom::Parser;

// NOTE: Not trying to solve for the first character
// being a "*" e.g. at the start of a file. The
// trigger is at least one space in front of it.
// The same goes for the end of the file. There
// has to be at least one space after the last
// `*`` . So, these tests exect to error.
//
// `*quick brown* fox`
// `the *quick brown*`

fn main() {
    assert_eq!(parse("the *quick* *brown* fox"), Ok(("", "quick")));
    assert_eq!(parse("the *quick*brown* fox"), Ok(("", "quick*brown")));
    assert_eq!(parse("the *quick brown* fox"), Ok(("", "quick brown")));
    assert_eq!(parse("the *quick\nbrown* fox"), Ok(("", "quick\nbrown")));
    assert_eq!(
        parse("the*quick*fox"),
        Err(Err::Error(Error::new(
            "the*quick*fox",
            ErrorKind::TakeUntil
        )))
    );
    println!("Testing complete");
}

fn parse(source: &str) -> IResult<&str, &str> {
    let (source, _) = take_until(" *")(source)?;
    let (_, value) = delimited(tag(" *"), take_until("* "), tag("* "))(source)?;
    Ok(("", value))
}
