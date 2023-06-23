use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

fn tags(source: &str) -> IResult<&str, String> {
    let (a, b) = many0(
        tuple((take_until(" <<"), tag(" <<"), take_until(">> "), tag(">> "))).map(
            |(preface, _, payload, _)| match attributes(preface, payload) {
                Ok((_, y)) => format!("{}", y),
                Err(_) => format!("aaaaaaaaa"),
            },
        ),
    )(source)?;
    Ok(("", format!("{}{}", b.join(""), a)))
}

fn attributes<'a>(preface: &'a str, payload: &'a str) -> IResult<&'a str, String> {
    let (a, b) = tuple((take_until("|"), tag("|")))(payload)?;
    Ok(("", format!("{} <{}>{}</{}> ", preface, a, b.0, a)))
}

#[cfg(test)]
mod test {

    use crate::nom_parse_multiple_neopolitan_tags::tags;

    #[test]
    pub fn tag_test() {
        assert_eq!(
            tags("alfa <<bravo|strong>> charlie <<delta|strong>> echo"),
            Ok((
                "",
                format!("alfa <strong>bravo</strong> charlie <strong>delta</strong> echo")
            )),
        )
    }

    //
}
