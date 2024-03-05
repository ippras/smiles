/// Aromatic symbol
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AromaticSymbol {
    B,
    C,
    N,
    O,
    P,
    S,
    Se,
    As,
}

pub fn aromatic_symbols(input: &str) -> IResult<&str, AromaticSymbol> {
    alt((
        map(tag("b"), |_| AromaticSymbol::B),
        map(tag("c"), |_| AromaticSymbol::C),
        map(tag("n"), |_| AromaticSymbol::N),
        map(tag("o"), |_| AromaticSymbol::O),
        map(tag("p"), |_| AromaticSymbol::P),
        map(tag("s"), |_| AromaticSymbol::S),
        map(tag("se"), |_| AromaticSymbol::Se),
        map(tag("as"), |_| AromaticSymbol::As),
    ))(input)
}

/// Aliphatic organic
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AliphaticOrganic {
    B,
    C,
    N,
    O,
    S,
    P,
    F,
    Cl,
    Br,
    I,
}

fn aliphatic_organic(input: &str) -> IResult<&str, AliphaticOrganic> {
    alt((
        map(tag("B"), |_| AliphaticOrganic::B),
        map(tag("C"), |_| AliphaticOrganic::C),
        map(tag("N"), |_| AliphaticOrganic::N),
        map(tag("O"), |_| AliphaticOrganic::O),
        map(tag("S"), |_| AliphaticOrganic::S),
        map(tag("P"), |_| AliphaticOrganic::P),
        map(tag("F"), |_| AliphaticOrganic::F),
        map(tag("Cl"), |_| AliphaticOrganic::Cl),
        map(tag("Br"), |_| AliphaticOrganic::Br),
        map(tag("I"), |_| AliphaticOrganic::I),
    ))(input)
}

// Aromatic organic
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AromaticOrganic {
    B,
    C,
    N,
    O,
    S,
    P,
}

fn aromatic_organic(input: &str) -> IResult<&str, AromaticOrganic> {
    alt((
        map(tag("b"), |_| AromaticOrganic::B),
        map(tag("c"), |_| AromaticOrganic::C),
        map(tag("n"), |_| AromaticOrganic::N),
        map(tag("o"), |_| AromaticOrganic::O),
        map(tag("s"), |_| AromaticOrganic::S),
        map(tag("p"), |_| AromaticOrganic::P),
    ))(input)
}
