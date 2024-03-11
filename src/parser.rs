use crate::{
    errors::{Error, Result, SyntaxError},
    lexer::Lexer,
    syntax::{SyntaxKind, SyntaxKind::*, SyntaxNode},
};
use itertools::{peek_nth, PeekNth};
use rowan::{GreenNode, GreenNodeBuilder};

/// Parser
pub struct Parser<'a> {
    /// input tokens
    // TODO
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

    // fn error(&mut self, expected: &'static [SyntaxKind]) {
    //     if let Some(lexeme) = self.lexer.next() {
    //         self.builder.token(ERROR.into(), &lexeme.text);
    //         self.errors.push(SyntaxError {
    //             expected,
    //             found: lexeme,
    //         });
    //     }
    // }
    /// Error
    fn error(&mut self, expected: &'static [SyntaxKind]) -> Error {
        match self.lexer.next() {
            Some(lexeme) => Error::Syntax(SyntaxError {
                expected,
                found: lexeme,
            }),
            None => Error::EndOfString,
        }
    }

    /// Peek unprocessed token
    fn peek(&mut self, index: usize) -> Option<SyntaxKind> {
        Some(self.lexer.peek_nth(index)?.kind)
    }

    // explicit implicit
    // serial, closure, branch
    pub fn parse(mut self) -> Result<Parse> {
        self.builder.start_node(ROOT.into());
        self.node()?;
        self.builder.finish_node(); // ROOT
        Ok(Parse {
            green_node: self.builder.finish(),
        })
    }

    fn node(&mut self) -> Result<()> {
        let checkpoint = self.builder.checkpoint();
        match self.peek(0) {
            Some(LEFT_BRACKET) => self.brackets()?,
            Some(ASTERISK | IMPLICIT) => self.bump(),
            _ => return Err(self.error(&[ASTERISK, BRACKETS, IMPLICIT])),
        }
        self.builder.start_node_at(checkpoint, NODE.into());
        self.builder.start_node_at(checkpoint, VERTEX.into());
        self.builder.finish_node(); // VERTEX
        self.edges()?;
        self.builder.finish_node(); // NODE
        Ok(())
    }

    fn edges(&mut self) -> Result<()> {
        self.builder.start_node(BONDS.into());
        // Closures
        loop {
            match self.peek(0) {
                Some(DIGIT) => {
                    self.builder.start_node(CLOSURE.into());
                    self.builder.start_node(INDEX.into());
                    self.bump(); // DIGIT
                    self.builder.finish_node(); // INDEX
                    self.builder.finish_node(); // CLOSURE
                }
                Some(BACKSLASH | COLON | DOLLAR | EQUALS | MINUS | NUMBER | SLASH)
                    if self.peek(1) == Some(DIGIT) =>
                {
                    self.builder.start_node(CLOSURE.into());
                    self.bond(); // BOND
                    self.builder.start_node(INDEX.into());
                    self.bump(); // DIGIT
                    self.builder.finish_node(); // INDEX
                    self.builder.finish_node(); // CLOSURE
                }
                _ => break,
            }
        }
        // Branches
        while let Some(LEFT_PAREN) = self.peek(0) {
            self.branch()?;
        }
        // Serial
        let checkpoint = self.builder.checkpoint();
        if let Some(BACKSLASH | COLON | DOLLAR | EQUALS | MINUS | NUMBER | SLASH) = self.peek(0) {
            self.bond(); // BOND
        }
        if let Some(ASTERISK | IMPLICIT) = self.peek(0) {
            self.builder.start_node_at(checkpoint, SERIAL.into());
            self.node()?;
            self.builder.finish_node(); // SERIAL
        }
        self.builder.finish_node(); // BONDS
        Ok(())
    }

    fn bond(&mut self) {
        self.builder.start_node(BOND.into());
        self.bump(); // BACKSLASH | COLON | DOLLAR | EQUALS | MINUS | NUMBER | SLASH
        self.builder.finish_node();
    }

    fn branch(&mut self) -> Result<()> {
        self.builder.start_node(BRANCH.into());
        self.bump(); // LEFT_PAREN
        if let Some(BACKSLASH | COLON | DOLLAR | EQUALS | MINUS | NUMBER | SLASH) = self.peek(0) {
            self.bond(); // BOND
        }
        self.node()?;
        self.bump(); // RIGHT_PAREN
        self.builder.finish_node(); // BRANCH
        Ok(())
    }

    fn brackets(&mut self) -> Result<()> {
        self.builder.start_node(BRACKETS.into());
        self.bump(); // LEFT_BRACKET
        if let Some(DIGIT) = self.peek(0) {
            self.isotope();
        }
        match self.peek(0) {
            Some(ASTERISK | EXPLICIT | H | IMPLICIT) => self.bump(),
            _ => return Err(self.error(&[ASTERISK, EXPLICIT, H, IMPLICIT])),
        }
        if let Some(AT) = self.peek(0) {
            self.parity();
        }
        if let Some(H) = self.peek(0) {
            self.hydrogens();
        }
        if let Some(PLUS | MINUS) = self.peek(0) {
            self.charge();
        }
        if let Some(COLON) = self.peek(0) {
            self.class()?;
        }
        if self.peek(0) != Some(RIGHT_BRACKET) {
            return Err(self.error(&[RIGHT_BRACKET]));
        }
        self.bump(); // RIGHT_BRACKET
        self.builder.finish_node();
        Ok(())
    }

    fn isotope(&mut self) {
        self.builder.start_node(ISOTOPE.into());
        self.unsigned();
        self.builder.finish_node(); // ISOTOPE
    }

    fn parity(&mut self) {
        self.builder.start_node(PARITY.into());
        self.bump(); // AT
        if let Some(AT) = self.peek(0) {
            self.bump(); // AT
        }
        self.builder.finish_node(); // PARITY
    }

    fn hydrogens(&mut self) {
        self.builder.start_node(HYDROGENS.into());
        self.bump(); // H
        if let Some(DIGIT) = self.peek(0) {
            self.unsigned();
        }
        self.builder.finish_node(); // HYDROGENS
    }

    fn charge(&mut self) {
        self.builder.start_node(CHARGE.into());
        self.signed(); // SIGNED
        self.builder.finish_node(); // CHARGE
    }

    fn class(&mut self) -> Result<()> {
        self.builder.start_node(CLASS.into());
        self.bump(); // COLON
        match self.peek(0) {
            Some(DIGIT) => self.unsigned(),
            _ => return Err(self.error(&[DIGIT])),
        }
        self.builder.finish_node(); // CLASS
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

    // fn bump(&mut self) {
    //     if let Some(Ok(token)) = self.lexer.next() {
    //         self.builder.token(token.into(), self.lexer.slice());
    //     }
    // }

    // fn list(&mut self) {
    //     assert_eq!(self.current(), Some(RightParen));
    //
    //     self.builder.start_node(LIST.into());
    //     self.bump(); // '('
    //     loop {
    //         match self.sexp() {
    //             SexpRes::Eof => {
    //                 self.errors.push("expected `)`".to_string());
    //                 break;
    //             }
    //             SexpRes::RParen => {
    //                 self.bump();
    //                 break;
    //             }
    //             SexpRes::Ok => (),
    //         }
    //     }
    //     // close the list node
    //     self.builder.finish_node();
    // }

    // fn sexp(&mut self) -> SexpRes {
    //     // Eat leading whitespace
    //     self.skip_ws();
    //     // Either a list, an atom, a closing paren,
    //     // or an eof.
    //     let t = match self.current() {
    //         None => return SexpRes::Eof,
    //         Some(R_PAREN) => return SexpRes::RParen,
    //         Some(t) => t,
    //     };
    //     match t {
    //         L_PAREN => self.list(),
    //         WORD => {
    //             self.builder.start_node(ATOM.into());
    //             self.bump();
    //             self.builder.finish_node();
    //         }
    //         ERROR => self.bump(),
    //         _ => unreachable!(),
    //     }
    //     SexpRes::Ok
    // }

    // fn skip_ws(&mut self) {
    //     while self.current() == Some(WHITESPACE) {
    //         self.bump()
    //     }
    // }
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
