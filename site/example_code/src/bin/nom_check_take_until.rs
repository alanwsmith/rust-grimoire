use nom::bytes::complete::take_until;

use nom::IResult;

fn main() {
    let (a, b) = check_it("alfa - bravo - charlie - delta").unwrap();
    dbg!(a);
    dbg!(b);
}

fn check_it(source: &str) -> IResult<&str, &str> {
    let (a, b) = take_until("-")(source)?;
    Ok((a, b))
}
