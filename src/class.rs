//! ATOM CLASS
//!
//! [Atom Class](http://opensmiles.org/opensmiles.html#atomclass)

use nom::{
    character::complete::{char, u64},
    sequence::preceded,
    IResult,
};

/// Class
///
/// A SMILES parser should accept at least four digits for the atom class, and
/// the values 0 to 9999.
pub fn class(input: &str) -> IResult<&str, u64> {
    preceded(char(':'), u64)(input)
}
