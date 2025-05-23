// This is for when you don't want to have the
// error automatically float up in a way that
// requires making the functions return
// an IResult. This helps where it won't
// let you use a value because it's
// owned by a function that would make it
// disappear when the function is finished.
//
// Note: I'm not sure what the error
// types are and they should probably be
// handled better.

use nom::bytes::complete::tag;
use nom::error::Error;

#[derive(Debug)]
struct Widget {
    text: String,
}

fn main() {
    let mut w = Widget {
        text: "the quick brown fox".to_string(),
    };
    parse(&mut w);
    dbg!(w);
}

fn parse(w: &mut Widget) {
    let response = tag::<&str, &str, Error<&str>>("the quick ")(w.text.as_str()).unwrap();
    w.text = response.0.to_string();
}
