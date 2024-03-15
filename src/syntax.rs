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
    ASTERISK,
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
    NUMBER,
    #[token("%")]
    PERCENT,
    #[token(".")]
    PERIOD,
    #[token("+")]
    PLUS,
    #[token("/")]
    SLASH,

    Sign(Sign),

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

    // Explicit symbol
    // #[regex("Ac|Ag|Al|Am|Ar|as|As|At|Au|Ba|Be|Bh|Bi|Bk|Ca|Cd|Ce|Cf|Cm|Cn|Co|Cr|Cs|Cu|Db|Ds|Dy|Er|Es|Eu|Fe|Fl|Fm|Fr|Ga|Gd|Ge|He|Hf|Hg|Ho|Hs|In|Ir|K|Kr|La|Li|Lr|Lu|Lv|Md|Mg|Mn|Mo|Mt|Na|Nb|Nd|Ne|Ni|No|Np|Os|Pa|Pb|Pd|Pm|Po|Pr|Pt|Pu|Ra|Rb|Re|Rf|Rg|Rh|Rn|Ru|Sb|Sc|se|Se|Sg|Si|Sm|Sn|Sr|Ta|Tb|Tc|Te|Th|Ti|Tl|Tm|U|V|W|Xe|Y|Yb|Zn|Zr")]
    // #[regex("|as|se")]
    // EXPLICIT,
    // Implicit symbol
    // #[regex("[BCFINOPS]|Cl|Br|[bcnops]")]
    // IMPLICIT,

    // #[regex("[bcnops]")]
    #[token("H")]
    H,
    // Explicit two chars
    #[token("He")]
    HE,
    #[token("Li")]
    LI,
    #[token("Be")]
    BE,
    #[token("Ne")]
    NE,
    #[token("Na")]
    NA,
    #[token("Mg")]
    MG,
    #[token("Al")]
    AL,
    #[token("Si")]
    SI,
    #[token("Ar")]
    AR,
    #[token("Ca")]
    CA,
    #[token("Sc")]
    SC,
    #[token("Ti")]
    TI,
    #[token("Cr")]
    CR,
    #[token("Mn")]
    MN,
    #[token("Fe")]
    FE,
    #[token("Co")]
    CO,
    #[token("Ni")]
    NI,
    #[token("Cu")]
    CU,
    #[token("Zn")]
    ZN,
    #[token("Ga")]
    GA,
    #[token("Ge")]
    GE,
    #[token("As")]
    AS,
    #[token("Se")]
    SE,
    #[token("Kr")]
    KR,
    #[token("Rb")]
    RB,
    #[token("Sr")]
    SR,
    #[token("Zr")]
    ZR,
    #[token("Nb")]
    NB,
    #[token("Mo")]
    MO,
    #[token("Tc")]
    TC,
    #[token("Ru")]
    RU,
    #[token("Rh")]
    RH,
    #[token("Pd")]
    PD,
    #[token("Ag")]
    AG,
    #[token("Cd")]
    CD,
    #[token("In")]
    IN,
    #[token("Sn")]
    SN,
    #[token("Sb")]
    SB,
    #[token("Te")]
    TE,
    #[token("Xe")]
    XE,
    #[token("Cs")]
    CS,
    #[token("Ba")]
    BA,
    #[token("Hf")]
    HF,
    #[token("Ta")]
    TA,
    #[token("Re")]
    RE,
    #[token("Os")]
    OS,
    #[token("Ir")]
    IR,
    #[token("Pt")]
    PT,
    #[token("Au")]
    AU,
    #[token("Hg")]
    HG,
    #[token("Tl")]
    TL,
    #[token("Pb")]
    PB,
    #[token("Bi")]
    BI,
    #[token("Po")]
    PO,
    // #[token("At")]
    // AT,
    #[token("Rn")]
    RN,
    #[token("Fr")]
    FR,
    #[token("Ra")]
    RA,
    #[token("Rf")]
    RF,
    #[token("Db")]
    DB,
    #[token("Sg")]
    SG,
    #[token("Bh")]
    BH,
    #[token("Hs")]
    HS,
    #[token("Mt")]
    MT,
    #[token("Ds")]
    DS,
    #[token("Rg")]
    RG,
    #[token("Cn")]
    CN,
    #[token("Fl")]
    FL,
    #[token("Lv")]
    LV,
    #[token("La")]
    LA,
    #[token("Ce")]
    CE,
    #[token("Pr")]
    PR,
    #[token("Nd")]
    ND,
    #[token("Pm")]
    PM,
    #[token("Sm")]
    SM,
    #[token("Eu")]
    EU,
    #[token("Gd")]
    GD,
    #[token("Tb")]
    TB,
    #[token("Dy")]
    DY,
    #[token("Ho")]
    HO,
    #[token("Er")]
    ER,
    #[token("Tm")]
    TM,
    #[token("Yb")]
    YB,
    #[token("Lu")]
    LU,
    #[token("Ac")]
    AC,
    #[token("Th")]
    TH,
    #[token("Pa")]
    PA,
    #[token("Np")]
    NP,
    #[token("Pu")]
    PU,
    #[token("Am")]
    AM,
    #[token("Cm")]
    CM,
    #[token("Bk")]
    BK,
    #[token("Cf")]
    CF,
    #[token("Es")]
    ES,
    #[token("Fm")]
    FM,
    #[token("Md")]
    MD,
    #[token("No")]
    NO,
    #[token("Lr")]
    LR,
    // Explicit one char
    #[token("K")]
    K,
    #[token("U")]
    U,
    #[token("V")]
    V,
    #[token("W")]
    W,
    #[token("Y")]
    Y,
    // Implicit two chars
    #[token("Cl")]
    CL,
    #[token("Br")]
    BR,
    // Implicit one char
    #[token("B")]
    B,
    #[token("C")]
    C,
    #[token("F")]
    F,
    #[token("I")]
    I,
    #[token("N")]
    N,
    #[token("O")]
    O,
    #[token("P")]
    P,
    #[token("S")]
    S,

    BRACKETS,

    // EXPLICIT,
    // IMPLICIT,
    BRANCH,
    BRANCHES,
    CLOSURE,
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
