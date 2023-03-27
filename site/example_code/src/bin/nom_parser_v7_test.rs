// This one was looking at making an AST
// for mark/org -> neo. Just a quick
// sketch. Not really complete.

#![allow(warnings)]
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::error::Error;
// use nom::error::ParseError;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

#[derive(Debug)]
enum Area {
    Base,
}

#[derive(Debug)]
struct Wagon {
    remainder: String,
    finalized: String,
    area: Area,
}

fn main() {
    let lines: Vec<&str> = vec!["", "- this is a list item", "# Some Title", "", "Some text"];
    let source = lines.join("\n");
    parse(source.as_str());
}

fn parse(source: &str) {
    let mut w = Wagon {
        remainder: source.to_string(),
        finalized: "".to_string(),
        area: Area::Base,
    };
    let (a, b) = crawler(&w).unwrap();
    dbg!(a);
    dbg!(b);
    w.remainder = a.to_string();
    dbg!(w);
}

fn crawler(w: &Wagon) -> IResult<&str, (&str, Area)> {
    let (remainder, captured) = alt((
        tuple((tag("\n# "), not_line_ending)).map(|x| ("a", Area::Base)),
        tuple((tag("\n- "), not_line_ending)).map(|x| ("a", Area::Base)),
        take(1u32).map(|x| ("a", Area::Base)),
    ))(w.remainder.as_str())?;

    dbg!(remainder);
    // dbg!(captured);

    Ok((remainder, captured))
}

// #[derive(Debug, Clone)]
// enum Area {
//     Base,
//     List,
//     // Done,
// }

// #[derive(Debug, Clone)]
// struct Wagon {
//     remainder: String,
//     finished: String,
//     area: Area,
// }

// fn main() {
//     println!("Parsing");
//     let lines: Vec<&str> = vec!["", "- this is a list item", "# Some Title", "", "Some text"];
//     let text = lines.join("\n");
//     parse(text);
// }

// fn parse(source: String) {
//     crawler();
//     // let mut w = Wagon {
//     //     remainder: source,
//     //     finished: "".to_string(),
//     //     area: Area::Base,
//     // };
//     // crawler(&mut w);
//     // dbg!(&w);
// }

// fn crawler<'a>() -> IResult<&'a str, &'a str> {
//     let mut w = Wagon {
//         remainder: "the quick brown fox".to_string(),
//         finished: "".to_string(),
//         area: Area::Base,
//     };
//     base(&mut w);
//     // let (remainder, status) = match w.area {
//     //     Area::Base => base(w),
//     //     _ => base(w),
//     // }?;
//     // dbg!(remainder);
//     // dbg!(status);
//     // w.remainder = remainder.to_string();
//     Ok(("", ""))
// }

// fn base<'a>(w: &mut Wagon) -> IResult<&'a str, &'a str> {
//     // let new_string = String::from(w.remainder.as_str());
//     let (remainder, status) = tag::<&str, &str, Error<&str>>("a")(w.remainder.as_str()).unwrap();
//     // alt::<ParseError<&str>, &str, &str, &str>>((take(1u32), eof))(new_string.as_str()).unwrap();
//     // let (remainder, status) = alt((
//     //     tuple((tag("\n# "), not_line_ending))
//     //         .map(|x| (format!("-> h1\n\n{}\n\n", x.1), Chunk::Base)),
//     //     tuple((tag("\n## "), not_line_ending))
//     //         .map(|x| (format!("-> h2\n\n{}\n\n", x.1), Chunk::Base)),
//     //     tag("\n- ").map(|x| (format!(""), Chunk::List)),
//     //     // take(1u32).map(|x| (format!("{}", x), Chunk::Base)),
//     //     take(1u32).map(|x| (format!("{}", x), Chunk::Base)),
//     //     eof.map(|x| (format!(""), Chunk::Done)),
//     // ))(w.remainder.to_string())?;
//     // dbg!(remainder);
//     // dbg!(&w.remainder);
//     // Ok((remainder, status));
//     Ok(("", ""))
// }

// fn crawler(source: &str, mut chunk: Chunk) -> IResult<&str, &str> {
//     let mut counter = 1;
//     let mut status: (&str, (&str, Chunk)) = ("", ("", Chunk::Base));
//     loop {
//         dbg!(counter);
//         dbg!(source);
//         let (source, taken) = match &chunk {
//             Chunk::Base => {
//                 println!("asdf");
//                 base(source, Chunk::Base)
//             }
//             _ => break,
//         }?;
//         dbg!(source);
//         chunk = taken.1;
//         // dbg!(taken);
//         // dbg!(source.len());
//         // dbg!(&taken);
//         // match taken.1 {
//         //     Done => break,
//         //     _ => (),
//         // }
//         if counter > 4 {
//             break;
//         }
//         counter += 1;
//     }
//     Ok(("", ""))
// }

//fn base(source: &str, chunk: Chunk) -> IResult<&str, (String, Chunk)> {
//    // dbg!(source);
//    let (remainder, status) = alt((
//        tuple((tag("\n# "), not_line_ending))
//            .map(|x| (format!("-> h1\n\n{}\n\n", x.1), Chunk::Base)),
//        tuple((tag("\n## "), not_line_ending))
//            .map(|x| (format!("-> h2\n\n{}\n\n", x.1), Chunk::Base)),
//        tag("\n- ").map(|x| (format!(""), Chunk::List)),
//        // take(1u32).map(|x| (format!("{}", x), Chunk::Base)),
//        take(1u32).map(|x| (format!("{}", x), Chunk::Base)),
//        eof.map(|x| (format!(""), Chunk::Done)),
//    ))(source)?;
//    //let (remainder, status) = tuple((tag("\n# "), not_line_ending))(source).map(|x| ("a", "b"))?;
//    // tuple((tag("\n# "), not_line_ending, rest)).map(|x| Chunk::Base)(source)?;
//    // tag("\n# ")(source)?;
//    // Ok(("", ("", Chunk::Base)))
//    // dbg!(&remainder);
//    // dbg!(&taken);
//    // dbg!(&status);
//    // dbg!(&remainder);
//    Ok((remainder, status))
//}

// fn crawler(source: &str) -> IResult<&str, &str> {
//     let (remainder, hit) = alt((
//         tuple((tag("\n# "), not_line_ending, rest)).map(|x| Widget::H1 {
//             hit: x.1,
//             remainder: x.2,
//         }),
//         tag("\n").map(|x| Widget::P),
//         take(1u32).map(|x| Widget::Placeholder),
//     ))(source)?;
//     let (remainder, hit) = match hit {
//         Widget::H1 { hit, remainder } => h1(hit, remainder),
//         _ => Ok(("", "")),
//     }?;
//     dbg!(&remainder);
//     dbg!(&hit);
//     Ok((remainder, hit))
// }

// fn h1<'a>(hit: &'a str, remainder: &'a str) -> IResult<&'a str, &'a str> {
//     // let (remainder, hit) = not_line_ending(hit)?;
//     Ok((remainder, hit))
// }

// #[derive(Debug)]
// enum Widget<'a> {
//     H1 { hit: &'a str, remainder: &'a str },
//     P,
//     Placeholder,
// }
