use logos::Logos;

// This is just the example taken straight from the
// docs
// https://crates.io/crates/logos

#[derive(Logos, Debug, PartialEq)]
enum Token {
    // Tokens can be literal strings, of any length.
    #[token("fast")]
    Fast,

    #[token(".")]
    Period,

    // Or regular expressions.
    #[regex("[a-zA-Z]+")]
    Text,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

fn main() {
    let mut lex =
        Token::lexer("Create ridiculously fast Lexers.");

    assert_eq!(lex.next(), Some(Token::Text));
    assert_eq!(lex.span(), 0..6);
    assert_eq!(lex.slice(), "Create");

    assert_eq!(lex.next(), Some(Token::Text));
    assert_eq!(lex.span(), 7..19);
    assert_eq!(lex.slice(), "ridiculously");

    assert_eq!(lex.next(), Some(Token::Fast));
    assert_eq!(lex.span(), 20..24);
    assert_eq!(lex.slice(), "fast");

    assert_eq!(lex.next(), Some(Token::Text));
    assert_eq!(lex.span(), 25..31);
    assert_eq!(lex.slice(), "Lexers");

    assert_eq!(lex.next(), Some(Token::Period));
    assert_eq!(lex.span(), 31..32);
    assert_eq!(lex.slice(), ".");

    assert_eq!(lex.next(), None);
}
