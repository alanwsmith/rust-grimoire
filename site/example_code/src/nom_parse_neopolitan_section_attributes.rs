use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

pub fn get_attributes(source: &str) -> IResult<&str, String> {
    let (a, b) = many0(tuple((tag(">>"), not_line_ending, line_ending)).map(|x| x.1))(source)?;
    dbg!(&b);
    dbg!(&a);
    Ok(("", format!("<p{}>{}</p>", b.join(""), a.trim())))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn attr_test() {
        let source = vec![">> class: alfa", ">> id: bravo", "", "some content"];
        assert_eq!(
            get_attributes(source.join("\n").as_str()).unwrap().1,
            format!(r#"<p class: alfa id: bravo>some content</p>"#)
        )
    }
}
