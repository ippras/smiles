//! BONDS
//!
//! [Bonds](http://opensmiles.org/opensmiles.html#bonds)

use crate::number;
use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::{map, map_res, opt},
    error::{Error, ErrorKind},
    sequence::{preceded, tuple},
    IResult,
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
                        return Err(Error::new(input, ErrorKind::MapRes));
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
