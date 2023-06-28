use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

#[derive(Debug)]
enum Area {
    Base,
}

fn main() {
    let lines: Vec<&str> = vec!["", "# Some Title", "", "- Some list", "", "More text"];
    let source = lines.join("\n");
    parse(source.as_str());
}

fn parse(mut source: &str) {
    let mut payload = ("".to_string(), Area::Base);
    let mut output = "".to_string();
    loop {
        (source, payload) = crawler(source, payload.1).unwrap();
        output.push_str(payload.0.as_str());
        if source == "" {
            break;
        }
    }
    dbg!(output);
}

fn crawler(source: &str, area: Area) -> IResult<&str, (String, Area)> {
    match area {
        Area::Base => {
            let (a, b) = alt((
                tuple((tag("\n# "), not_line_ending))
                    .map(|x| (format!("-> title\n\n{}", x.1), Area::Base)),
                tuple((tag("\n- "), not_line_ending))
                    .map(|x| (format!("-> list\n\n{}", x.1), Area::Base)),
                take(1u32).map(|x| (format!("{}", x), Area::Base)),
                eof.map(|_| (format!(""), Area::Base)),
            ))(source)?;
            Ok((a, b))
        }
    }
}
