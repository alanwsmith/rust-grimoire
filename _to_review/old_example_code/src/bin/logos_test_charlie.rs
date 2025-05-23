use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[regex("[a-zA-Z]+")]
    Text,

    #[error]
    Error,
}

fn main() {
    let mut lex = Token::lexer("the quick brown fox");

    while let Some(token) = lex.next() {}
}
