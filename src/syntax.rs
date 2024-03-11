use crate::language::Language;
use logos::Logos;

pub(crate) type SyntaxNode = rowan::SyntaxNode<Language>;

pub(crate) type SyntaxToken = rowan::SyntaxToken<Language>;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Logos, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
pub(crate) enum SyntaxKind {
    #[token("*")]
    ASTERISK,
    #[token("@")]
    AT,
    #[token(":")]
    COLON,
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
    #[token(".")]
    PERIOD,
    #[token("%")]
    PERCENT,

    #[token("-")]
    MINUS,
    #[token("+")]
    PLUS,
    #[token("=")]
    EQUALS,
    #[token("#")]
    NUMBER,
    #[token("$")]
    DOLLAR,
    #[token("/")]
    SLASH,
    #[token("\\")]
    BACKSLASH,

    // #[regex("[+-][0-9]*")]
    // SIGNED,
    // #[regex("[0-9]*")]
    // UNSIGNED,
    #[regex("[0-9]")]
    DIGIT,

    // Explicit symbol
    // #[regex("Ac|Ag|Al|Am|Ar|as|As|At|Au|Ba|Be|Bh|Bi|Bk|Ca|Cd|Ce|Cf|Cm|Cn|Co|Cr|Cs|Cu|Db|Ds|Dy|Er|Es|Eu|Fe|Fl|Fm|Fr|Ga|Gd|Ge|He|Hf|Hg|Ho|Hs|In|Ir|K|Kr|La|Li|Lr|Lu|Lv|Md|Mg|Mn|Mo|Mt|Na|Nb|Nd|Ne|Ni|No|Np|Os|Pa|Pb|Pd|Pm|Po|Pr|Pt|Pu|Ra|Rb|Re|Rf|Rg|Rh|Rn|Ru|Sb|Sc|se|Se|Sg|Si|Sm|Sn|Sr|Ta|Tb|Tc|Te|Th|Ti|Tl|Tm|U|V|W|Xe|Y|Yb|Zn|Zr")]
    #[regex("[KUVWY]|He|Li|Be|Ne|Na|Mg|Al|Si|Ar|Ca|Sc|Ti|Cr|Mn|Fe|Co|Ni|Cu|Zn|Ga|Ge|As|Se|Kr|Rb|Sr|Zr|Nb|Mo|Tc|Ru|Rh|Pd|Ag|Cd|In|Sn|Sb|Te|Xe|Cs|Ba|Hf|Ta|Re|Os|Ir|Pt|Au|Hg|Tl|Pb|Bi|Po|At|Rn|Fr|Ra|Rf|Db|Sg|Bh|Hs|Mt|Ds|Rg|Cn|Fl|Lv|La|Ce|Pr|Nd|Pm|Sm|Eu|Gd|Tb|Dy|Ho|Er|Tm|Yb|Lu|Ac|Th|Pa|Np|Pu|Am|Cm|Bk|Cf|Es|Fm|Md|No|Lr|as|se")]
    EXPLICIT,
    // Implicit symbol
    #[regex("[BCFINOPS]|Cl|Br|[bcnops]")]
    IMPLICIT,
    #[regex("H")]
    H,

    BRACES,
    BRACKETS,
    PARENTHESES,

    BONDS,
    BOND,
    SERIAL,
    CLOSURE,

    NODE,
    VERTEX,

    UNSIGNED,
    SIGNED,

    INDEX,
    BRANCH,
    ATOM,
    CHAIN,
    CHARGE,
    CLASS,
    HYDROGENS,
    ISOTOPE,
    PARITY,

    END,
    ERROR,
    ROOT,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(value: SyntaxKind) -> Self {
        Self(value as _)
    }
}

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
