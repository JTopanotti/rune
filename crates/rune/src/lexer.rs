use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(Debug, Copy, Clone, PartialEq, Logos, FromPrimitive, ToPrimitive)]
pub(crate) enum SyntaxKind {
    Root,

    BinaryOp,

    #[regex(" +")]
    Whitespace,

    #[token("fn")]
    FnKw,

    #[token("let")]
    LetKw,

    #[regex("[A-Za-z][A-Za-z0-9]+")]
    Identifier,

    #[regex("[0-9]+")]
    Number,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Asterisk,

    #[token("/")]
    Slash,

    #[token("=")]
    Equals,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[error]
    Err,
}

pub(crate) struct Lexer<'a> {
    inner: logos::Lexer<'a, SyntaxKind>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            inner: SyntaxKind::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (SyntaxKind, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let text = self.inner.slice();

        Some((kind, text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, kind: SyntaxKind) {
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some((kind, input)));
    }

    #[test]
    fn lex_spaces() {
        check("   ", SyntaxKind::Whitespace);
    }

    #[test]
    fn lex_fn_keyword() {
        check("fn", SyntaxKind::FnKw);
    }

    #[test]
    fn lex_let_keyword() {
        check("let", SyntaxKind::LetKw);
    }

    #[test]
    fn lex_alphabetic_identifier() {
        check("abcd", SyntaxKind::Identifier);
    }

    #[test]
    fn lex_alphanumeric_identifier() {
        check("ab123cde456", SyntaxKind::Identifier);
    }

    #[test]
    fn lex_mixed_case_identifier() {
        check("ABCdef", SyntaxKind::Identifier);
    }

    #[test]
    fn lex_number() {
        check("123456", SyntaxKind::Number);
    }

    #[test]
    fn lex_plus() {
        check("+", SyntaxKind::Plus);
    }

    #[test]
    fn lex_minus() {
        check("-", SyntaxKind::Minus);
    }

    #[test]
    fn lex_multiply() {
        check("*", SyntaxKind::Asterisk);
    }

    #[test]
    fn lex_divide() {
        check("/", SyntaxKind::Slash);
    }

    #[test]
    fn lex_equals() {
        check("=", SyntaxKind::Equals);
    }

    #[test]
    fn lex_left_brace() {
        check("{", SyntaxKind::LBrace);
    }

    #[test]
    fn lex_right_brace() {
        check("}", SyntaxKind::RBrace);
    }
}