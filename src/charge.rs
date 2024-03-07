//! CHARGES
//!
//! [Charge](http://opensmiles.org/opensmiles.html#charge)

use crate::number;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, map_res, opt},
    error::{Error, ErrorKind},
    sequence::preceded,
    IResult,
};
use std::ops::Neg;
use tracing::warn;

/// Charge
///
/// An implementation is required to accept charges in the range -15 to +15.
pub fn charge(input: &str) -> IResult<&str, i8> {
    alt((
        map(tag("++"), |_| {
            warn!("++ charge is DEPRECATED!");
            2
        }),
        map(tag("--"), |_| {
            warn!("-- charge is DEPRECATED!");
            -2
        }),
        preceded(char('+'), count),
        map(preceded(char('-'), count), Neg::neg),
    ))(input)
}

fn count(input: &str) -> IResult<&str, i8> {
    map_res(opt(map_res(digit1, number)), |count: Option<u8>| {
        let count = count.unwrap_or(1) as _;
        if count > 15 {
            return Err(Error::new(input, ErrorKind::MapRes));
        }
        Ok(count)
    })(input)
}
