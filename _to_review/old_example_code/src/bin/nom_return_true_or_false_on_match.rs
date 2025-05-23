use nom::bytes::complete::tag_no_case;

fn main() {
    assert_eq!(false, matches("brown dog"));
    assert_eq!(true, matches("red fox"));
}

fn matches(source: &str) -> bool {
    let check = tag_no_case::<&str, &str, ()>("red")(source).ok();
    match check {
        None => false,
        Some(_) => true,
    }
}
