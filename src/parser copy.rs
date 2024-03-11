use crate::{
    errors::{Error, SyntaxError},
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
    /// the list of syntax errors
    errors: Vec<SyntaxError>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: peek_nth(Lexer::new(input)),
            builder: GreenNodeBuilder::new(),
            errors: Vec::new(),
        }
    }

    /// Advance one token, adding it to the current branch of the tree builder.
    fn bump(&mut self) {
        let lexeme = self.lexer.next().unwrap();
        self.builder.token(lexeme.kind.into(), &lexeme.text);
    }

    /// Error
    fn error(&mut self, expected: &'static [SyntaxKind]) {
        if let Some(lexeme) = self.lexer.next() {
            self.builder.token(ERROR.into(), &lexeme.text);
            self.errors.push(SyntaxError {
                expected,
                found: lexeme,
            });
        }
    }

    /// Peek unprocessed token
    fn peek(&mut self, index: usize) -> Option<SyntaxKind> {
        Some(self.lexer.peek_nth(index)?.kind)
    }

    // explicit implicit
    // serial, closure, branch
    pub fn parse(mut self) -> Parse {
        self.builder.start_node(ROOT.into());
        self.node();
        self.builder.finish_node(); // ROOT
        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }

    fn node(&mut self) -> Result<(), Error> {
        let checkpoint = self.builder.checkpoint();
        match self.peek(0) {
            Some(LEFT_BRACKET) => self.brackets(),
            Some(ASTERISK | IMPLICIT) => self.bump(),
            _ => return Err(self.error(&[ASTERISK, BRACKETS, IMPLICIT])),
        }
        self.builder.start_node_at(checkpoint, NODE.into());
        self.builder.start_node_at(checkpoint, VERTEX.into());
        self.builder.finish_node(); // VERTEX
        self.edges();
        self.builder.finish_node(); // NODE
    }

    fn edges(&mut self) {
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
            self.branch();
        }
        // Serial
        let checkpoint = self.builder.checkpoint();
        self.try_bond();
        // if let Some(BACKSLASH | COLON | DOLLAR | EQUALS | MINUS | NUMBER | SLASH) = self.peek(0) {
        //     checkpoint = Some(self.builder.checkpoint());
        //     self.bond(); // BOND
        // }
        if let Some(ASTERISK | IMPLICIT) = self.peek(0) {
            self.builder.start_node_at(checkpoint, SERIAL.into());
            self.node();
            self.builder.finish_node(); // SERIAL
        }
        self.builder.finish_node(); // BONDS
    }

    // bridge ::= nonzero | "%" nonzero digit

    fn try_bond(&mut self) -> Option<SyntaxKind> {
        match self.peek(0)? {
            token @ (BACKSLASH | COLON | DOLLAR | EQUALS | MINUS | NUMBER | SLASH) => {
                self.bond(); // BACKSLASH | COLON | DOLLAR | EQUALS | MINUS | NUMBER | SLASH
                Some(token)
            }
            _ => None,
        }
    }

    fn bond(&mut self) {
        self.builder.start_node(BOND.into());
        self.bump(); // BACKSLASH | COLON | DOLLAR | EQUALS | MINUS | NUMBER | SLASH
        self.builder.finish_node();
    }

    fn branch(&mut self) {
        self.builder.start_node(BRANCH.into());
        self.bump(); // LEFT_PAREN

        self.try_bond();
        self.node();
        // match self.peek(0) {
        //     Some(LEFT_BRACKET) => self.brackets(),
        //     Some(ASTERISK | IMPLICIT) => self.bump(),
        //     _ => self.error(&[ASTERISK, BRACKETS, IMPLICIT]),
        // }
        // if self.peek(0) != Some(RIGHT_PAREN) {
        //     self.error(&[RIGHT_PAREN]);
        //     return;
        // }

        self.bump(); // RIGHT_PAREN
        self.builder.finish_node(); // BRANCH
    }

    fn atom(&mut self) {
        self.builder.start_node(ATOM.into());
        match self.peek(0) {
            Some(LEFT_BRACKET) => self.brackets(),
            Some(ASTERISK | IMPLICIT) => self.bump(),
            _ => self.error(&[ASTERISK, BRACKETS, IMPLICIT]),
        }
        self.builder.finish_node(); // ATOM
    }

    fn try_brackets(&mut self) -> Option<SyntaxKind> {
        match self.peek(0)? {
            LEFT_BRACKET => {
                self.brackets(); // LEFT_BRACKET
                Some(LEFT_BRACKET)
            }
            _ => None,
        }
    }

    fn brackets(&mut self) {
        self.builder.start_node(BRACKETS.into());
        self.bump(); // LEFT_BRACKET
        self.try_isotope();
        match self.peek(0) {
            Some(ASTERISK | EXPLICIT | H | IMPLICIT) => self.bump(),
            _ => self.error(&[ASTERISK, EXPLICIT, H, IMPLICIT]),
        }
        self.try_parity();
        self.try_hydrogens();
        self.try_charge();
        self.try_class();
        match self.peek(0) {
            Some(RIGHT_BRACKET) => self.bump(), // RIGHT_BRACKET
            _ => self.error(&[RIGHT_BRACKET]),
        }
        self.builder.finish_node();
    }

    fn try_hydrogens(&mut self) {
        if let Some(H) = self.peek(0) {
            self.builder.start_node(HYDROGENS.into());
            self.bump(); // H
            if let Some(DIGIT) = self.peek(0) {
                self.unsigned();
            }

            self.builder.finish_node(); // HYDROGENS
        }
    }

    fn try_isotope(&mut self) {
        if let Some(DIGIT) = self.peek(0) {
            self.builder.start_node(ISOTOPE.into());
            self.unsigned();

            self.builder.finish_node(); // ISOTOPE
        }
    }

    fn try_parity(&mut self) {
        if let Some(AT) = self.peek(0) {
            self.builder.start_node(PARITY.into());
            self.bump(); // AT
            if let Some(AT) = self.peek(0) {
                self.bump(); // AT
            }

            self.builder.finish_node(); // PARITY
        }
    }

    fn try_charge(&mut self) {
        if let Some(PLUS | MINUS) = self.peek(0) {
            self.builder.start_node(CHARGE.into());
            self.signed(); // SIGNED
            self.builder.finish_node(); // CHARGE
        }
    }

    fn try_class(&mut self) {
        if let Some(COLON) = self.peek(0) {
            self.builder.start_node(CLASS.into());
            self.bump(); // COLON
            match self.peek(0) {
                Some(DIGIT) => self.unsigned(),
                _ => self.error(&[DIGIT]),
            }

            self.builder.finish_node(); // CLASS
        }
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
    errors: Vec<SyntaxError>,
}

impl Parse {
    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }

    pub fn errors(&self) -> &[SyntaxError] {
        &self.errors
    }
}
