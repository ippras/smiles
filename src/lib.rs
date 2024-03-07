//! SMILES STRINGS
//!
//! [opensmiles.org](http://opensmiles.org/opensmiles.html)

use self::chain::{chain, Chain};
use nom::{
    character::complete::multispace0,
    combinator::opt,
    error::{Error, ErrorKind, FromExternalError},
    sequence::terminated,
    IResult,
};
use std::str::FromStr;

// smiles ::= terminator | chain terminator
// terminator ::= SPACE | TAB | LINEFEED | CARRIAGE_RETURN | END_OF_STRING

pub fn smiles(input: &str) -> IResult<&str, Chain> {
    // alt((
    //     map(terminator, |_| None),
    //     map(, |chain| Some(chain)),
    // ))
    terminated(chain, opt(terminator))(input)
}

pub fn terminator(input: &str) -> IResult<&str, &str> {
    multispace0(input)
}

fn number<I: AsRef<str>, O: FromStr>(input: I) -> Result<O, Error<I>> {
    Ok(input
        .as_ref()
        .parse()
        .map_err(|error| Error::from_external_error(input, ErrorKind::MapRes, error))?)
}

pub mod atom;
pub mod bond;
pub mod bracket;
pub mod chain;
pub mod charge;
pub mod chirality;
pub mod class;
pub mod hydrogen;
pub mod organic;
