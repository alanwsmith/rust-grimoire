// #![allow(warnings)]

// use syntect::highlighting::ThemeSet;
// use syntect::html::highlighted_html_for_string;
// use syntect::parsing::SyntaxSet;

// fn main() {
//     let ss = SyntaxSet::load_defaults_newlines();
//     let ts = ThemeSet::load_defaults();
//     let syntax = ss.find_syntax_by_extension("rs").unwrap();
//     let source_text = String::from("<h1>Alfa</h1>");
//     let highlighted =
//         highlighted_html_for_string(&source_text, &ss, &syntax, &ts.themes["base16-ocean.dark"])
//             .unwrap();
//     dbg!(highlighted);
// }

//// This ones doesn't work, it was a work in progresss
//use syntect::highlighting::ThemeSet;
//use syntect::html::styled_line_to_highlighted_html;
//use syntect::parsing::SyntaxSet;
////
//fn main() {
//    let ss = SyntaxSet::load_defaults_newlines();
//    let ts = ThemeSet::load_defaults();
//    let syntax = ss.find_syntax_by_extension("rs").unwrap();
//    let source_text = String::from("<h1>Alfa</h1>");
//    let highlighted = styled_line_to_highlighted_html(
//        &source_text,
//        &ss,
//        &syntax,
//        &ts.themes["base16-ocean.dark"],
//    );
//    dbg!(highlighted);
//}

//  This does it without the pre tag.
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::html::{styled_line_to_highlighted_html, IncludeBackground};
use syntect::parsing::SyntaxSet;
//
fn main() {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_name("HTML").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    let regions = h.highlight_line("<h1>Alfa</h1>", &ps).unwrap();
    let html = styled_line_to_highlighted_html(&regions[..], IncludeBackground::No).unwrap();
    dbg!(html);
}

// use syntect::easy::HighlightLines;
// use syntect::highlighting::{Style, ThemeSet};
// use syntect::html::line_tokens_to_classed_spans;
// use syntect::html::{styled_line_to_highlighted_html, IncludeBackground};
// // use syntect::parsing::SyntaxSet;
// fn main() {
//     let ps = SyntaxSet::load_defaults_newlines();
//     let ts = ThemeSet::load_defaults();
//     let output = line_tokens_to_classed_spans("<h1>Alfa</h1>");
//     // let syntax = ps.find_syntax_by_name("Ruby").unwrap();
//     // let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
//     // let regions = h.highlight_line("5", &ps).unwrap();
//     // let html = styled_line_to_highlighted_html(&regions[..], IncludeBackground::No).unwrap();
//     // assert_eq!(html, "<span style=\"color:#d08770;\">5</span>");
//     println!("done");
// }

//use syntect::html::{ClassStyle, ClassedHTMLGenerator};
//use syntect::parsing::SyntaxSet;
//use syntect::util::LinesWithEndings;
////
//fn main() {
//    let current_code = r#"<h1>Alfa</h1>
//<p>Bravo</p>"#;
//    let syntax_set = SyntaxSet::load_defaults_newlines();
//    let syntax = syntax_set.find_syntax_by_name("HTML").unwrap();
//    let mut html_generator =
//        ClassedHTMLGenerator::new_with_class_style(syntax, &syntax_set, ClassStyle::Spaced);
//    for line in LinesWithEndings::from(current_code) {
//        html_generator.parse_html_for_line_which_includes_newline(line);
//    }
//    let output_html = html_generator.finalize();
//    dbg!(output_html);
//}
