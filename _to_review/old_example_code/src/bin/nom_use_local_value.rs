use nom::bytes::complete::take_until;
use nom::IResult;

struct Widget {
    pub data: String,
    pub d2: Option<String>,
}

fn main() {
    let w = Widget {
        data: String::from("alfa bravo charlie"),
        d2: Some(String::from("delta echo foxtrot")),
    };
    let _results = parse(&w);
}

fn parse(w: &Widget) -> IResult<&str, &str> {
    let (_a, _b) = take_until("bravo")(w.data.as_str())?;
    let (_a, _b) = take_until("bravo")(w.d2.as_ref().unwrap().as_str())?;
    dbg!(&w.data);
    Ok(("", ""))
}
