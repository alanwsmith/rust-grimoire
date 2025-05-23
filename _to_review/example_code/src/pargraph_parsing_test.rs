fn parse_it(source: &str) -> Vec<String> {
    let content = source.lines().fold(
        vec![String::from("")],
        |mut acc, x| {
            if x.is_empty() {
                acc.push("".to_string())
            } else {
                if !acc.last().unwrap().is_empty() {
                    acc.last_mut().unwrap().push_str(" ");
                }
                acc.last_mut().unwrap().push_str(x);
            }
            acc
        },
    );

    // source.lines().for_each(|x| {
    //     if x.is_empty() {
    //         target.push("".to_string())
    //     } else {
    //         target.last_mut().unwrap().push_str(x);
    //     }
    //     ()
    // });

    content
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    pub fn test_paragraphs() {
        let lines = vec![
            "Slide the tray",
            "on the glass top",
            "",
            "Polish the glass",
            "",
        ];
        let target: Vec<String> = vec![
            "Slide the tray on the glass top".to_string(),
            "Polish the glass".to_string(),
        ];
        assert_eq!(
            parse_it(lines.join("\n").as_str()),
            target
        );
    }
}
