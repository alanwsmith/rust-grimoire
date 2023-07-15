use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::character::complete::newline;
use nom::character::complete::space0;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::terminated;
use nom::sequence::tuple;

use nom::IResult;
use nom::Parser;

pub fn get_paragraphs(source: &str) -> IResult<&str, Vec<String>> {
    let (source, paragraphs) = many_till(paragraph, eof)(source)?;
    Ok((source, paragraphs.0))
}

pub fn paragraph(source: &str) -> IResult<&str, String> {
    let (source, p) = alt((
        terminated(take_until("\n"), tuple((newline, space0, newline)))
            .map(|s: &str| s.trim()),
        rest,
    ))(source.trim())?;
    Ok((source, p.to_string()))
}

#[cfg(test)]

mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "",
        vec![]
    )]
    #[case(
        "Draw the chart",
        vec![
            "Draw the chart".to_string()
        ]
    )]
    #[case(
        "Cap the jar\n\n",
        vec![
            "Cap the jar".to_string()
        ]
    )]
    #[case(
        "Let it settle\n\nDrop the ashes",
        vec![
            "Let it settle".to_string(), 
            "Drop the ashes".to_string()
        ]
    )]
    #[case(
        "Cut the pie\n\nDip the pail\n\n",
        vec![
            "Cut the pie".to_string(), 
            "Dip the pail".to_string()
        ]
    )]
    #[case(
    "Heavy black lines\n\n \nFasten two pins", 
        vec![
            "Heavy black lines".to_string(), 
            "Fasten two pins".to_string()
        ]
    )]
    #[case(
    "Fly by night\n \nDrive the nail", 
        vec![
            "Fly by night".to_string(), 
            "Drive the nail".to_string()
        ]
    )]
    #[case(
    "Hurdle the pit  \n\nLift the stone \n ",
        vec![
            "Hurdle the pit".to_string(),
            "Lift the stone".to_string()
        ]
    )]
    fn paragarphs_test(#[case] input: &str, #[case] expected: Vec<String>) {
        let results = get_paragraphs(input).unwrap().1;
        assert_eq!(expected, results);
    }
}
