// use nom::branch::alt;
use nom::bytes::complete::tag;
// use nom::bytes::complete::take_until;
// use nom::combinator::opt;
// use nom::combinator::rest;
// use nom::multi::many0;
// use nom::multi::separated_list0;
use nom::bytes::complete::is_not;
use nom::multi::separated_list1;
use nom::sequence::delimited;
// use nom::sequence::tuple;
use nom::combinator::verify;
use nom::IResult;
// use nom::Parser;

// fn cutit(source: &str) -> IResult<&str, String> {
//     let (a, b) = tuple((
//         // separated_list1(tag("|"), take_until("|")),
//         separated_list1(take_until("|"), tag("|")),
//         alt((rest, rest)),
//     ))(source)?;
//     dbg!(b);
//     dbg!(a);
//     Ok(("", format!("")))
// }

fn neotag(source: &str) -> IResult<&str, String> {
    let (a, b) = verify(
        delimited(
            tag("<<"),
            separated_list1(tag("|"), is_not("|>")),
            tag(">>"),
        ),
        |v: &Vec<&str>| v.len() > 1,
    )(source)?;
    Ok((a, format!("<{}>{}</{}>", b[1], b[0], b[1])))
}

fn content(source: &str) -> IResult<&str, String> {
    // dbg!(&a);
    // dbg!(&b);

    // let (a, b) = many0(
    //     tuple((
    //         take_until(" <<"),
    //         tag(" <<"),
    //         take_until(">>").map(|x| cutit(x)),
    //         tag(">>"),
    //     )), // .map(
    //         //     |(preface, _, text, _, payload)| match attributes(preface, payload) {
    //         //         Ok((_, y)) => {
    //         //             dbg!(&preface);
    //         //             dbg!(&text);
    //         //             dbg!(&payload);
    //         //             format!("{}", y)
    //         //         }
    //         //         Err(_) => format!("aaaaaaaaa"),
    //         //     },
    //         // ),
    // )(source)?;
    // // dbg!(&a);
    // // dbg!(&b);
    // Ok(("", format!("")))

    Ok(("", format!("")))
}

// fn attributes<'a>(preface: &'a str, payload: &'a str) -> IResult<&'a str, String> {
//     // dbg!(preface);
//     // dbg!(payload);
//     let (a, b) = many0(tuple((take_until("|"), tag("|"))))(payload)?;
//     // dbg!(&a);
//     // dbg!(&b);
//     match a {
//         // "link" => Ok(("", format!(r#"{} <a>{}</a> "#, preface, b.0))),
//         "link" => {
//             dbg!("DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD");
//             Ok(("", format!(r#"{} <a>{}</a> "#, preface, b.0)))
//         }
//         "strong" => Ok(("", format!(r#"{} <strong>{}</strong> "#, preface, b.0))),
//         _ => Ok(("", format!(""))),
//     }
//     Ok(("", format!("ATTRIBUTES")))
// }

// fn attributes<'a>(preface: &'a str, payload: &'a str) -> IResult<&'a str, String> {
//     let (a, b) = tuple((take_until("|"), tag("|")))(payload)?;
//     match a {
//         "link" => Ok(("", format!("{} <link>{}</link> ", preface, b.0))),
//         "strong" => Ok(("", format!("{} <strong>{}</strong> ", preface, b.0))),
//         _ => Ok(("", format!(""))),
//     }
// }

#[cfg(test)]
mod test {

    use super::*;
    // use crate::nom_parse_multiple_neopolitan_tags::tags;

    #[test]
    pub fn strong_tag() {
        assert_eq!(
            neotag("<<bravo|strong>>"),
            Ok(("", format!("<strong>bravo</strong>"))),
        )
    }

    // #[test]
    // pub fn link_tags() {
    //     assert_eq!(
    //         tags("alfa <<bravo|link|https://www.example.com/>> charlie <<delta|link|https://www.alanwsmith.com/>> echo"),
    //         Ok((
    //             "",
    //             format!("alfa <link>bravo</link> charlie <link>delta</link> echo")
    //         )),
    //     )
    // }

    // #[test]
    // fn it_works() {
    //     let input = "<<alfa|strong>>";
    //     assert_eq!(tag(input).unwrap().1, vec!["alfa", "strong"]);
    // }

    // #[test]
    // fn it_works2() {
    //     let input = "<<bravo|link|https://localhost.com/>>";
    //     assert_eq!(
    //         tag(input).unwrap().1,
    //         vec!["bravo", "link", "https://localhost.com/"]
    //     );
    // }

    // #[test]
    // fn it_works3() {
    //     let input = "<<charlie|link|https://localhost.com/|class: whatever>>";
    //     assert_eq!(
    //         tag(input).unwrap().1,
    //         vec![
    //             "charlie",
    //             "link",
    //             "https://localhost.com/",
    //             "class: whatever"
    //         ]
    //     );
    // }

    // #[test]
    // fn it_works4() {
    //     let input = "<<delta|emd|class: more>>";
    //     assert_eq!(tag(input).unwrap().1, vec!["delta", "emd", "class: more"]);
    // }

    //
}
