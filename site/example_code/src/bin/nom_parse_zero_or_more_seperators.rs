use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::multi::many0;
use nom::IResult;

fn main() {
    assert_eq!(get_parts("").unwrap().1, None);
    assert_eq!(get_parts("a").unwrap().1, Some(vec!["a"]));
    assert_eq!(get_parts("a|b").unwrap().1, Some(vec!["a", "b"]));
    assert_eq!(get_parts("a|b|c").unwrap().1, Some(vec!["a", "b", "c"]));
    assert_eq!(get_parts("a|b|").unwrap().1, Some(vec!["a", "b", ""]));
    assert_eq!(get_parts("|a|b").unwrap().1, Some(vec!["", "a", "b"]));
}

fn get_parts(source: &str) -> IResult<&str, Option<Vec<&str>>> {
    if source.is_empty() {
        Ok(("", None))
    } else {
        let (remainder, mut parts) = many0(part)(source)?;
        parts.push(remainder);
        Ok(("", Some(parts)))
    }
}

fn part(source: &str) -> IResult<&str, &str> {
    let (source, content) = take_until("|")(source)?;
    let (source, _) = tag("|")(source)?;
    Ok((source, content))
}

// fn get_parts(source: &str) -> IResult<&str, Option<Vec<&str>>> {
//     let (remainder, mut parts) = many0(part)(source)?;
//     parts.push(remainder);
//     Ok(("", Some(parts)))
// }
