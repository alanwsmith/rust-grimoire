use nom::combinator::eof;
use nom::error::ErrorKind;
use nom::Err;
use nom::IResult;

// This shows the things you have to use
// order to do the error checks with nom
// They aren't shown in the examples.
// (I wouldn't be surprised to find a way
// to shorten things.)

fn main() {
    let parser = eof;

    assert_eq!(
        parser("alfa"),
        Err(Err::Error(("alfa", ErrorKind::Eof)))
    );

    assert_eq!(
        parse_outside("bravo"),
        Err(Err::Error(nom::error::Error {
            input: "bravo",
            code: ErrorKind::Eof
        }))
    )
}

fn parse_outside(s: &str) -> IResult<&str, &str> {
    eof(s)?;
    Ok(("never reached", "for this example"))
}
