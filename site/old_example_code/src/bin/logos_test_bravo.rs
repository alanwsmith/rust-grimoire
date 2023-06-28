use logos::Logos;
// use logos::Span;

// Basic test, but looking at "Pest" which
// might assemble the AST too

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[token("-> title")]
    Title,

    #[regex(" +")]
    Space,

    #[regex("\t+")]
    Tabs,

    #[regex("\n")]
    Newline,

    #[regex("[a-zA-Z_0-9]+")]
    Word,

    #[error]
    Error,
}

// struct Page {
//     sections: Vec<Section>
// }

// struct Section {
//     kind: SectionTypek
// }

enum SectionType {
    Title,
    Content,
}

fn main() {
    let text = r#"-> title

This is the title

-> c

This is some content

With some more content

"#;

    // let mut lex = Token::lexer(text);
    // dbg!(lex.next());
    // dbg!(lex.slice());

    let tokens: Vec<(Token, _)> =
        Token::lexer(text).spanned().collect();

    // This doesn't show the actual text
    for token in tokens.iter() {
        dbg!(&token.0);
    }
    // Nor does this:
    // dbg!(Token::lexer(text).spanned());

    // It's just not in at that level which is
    // fine you just have to get it by referencing
    // it
}
