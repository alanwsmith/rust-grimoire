use nom::bytes::complete::take_until;
use nom::IResult;

// NOTE: Not sure this is the best way to do
// this. Guessing it might be better to
// return the IResult and handle it via
// Err instead of None.

fn main() {
    let output = grab_until();
    dbg!(output);
}

fn grab_until() -> Option<String> {
    let text = "the quick brown fox";

    let search = "quick";

    let result: IResult<&str, &str> =
        take_until(search)(text);



    match result {
        Ok(value) => Some(String::from(value.0)),
        Err(_) => None,
    }
}
