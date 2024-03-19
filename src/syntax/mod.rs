// String            ::= sequence?
// sequence          ::= atom ( union | branch | gap )*
// union             ::= bond? ( bridge | sequence )
// branch            ::= "(" ( dot | bond )? sequence ")"
// gap               ::= dot sequence
// atom              ::= star | shortcut | selection | bracket
// bracket           ::= "[" isotope? symbol parity?
//                       virtual_hydrogen? charge? "]"
// isotope           ::= nonzero digit? digit?
// symbol            ::= star | element | selection
// virtual_hydrogen  ::= "H" nonzero?
// charge            ::= ( "+" | "-" ) nonzero?
// bridge            ::= nonzero | "%" nonzero digit
// parity            ::= "@" "@"?
// star              ::= "*"
// dot               ::= "."
// shortcut          ::= "B" "r"? | "C" "l"? | "N" | "O" | "P" | "S" | "F"
//                     | "I"
// selection         ::= "b" | "c" | "n" | "o" | "p" | "s"
// element           ::= "A" ( "c" | "g" | "l" | "m" | "r" | "s" | "t" | "u" )
//                     | "B" ( "a" | "e" | "i" | "k" | "r" )?
//                     | "C" ( "a" | "d" | "e" | "f" | "l" | "m" | "o"
//                     | "r" | "s" | "u" )?
//                     | "Dy"
//                     | "E" ( "r" | "s" | "u" )
//                     | "F" ( "e" | "m" | "r" )?
//                     | "G" ( "a" | "d" | "e" )
//                     | "H" ( "e" | "f" | "g" | "o" )?
//                     | "I" ( "n" | "r" )?
//                     | "K" "r"?
//                     | "L" ( "a" | "i" | "r" | "u" )
//                     | "M" ( "g" | "n" | "o" )
//                     | "N" ( "a" | "b" | "d" | "e" | "i" | "o" | "p" )?
//                     | "O" "s"?
//                     | "P" ( "a" | "b" | "d" | "m" | "o" | "r" | "t" | "u" )?
//                     | "R" ( "a" | "b" | "e" | "f" | "h" | "n" | "u" )
//                     | "S" ( "b" | "c" | "e" | "i" | "m" | "n" | "r" )?
//                     | "T" ( "a" | "b" | "c" | "e" | "h" | "i" | "l" | "m" )
//                     | "U" | "V" | "W" | "Xe" | "Y" "b"?
//                     | "Z" ( "n" | "r" )
// bond              ::= "-" | "=" | "#" | "/" | "\"
// digit             ::= "0" | nonzero
// nonzero           ::= "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"

pub use self::error::Error;

use crate::language::Language;
use logos::Logos;

pub(crate) type SyntaxNode = rowan::SyntaxNode<Language>;

pub(crate) type SyntaxToken = rowan::SyntaxToken<Language>;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Logos, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
pub enum SyntaxKind {
    #[token("@")]
    AT,
    #[token("\\")]
    BACKSLASH,
    #[token(":")]
    COLON,
    #[token("$")]
    DOLLAR,
    #[token(".")]
    DOT,
    #[token("=")]
    EQUALS,
    #[token("#")]
    HASH,
    #[token("-")]
    MINUS,
    #[token("%")]
    PERCENT,
    #[token("+")]
    PLUS,
    #[token("/")]
    SLASH,
    #[token("*")]
    STAR,

    #[token("[")]
    LEFT_BRACKET,
    #[token("{")]
    LEFT_CURLY,
    #[token("(")]
    LEFT_PAREN,
    #[token("]")]
    RIGHT_BRACKET,
    #[token("}")]
    RIGHT_CURLY,
    #[token(")")]
    RIGHT_PAREN,

    #[regex("[0-9]")]
    DIGIT,

    #[regex("Br?|Cl?|F|I|N|O|P|S")]
    ORGANIC,
    #[regex("A[cglmrstu]|B[aeik]|C[adefmorsu]|Dy|E[rsu]|F[emr]|G[ade]|H[efgo]|I[nr]|Kr?|L[airu]|M[gno]|N[abdeiop]|Os|P[abdmortu]|R[abefhnu]|S[bceimnr]|T[abcehilm]|U|V|W|Xe|Yb?|Z[nr]")]
    NONORGANIC,
    #[token("H")]
    H,

    // COMPLEX,
    // SIMPLE,
    BRANCH,
    BRANCHES,
    EDGE,
    INDEX,
    INDEXED,
    NODE,
    TREE,
    UNINDEXED,

    CHARGE,
    CLASS,
    ELEMENT,
    HYDROGENS,
    ISOTOPE,
    PARITY,

    UNSIGNED,
    SIGNED,

    END_OF_STRING,
    ERROR,
    ROOT,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(value: SyntaxKind) -> Self {
        Self(value as _)
    }
}

pub(crate) mod ast;

mod error;
