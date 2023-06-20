use nom::IResult;
use nom::bytes::complete::is_not;

fn main() {
    test_space();
    test_tab();
    test_newline();
    println!("All tests passed");
}

fn parse(source: &str) -> IResult<&str, &str> {
    let (a, b) = is_not(" \t\n")(source)?;
    Ok((a, b))
}

fn test_space() {
    let expected = "alfa_bravo";
    let (_, result) = parse("alfa_bravo charlie").unwrap();
    assert_eq!(expected, result);
}

fn test_tab() {
    let expected = "delta_echo";
    let (_, result) = parse("delta_echo\tfoxtrot").unwrap();
    assert_eq!(expected, result);
}

fn test_newline() {
    let expected = "golf_hotel";
    let (_, result) = parse("golf_hotel\nindia").unwrap();
    assert_eq!(expected, result);
}


