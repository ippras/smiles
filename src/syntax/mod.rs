use crate::language::Language;
use logos::Logos;

pub type SyntaxNode = rowan::SyntaxNode<Language>;

pub type SyntaxToken = rowan::SyntaxToken<Language>;

#[derive(Clone, Copy, Debug, Eq, Hash, Logos, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
pub enum Sign {
    #[token("*")]
    ASTERISK,
    #[token("@")]
    AT,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Logos, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
pub enum SyntaxKind {
    #[token("*")]
    STAR,
    #[token("@")]
    AT,
    #[token("\\")]
    BACKSLASH,
    #[token(":")]
    COLON,
    #[token("$")]
    DOLLAR,
    #[token("=")]
    EQUALS,
    #[token("-")]
    MINUS,
    #[token("#")]
    HASH,
    #[token("%")]
    PERCENT,
    #[token(".")]
    PERIOD,
    #[token("+")]
    PLUS,
    #[token("/")]
    SLASH,

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

    // Implicit
    #[regex("[BCFINOPS]|[bcnops]|Cl|Br")]
    IMPLICIT,
    // Explicit
    #[regex("[KUVWY]|Ac|Ag|Al|Am|Ar|As|At|Au|Ba|Be|Bh|Bi|Bk|Ca|Cd|Ce|Cf|Cm|Cn|Co|Cr|Cs|Cu|Db|Ds|Dy|Er|Es|Eu|Fe|Fl|Fm|Fr|Ga|Gd|Ge|He|Hf|Hg|Ho|Hs|In|Ir|Kr|La|Li|Lr|Lu|Lv|Md|Mg|Mn|Mo|Mt|Na|Nb|Nd|Ne|Ni|No|Np|Os|Pa|Pb|Pd|Pm|Po|Pr|Pt|Pu|Ra|Rb|Re|Rf|Rg|Rh|Rn|Ru|Sb|Sc|Se|Sg|Si|Sm|Sn|Sr|Ta|Tb|Tc|Te|Th|Ti|Tl|Tm|Xe|Yb|Zn|Zr|as|se")]
    EXPLICIT,
    #[token("H")]
    H,

    // COMPLEX,
    // SIMPLE,
    BRACKETS,

    BRANCH,
    BRANCHES,
    INDEXED,
    UNINDEXED,
    EDGE,
    NODE,
    TREE,

    INDEX,
    ATOM,
    CHARGE,
    CLASS,
    HYDROGENS,
    ISOTOPE,
    PARITY,

    UNSIGNED,
    SIGNED,

    END_OF_STRING,
    ROOT,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(value: SyntaxKind) -> Self {
        Self(value as _)
    }
}

pub(crate) mod ast;
pub(crate) mod st;

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
