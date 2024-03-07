//! BONDS AND CHAINS
//!
//! [Bonds](http://opensmiles.org/opensmiles.html#bonds)

use crate::{
    atom::{atom, Atom},
    bond::{bond, ring_bond, Bond, RingBond},
};
use nom::{
    branch::alt,
    character::complete::char,
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

/// Chain
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Chain {
    Unchained(Link),
    Chained {
        link: Link,
        span: Option<Span>,
        chain: Box<Chain>,
    },
}

impl Chain {
    pub fn link(&self) -> &Link {
        match self {
            Self::Unchained(link) => link,
            Self::Chained { link, .. } => link,
        }
    }
}

pub fn chain(input: &str) -> IResult<&str, Chain> {
    alt((
        map(tuple((link, opt(span), chain)), |(link, span, chain)| {
            Chain::Chained {
                link,
                span,
                chain: Box::new(chain),
            }
        }),
        map(link, Chain::Unchained),
    ))(input)
}

/// Chain link
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Link {
    pub atom: Atom,
    pub ring_bonds: Vec<RingBond>,
    pub branches: Vec<Branch>,
}

fn link(input: &str) -> IResult<&str, Link> {
    map(
        tuple((atom, many0(ring_bond), many0(branch))),
        |(atom, ring_bonds, branches)| Link {
            atom,
            ring_bonds,
            branches,
        },
    )(input)
}

/// Branch
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Hash)]
pub struct Branch {
    pub span: Span,
    pub chain: Chain,
}

pub fn branch(input: &str) -> IResult<&str, Branch> {
    delimited(
        char('('),
        map(tuple((span, chain)), |(span, chain)| Branch { span, chain }),
        char(')'),
    )(input)
}

// Span
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone, Hash)]
pub enum Span {
    Bond(Bond),
    Dot,
}

pub fn span(input: &str) -> IResult<&str, Span> {
    alt((map(bond, Span::Bond), map(char('.'), |_| Span::Dot)))(input)
}
