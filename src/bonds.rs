//! BONDS AND CHAINS
//!
//! [Bonds](http://opensmiles.org/opensmiles.html#bonds)

use crate::{
    atoms::{atom, Atom},
    number,
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1, take_while_m_n},
    character::{
        complete::{char, digit1},
        is_digit,
    },
    combinator::{map, map_res, opt},
    error::{Error, ErrorKind},
    multi::many0,
    sequence::{delimited, preceded, tuple},
    Err, IResult, InputTake,
};
use tracing::warn;

/// Bond
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Bond {
    Single,
    Double,
    Triple,
    Quadruple,
    Aromatic,
    Up,
    Down,
}

pub fn bond(input: &str) -> IResult<&str, Bond> {
    alt((
        map(char('-'), |_| Bond::Single),
        map(char('='), |_| Bond::Double),
        map(char('#'), |_| Bond::Triple),
        map(char('$'), |_| Bond::Quadruple),
        map(char(':'), |_| Bond::Aromatic),
        map(char('/'), |_| Bond::Up),
        map(char('\\'), |_| Bond::Down),
    ))(input)
}

/// Ring bond
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RingBond {
    pub bond: Option<Bond>,
    pub number: u8,
}

// Two-digit ring numbers are permitted, but must be preceded by the percent '%'
// symbol, such as C%25CCCCC%25 for cyclohexane. Three-digit numbers and larger
// are never permitted. However, note that three digits are not invalid; for
// example, C%123 is the same as C3%12, that is, an atom with two rnum
// specifications.
//
// The digit(s) representing a ring-closure are interpreted as a number, not a
// symbol, and two rnums match if their numbers match. Thus, C1CCCCC%01 is a
// valid SMILES and is the same as C1CCCCC1. Likewise, C%00CCCCC%00 is a valid
// SMILES.
pub fn ring_bond(input: &str) -> IResult<&str, RingBond> {
    map(
        tuple((
            opt(bond),
            alt((
                map_res(digit1, |digits: &str| {
                    let count = digits.chars().count();
                    if count != 1 {
                        return Err(Error::new(input.clone(), ErrorKind::MapRes));
                    }
                    Ok(number(digits)?)
                }),
                map_res(preceded(char('%'), digit1), |digits: &str| {
                    // Two-digit ring numbers are permitted, but must be
                    // preceded by the percent '%' symbol, such as C%25CCCCC%25
                    // for cyclohexane. Three-digit numbers and larger are never
                    // permitted. However, note that three digits are not
                    // invalid; for example, C%123 is the same as C3%12, that
                    // is, an atom with two rnum specifications.
                    //
                    // The digit(s) representing a ring-closure are interpreted
                    // as a number, not a symbol, and two rnums match if their
                    // numbers match. Thus, C1CCCCC%01 is a valid SMILES and is
                    // the same as C1CCCCC1. Likewise, C%00CCCCC%00 is a valid
                    // SMILES.
                    // let digits: String = digits.chars().take(2).collect();
                    let rest: String = digits.chars().skip(2).collect();
                    if !rest.is_empty() {
                        warn!(rest);
                    }
                    number(digits.chars().take(2).collect::<String>())
                }),
            )),
        )),
        |(bond, number)| RingBond { bond, number },
    )(input)
}

/// Branched atom
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BranchedAtom {
    pub atom: Atom,
    pub ring_bonds: Vec<RingBond>,
    pub branches: Vec<Branch>,
}

fn branched_atom(input: &str) -> IResult<&str, BranchedAtom> {
    map(
        tuple((atom, many0(ring_bond), many0(branch))),
        |(atom, ring_bonds, branches)| BranchedAtom {
            atom,
            ring_bonds,
            branches,
        },
    )(input)
}

/// Branch
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Hash)]
pub enum Branch {
    Chain { chain: Chain },
    BondChain { bond: Bond, chain: Chain },
    DotChain { dot: Dot, chain: Chain },
}

pub fn branch(input: &str) -> IResult<&str, Branch> {
    delimited(
        char('('),
        alt((
            map(chain, |chain| Branch::Chain { chain }),
            map(tuple((bond, chain)), |(bond, chain)| Branch::BondChain {
                bond,
                chain,
            }),
            map(tuple((dot, chain)), |(dot, chain)| Branch::DotChain {
                dot,
                chain,
            }),
        )),
        char(')'),
    )(input)
}

/// Chain
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Chain {
    BranchedAtom {
        branched_atom: BranchedAtom,
    },
    ChainBranchedAtom {
        chain: Box<Chain>,
        branched_atom: BranchedAtom,
    },
    ChainBondBranchedAtom {
        chain: Box<Chain>,
        bond: Bond,
        branched_atom: BranchedAtom,
    },
    ChainDotBranchedAtom {
        chain: Box<Chain>,
        dot: Dot,
        branched_atom: BranchedAtom,
    },
}

pub fn chain(input: &str) -> IResult<&str, Chain> {
    alt((
        map(branched_atom, |branched_atom| Chain::BranchedAtom {
            branched_atom,
        }),
        map(tuple((chain, branched_atom)), |(chain, branched_atom)| {
            Chain::ChainBranchedAtom {
                chain: Box::new(chain),
                branched_atom,
            }
        }),
        map(
            tuple((chain, bond, branched_atom)),
            |(chain, bond, branched_atom)| Chain::ChainBondBranchedAtom {
                chain: Box::new(chain),
                bond,
                branched_atom,
            },
        ),
        map(
            tuple((chain, dot, branched_atom)),
            |(chain, dot, branched_atom)| Chain::ChainDotBranchedAtom {
                chain: Box::new(chain),
                dot,
                branched_atom,
            },
        ),
    ))(input)
}

// Dot
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone, Hash)]
pub struct Dot;

pub fn dot(input: &str) -> IResult<&str, Dot> {
    map(char('.'), |_| Dot)(input)
}
