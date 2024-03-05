//! ORGANIC SUBSET ATOMS
//!
//! [Organic Subset](http://opensmiles.org/opensmiles.html#orgsbst)

use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

/// Organic
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Organic {
    Aliphatic(Aliphatic),
    Aromatic(Aromatic),
}

pub fn organic(input: &str) -> IResult<&str, Organic> {
    alt((
        map(aliphatic_organic, Organic::Aliphatic),
        map(aromatic_organic, Organic::Aromatic),
    ))(input)
}

/// Aliphatic organic
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Aliphatic {
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

pub fn aliphatic_organic(input: &str) -> IResult<&str, Aliphatic> {
    alt((
        map(tag("B"), |_| Aliphatic::B),
        map(tag("C"), |_| Aliphatic::C),
        map(tag("N"), |_| Aliphatic::N),
        map(tag("O"), |_| Aliphatic::O),
        map(tag("S"), |_| Aliphatic::S),
        map(tag("P"), |_| Aliphatic::P),
        map(tag("F"), |_| Aliphatic::F),
        map(tag("Cl"), |_| Aliphatic::Cl),
        map(tag("Br"), |_| Aliphatic::Br),
        map(tag("I"), |_| Aliphatic::I),
    ))(input)
}

// Aromatic organic
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Aromatic {
    B,
    C,
    N,
    O,
    S,
    P,
}

pub fn aromatic_organic(input: &str) -> IResult<&str, Aromatic> {
    alt((
        map(tag("b"), |_| Aromatic::B),
        map(tag("c"), |_| Aromatic::C),
        map(tag("n"), |_| Aromatic::N),
        map(tag("o"), |_| Aromatic::O),
        map(tag("s"), |_| Aromatic::S),
        map(tag("p"), |_| Aromatic::P),
    ))(input)
}
