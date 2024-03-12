use crate::syntax::SyntaxKind;
use logos::Logos;
use rowan::{TextRange, TextSize};
use smol_str::SmolStr;
use std::{
    fmt::{Display, Formatter, Result},
    ops::Range,
};

/// Lexer
#[derive(Clone, Debug)]
pub(crate) struct Lexer<'a> {
    lexer: logos::Lexer<'a, SyntaxKind>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            lexer: SyntaxKind::lexer(input),
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Lexeme;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.lexer.next()?.unwrap();
        let text = SmolStr::from(self.lexer.slice());
        let Range { start, end } = self.lexer.span();
        let start = TextSize::try_from(start).unwrap();
        let end = TextSize::try_from(end).unwrap();
        let range = TextRange::new(start, end);
        Some(Lexeme { kind, text, range })
    }
}

/// Lexeme
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Lexeme {
    pub(crate) kind: SyntaxKind,
    pub(crate) text: SmolStr,
    pub(crate) range: TextRange,
}

impl Display for Lexeme {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let Self { kind, range, text } = self;
        write!(f, "{{ kind: {kind:?}, range: {range:?}, text: {text:?} }}")
    }
}
