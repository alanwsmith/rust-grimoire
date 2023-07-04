#![allow(unused_imports)]
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::combinator::opt;
use nom::combinator::rest;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

pub fn attributes(
    source: &str,
) -> IResult<&str, Vec<(&str, &str)>> {
    let (a, b) = opt(tuple((
        tag(">> "),
        separated_list0(
            tag(">>"),
            alt((take_until(">>"), rest)),
        ), //.map(|x| {
           // tuple((take_until(": "), tag(": "), rest))
           //  .map(|r| vec![r.0, r.2])
           //        }),
    )))(source)?;

    // dbg!(&a);
    // dbg!(&b);

    let delta = b
        .unwrap()
        .1
        .into_iter()
        .map(|x| {
            // dbg!(x.split(":").collect::<Vec<&str>>());
            x.split(":").collect::<Vec<&str>>()

            //.map(|w| {
            //dbg!(w)
            ////(w[0: usize].trim(), w[1: usize].trim())
            //})
        })
        .collect::<Vec<Vec<&str>>>();

    // dbg!(&delta);
    let echo = delta
        .iter()
        .map(|r| (r[0].trim(), r[1].trim()))
        .collect::<Vec<(&str, &str)>>();
    // dbg!(&echo);

    // let (a, b) =
    //     tuple((take_until(": "), tag(": "), rest))(
    //         b.unwrap().1[0],
    //     )?;

    // let thing = (b.unwrap().1, b.unwrap().4);
    // dbg!(&a);
    // dbg!(&b);

    //
    // Ok((a, vec![]))
    Ok((a, echo))

    //
}

#[cfg(test)]

mod test {

    use crate::nom_parse_optional_neopolitan_attributes::attributes;

    #[test]
    pub fn basic_attr_test() {
        let lines = [">> class: echo"].join("\n");
        assert_eq!(
            attributes(lines.as_str()),
            Ok(("", vec![("class", "echo")]))
        );
    }

    #[test]
    pub fn with_two_attirbutes() {
        let lines =
            [">> class: foxtrot >> id: charlie"].join("\n");
        assert_eq!(
            attributes(lines.as_str()),
            Ok((
                "",
                vec![
                    ("class", "foxtrot"),
                    ("id", "charlie")
                ]
            ))
        );
    }

    //
}
