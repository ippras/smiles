use crate::{
    errors::{Error, Result, SyntaxError},
    lexer::{Lexeme, Lexer},
    syntax::{
        SyntaxKind::{self, *},
        SyntaxNode,
    },
};
use itertools::{peek_nth, PeekNth};
use rowan::{GreenNode, GreenNodeBuilder};

/// Parser
pub struct Parser<'a> {
    /// input tokens
    // TODO: pub -> private
    pub lexer: PeekNth<Lexer<'a>>,
    /// the in-progress tree
    builder: GreenNodeBuilder<'static>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: peek_nth(Lexer::new(input)),
            builder: GreenNodeBuilder::new(),
        }
    }

    /// Advance one token, adding it to the current branch of the tree builder.
    fn bump(&mut self) {
        let lexeme = self.lexer.next().unwrap();
        self.builder.token(lexeme.kind.into(), &lexeme.text);
    }

    /// Error
    fn error(&mut self, expected: &'static [SyntaxKind]) -> Error {
        match self.lexer.next() {
            Some(lexeme) => Error::Syntax(SyntaxError {
                expected,
                found: lexeme,
            }),
            None => Error::Syntax(SyntaxError {
                expected,
                found: Lexeme {
                    kind: END_OF_STRING,
                    text: Default::default(),
                    range: Default::default(),
                },
            }),
        }
    }

    /// Peek unprocessed token
    fn peek(&mut self, index: usize) -> Option<SyntaxKind> {
        Some(self.lexer.peek_nth(index)?.kind)
    }

    // explicit implicit
    // serial, closure, branch
    // node, vertex, edges
    pub fn parse(mut self) -> Result<Parse> {
        self.builder.start_node(ROOT.into());
        self.node()?;
        self.builder.finish_node(); // ROOT
        Ok(Parse {
            green_node: self.builder.finish(),
        })
    }

    fn node(&mut self) -> Result<()> {
        self.builder.start_node(NODE.into());
        self.vertex()?;
        // if self.peek(0).is_some() && !(self.is_closure() || self.is_branch() || self.is_main()) {
        //     return Err(self.error(&[CLOSURE, BRANCH, MAIN]));
        // }
        if self.is_closure() || self.is_branch() || self.is_main() {
            self.edges()?;
        }
        // if self.peek(0).is_some() {
        //     return Err(self.error(&[CLOSURE, BRANCH, MAIN]));
        // }
        self.builder.finish_node(); // NODE
        Ok(())
    }

    fn vertex(&mut self) -> Result<()> {
        self.builder.start_node(VERTEX.into());
        match self.peek(0) {
            Some(LEFT_BRACKET) => self.brackets()?,
            Some(ASTERISK | IMPLICIT) => self.bump(),
            _ => return Err(self.error(&[ASTERISK, IMPLICIT, LEFT_BRACKET])),
        }
        self.builder.finish_node(); // VERTEX
        Ok(())
    }

    fn edges(&mut self) -> Result<()> {
        self.builder.start_node(EDGES.into());
        loop {
            if self.is_closure() {
                self.closure();
            } else if self.is_branch() {
                self.branch()?;
            } else {
                break self.main()?;
            }
        }
        self.builder.finish_node(); // EDGES
        Ok(())
    }

    fn is_branch(&mut self) -> bool {
        self.peek(0) == Some(LEFT_PAREN)
    }

    fn is_closure(&mut self) -> bool {
        self.peek(0) == Some(DIGIT) || (self.is_bound() && self.peek(1) == Some(DIGIT))
    }

    fn is_main(&mut self) -> bool {
        self.is_vertex(0) || (self.is_bound() && self.is_vertex(1))
    }

    fn is_vertex(&mut self, index: usize) -> bool {
        self.peek(index) == Some(LEFT_BRACKET)
            || matches!(self.peek(index), Some(ASTERISK | IMPLICIT))
    }

    fn is_bound(&mut self) -> bool {
        matches!(
            self.peek(0),
            Some(BACKSLASH | COLON | DOLLAR | EQUALS | MINUS | NUMBER | SLASH),
        )
    }

    fn closure(&mut self) {
        self.builder.start_node(CLOSURE.into());
        if self.is_bound() {
            self.bond(); // BOND
        }
        self.builder.start_node(INDEX.into());
        self.bump(); // DIGIT
        self.builder.finish_node(); // INDEX
        self.builder.finish_node(); // CLOSURE
    }

    fn branch(&mut self) -> Result<()> {
        self.builder.start_node(BRANCH.into());
        self.bump(); // LEFT_PAREN
        if self.is_bound() {
            self.bond(); // BOND
        }
        self.node()?;
        if self.peek(0) != Some(RIGHT_PAREN) {
            return Err(self.error(&[RIGHT_PAREN]));
        }
        self.bump(); // RIGHT_PAREN
        self.builder.finish_node(); // BRANCH
        Ok(())
    }

    fn main(&mut self) -> Result<()> {
        self.builder.start_node(MAIN.into());
        if self.is_bound() {
            self.bond(); // BOND
        }
        self.node()?; // NODE
        self.builder.finish_node(); // MAIN
        Ok(())
    }

    fn bond(&mut self) {
        self.builder.start_node(BOND.into());
        self.bump(); // BACKSLASH | COLON | DOLLAR | EQUALS | MINUS | NUMBER | SLASH
        self.builder.finish_node();
    }

    fn brackets(&mut self) -> Result<()> {
        self.builder.start_node(BRACKETS.into());
        self.bump(); // LEFT_BRACKET
        if let Some(DIGIT) = self.peek(0) {
            self.builder.start_node(ISOTOPE.into());
            self.unsigned();
            self.builder.finish_node(); // ISOTOPE
        }
        match self.peek(0) {
            Some(ASTERISK | EXPLICIT | H | IMPLICIT) => self.bump(),
            _ => return Err(self.error(&[ASTERISK, EXPLICIT, H, IMPLICIT])),
        }
        if let Some(AT) = self.peek(0) {
            self.builder.start_node(PARITY.into());
            self.bump(); // AT
            if let Some(AT) = self.peek(0) {
                self.bump(); // AT
            }
            self.builder.finish_node(); // PARITY
        }
        if let Some(H) = self.peek(0) {
            self.builder.start_node(HYDROGENS.into());
            self.bump(); // H
            if let Some(DIGIT) = self.peek(0) {
                self.unsigned();
            }
            self.builder.finish_node(); // HYDROGENS
        }
        if let Some(PLUS | MINUS) = self.peek(0) {
            self.builder.start_node(CHARGE.into());
            self.signed(); // SIGNED
            self.builder.finish_node(); // CHARGE
        }
        if let Some(COLON) = self.peek(0) {
            self.builder.start_node(CLASS.into());
            self.bump(); // COLON
            match self.peek(0) {
                Some(DIGIT) => self.unsigned(),
                _ => return Err(self.error(&[DIGIT])),
            }
            self.builder.finish_node(); // CLASS
        }
        if self.peek(0) != Some(RIGHT_BRACKET) {
            return Err(self.error(&[RIGHT_BRACKET]));
        }
        self.bump(); // RIGHT_BRACKET
        self.builder.finish_node();
        Ok(())
    }

    fn signed(&mut self) {
        self.builder.start_node(SIGNED.into());
        self.bump(); // PLUS | MINUS
        if let Some(DIGIT) = self.peek(0) {
            self.unsigned(); // UNSIGNED
        }
        self.builder.finish_node(); // SIGNED
    }

    fn unsigned(&mut self) {
        self.builder.start_node(UNSIGNED.into());
        while let Some(DIGIT) = self.peek(0) {
            self.bump(); // DIGIT
        }
        self.builder.finish_node(); // UNSIGNED
    }
}

/// Parse
pub struct Parse {
    green_node: GreenNode,
}

impl Parse {
    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }
}
