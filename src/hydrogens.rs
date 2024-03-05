//! HYDROGENS
//!
//! [Hydrogens](http://opensmiles.org/opensmiles.html#hydrogens)

use super::number;
use nom::{
    character::complete::{char, digit1},
    combinator::{map, map_res, opt},
    sequence::preceded,
    IResult,
};

pub fn hydrogens(input: &str) -> IResult<&str, u8> {
    preceded(
        char('H'),
        map(opt(map_res(digit1, number)), |count| count.unwrap_or(1)),
    )(input)
}
