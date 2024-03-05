//! ATOMS
//!
//! [Atoms](http://opensmiles.org/opensmiles.html#inatoms)

use self::{
    bracket::{bracket_atom, Bracket},
    organic::{organic, Organic},
};
use crate::class::class;
use nom::{branch::alt, character::complete::char, combinator::map, IResult};

/// Atom
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Atom {
    Bracket(Bracket),
    Organic(Organic),
    Unknown,
}

pub fn atom(input: &str) -> IResult<&str, Atom> {
    alt((
        map(bracket_atom, Atom::Bracket),
        map(organic, Atom::Organic),
        map(char('*'), |_| Atom::Unknown),
    ))(input)
}

mod bracket;
mod organic;
