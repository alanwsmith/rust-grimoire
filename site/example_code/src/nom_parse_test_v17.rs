use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::alpha1;
use nom::character::complete::multispace0;
use nom::multi::many1;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum Section {
    H2(Vec<crate::nom_parse_test_v17::Content>),
    Title(Vec<crate::nom_parse_test_v17::Content>),
    P(Vec<crate::nom_parse_test_v17::Content>),
    None,
}

#[derive(Debug, PartialEq)]
pub enum Content {
    Title(String),
    Paragraph(String),
}

pub fn title_content(
    source: &str,
) -> IResult<&str, Section> {
    let (source, _) = multispace0(source)?;
    let (source, captured) = alpha1(source)?;
    let (source, _) = multispace0(source)?;
    dbg!(&source);
    Ok((
        source,
        Section::Title(vec![Content::Title(
            captured.to_string(),
        )]),
    ))
}

pub fn h2_content(source: &str) -> IResult<&str, Section> {
    let (source, _) = multispace0(source)?;
    let (source, captured) = alpha1(source)?;
    let (source, _) = multispace0(source)?;
    dbg!(&source);
    Ok((
        source,
        Section::H2(vec![Content::Title(
            captured.to_string(),
        )]),
    ))
}

pub fn parse(source: &str) -> IResult<&str, Vec<Section>> {
    let (source, captured) = many1(preceded(
        tag("-> "),
        alt((
            tuple((tag_no_case("title"), title_content)),
            tuple((tag_no_case("h2"), h2_content)),
            // tuple((tag_no_case("p"), content)),
        )),
    ))(source)?;

    // match captured[0] {
    //     "title" => {}
    //     "h2" => {}
    //     "p" => {}
    //     _ => {}
    // }

    dbg!(&captured);
    dbg!(&source);

    let payload = captured
        .into_iter()
        .map(|(_, y)| y)
        .collect::<Vec<Section>>();

    dbg!(&payload);
    Ok((source, payload))

    // Ok((
    //     source,
    //     vec![
    //         Section::Title(vec![Content::Title(
    //             "Alfa".to_string(),
    //         )]),
    //         Section::H2(vec![Content::Title(
    //             "Charlie".to_string(),
    //         )]),
    //         // Section::P(vec![
    //         //     Content::Paragraph(
    //         //         "Echo Foxtrot".to_string(),
    //         //     ),
    //         //     Content::Paragraph(
    //         //         "Golf Hotel".to_string(),
    //         //     ),
    //         // ]),
    //     ],
    // ))

    //
}

#[cfg(test)]

mod test {

    use super::*;

    #[test]
    pub fn solo_test_1() {
        let lines = vec![
            "-> title", "", "Alfa", "", "-> h2", "",
            "Charlie",
        ]
        .join("\n");
        let expected = vec![
            Section::Title(vec![Content::Title(
                "Alfa".to_string(),
            )]),
            Section::H2(vec![Content::Title(
                "Charlie".to_string(),
            )]),
            // Section::P(vec![
            //     Content::Paragraph(
            //         "Echo Foxtrot".to_string(),
            //     ),
            //     Content::Paragraph(
            //         "Golf Hotel".to_string(),
            //     ),
            // ]),
        ];
        assert_eq!(
            expected,
            parse(lines.as_str()).unwrap().1
        );
    }
}
