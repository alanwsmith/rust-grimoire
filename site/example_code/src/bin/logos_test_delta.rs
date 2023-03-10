use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token<'a> {
    #[regex("-> TITLE")]
    TitleTag,

    #[regex(r"\n")]
    Newline,

    #[regex(r" ")]
    Space,

    #[regex(r"\w+")]
    Word(&'a str),

    #[error]
    Error,
}

fn main() {
    let lex: Vec<_> =
        Token::lexer("-> TITLE\n\nquick brown fox")
            .spanned()
            .collect();
    dbg!(lex);
}
