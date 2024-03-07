//! CHIRALITY
//!
//! [Chirality](http://opensmiles.org/opensmiles.html#chirality)

use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

/// Chiral
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Chiral {
    /// `@TH`
    Tetrahedral(Tetrahedral),
    /// `@AL`
    Allenal(Allenal),
    /// `@SP`
    SquarePlanar(SquarePlanar),
    /// `@TB`
    TrigonalBipyramidal(TrigonalBipyramidal),
    /// `@OH`
    Octahedral(Octahedral),
}

// alt((
//     map(alt((tag("@"), tag("@TH1"))), |_| Tetrahedral::One),
//     map(alt((tag("@@"), tag("@TH2"))), |_| Tetrahedral::Two),
// )),
pub fn chiral(input: &str) -> IResult<&str, Chiral> {
    alt((
        map(tetrahedral, Chiral::Tetrahedral),
        map(allenal, Chiral::Allenal),
        map(square_planar, Chiral::SquarePlanar),
        map(trigonal_bipyramidal, Chiral::TrigonalBipyramidal),
        map(octahedral, Chiral::Octahedral),
    ))(input)
}

/// Tetrahedral Centers
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Tetrahedral {
    One = 1,
    Two,
}

pub fn tetrahedral(input: &str) -> IResult<&str, Tetrahedral> {
    alt((
        map(tag("@TH1"), |_| Tetrahedral::One),
        map(tag("@TH2"), |_| Tetrahedral::Two),
    ))(input)
}

/// Tetrahedral Allene-like Systems
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Allenal {
    One = 1,
    Two,
}

pub fn allenal(input: &str) -> IResult<&str, Allenal> {
    alt((
        map(tag("@AL1"), |_| Allenal::One),
        map(tag("@AL2"), |_| Allenal::Two),
    ))(input)
}

/// Square Planar Centers
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SquarePlanar {
    One = 1,
    Two,
    Three,
}

pub fn square_planar(input: &str) -> IResult<&str, SquarePlanar> {
    alt((
        map(tag("@SP1"), |_| SquarePlanar::One),
        map(tag("@SP2"), |_| SquarePlanar::Two),
        map(tag("@SP3"), |_| SquarePlanar::Three),
    ))(input)
}

/// Trigonal Bipyramidal Centers
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TrigonalBipyramidal {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
}

pub fn trigonal_bipyramidal(input: &str) -> IResult<&str, TrigonalBipyramidal> {
    alt((
        map(tag("@TB1"), |_| TrigonalBipyramidal::One),
        map(tag("@TB2"), |_| TrigonalBipyramidal::Two),
        map(tag("@TB3"), |_| TrigonalBipyramidal::Three),
        map(tag("@TB4"), |_| TrigonalBipyramidal::Four),
        map(tag("@TB5"), |_| TrigonalBipyramidal::Five),
        map(tag("@TB6"), |_| TrigonalBipyramidal::Six),
        map(tag("@TB7"), |_| TrigonalBipyramidal::Seven),
        map(tag("@TB8"), |_| TrigonalBipyramidal::Eight),
        map(tag("@TB9"), |_| TrigonalBipyramidal::Nine),
        map(tag("@TB10"), |_| TrigonalBipyramidal::Ten),
        map(tag("@TB11"), |_| TrigonalBipyramidal::Eleven),
        map(tag("@TB12"), |_| TrigonalBipyramidal::Twelve),
        map(tag("@TB13"), |_| TrigonalBipyramidal::Thirteen),
        map(tag("@TB14"), |_| TrigonalBipyramidal::Fourteen),
        map(tag("@TB15"), |_| TrigonalBipyramidal::Fifteen),
        map(tag("@TB16"), |_| TrigonalBipyramidal::Sixteen),
        map(tag("@TB17"), |_| TrigonalBipyramidal::Seventeen),
        map(tag("@TB18"), |_| TrigonalBipyramidal::Eighteen),
        map(tag("@TB19"), |_| TrigonalBipyramidal::Nineteen),
        map(tag("@TB20"), |_| TrigonalBipyramidal::Twenty),
    ))(input)
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Octahedral {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    TwentyOne,
    TwentyTwo,
    TwentyThree,
    TwentyFour,
    TwentyFive,
    TwentySix,
    TwentySeven,
    TwentyEight,
    TwentyNine,
    Thirty,
}

pub fn octahedral(input: &str) -> IResult<&str, Octahedral> {
    alt((
        alt((
            map(tag("@OH1"), |_| Octahedral::One),
            map(tag("@OH2"), |_| Octahedral::Two),
            map(tag("@OH3"), |_| Octahedral::Three),
            map(tag("@OH4"), |_| Octahedral::Four),
            map(tag("@OH5"), |_| Octahedral::Five),
            map(tag("@OH6"), |_| Octahedral::Six),
            map(tag("@OH7"), |_| Octahedral::Seven),
            map(tag("@OH8"), |_| Octahedral::Eight),
            map(tag("@OH9"), |_| Octahedral::Nine),
            map(tag("@OH10"), |_| Octahedral::Ten),
            map(tag("@OH11"), |_| Octahedral::Eleven),
            map(tag("@OH12"), |_| Octahedral::Twelve),
            map(tag("@OH13"), |_| Octahedral::Thirteen),
            map(tag("@OH14"), |_| Octahedral::Fourteen),
            map(tag("@OH15"), |_| Octahedral::Fifteen),
            map(tag("@OH16"), |_| Octahedral::Sixteen),
            map(tag("@OH17"), |_| Octahedral::Seventeen),
            map(tag("@OH18"), |_| Octahedral::Eighteen),
            map(tag("@OH19"), |_| Octahedral::Nineteen),
            map(tag("@OH20"), |_| Octahedral::Twenty),
            map(tag("@OH21"), |_| Octahedral::TwentyOne),
        )),
        alt((
            map(tag("@OH22"), |_| Octahedral::TwentyTwo),
            map(tag("@OH23"), |_| Octahedral::TwentyThree),
            map(tag("@OH24"), |_| Octahedral::TwentyFour),
            map(tag("@OH25"), |_| Octahedral::TwentyFive),
            map(tag("@OH26"), |_| Octahedral::TwentySix),
            map(tag("@OH27"), |_| Octahedral::TwentySeven),
            map(tag("@OH28"), |_| Octahedral::TwentyEight),
            map(tag("@OH29"), |_| Octahedral::TwentyNine),
            map(tag("@OH30"), |_| Octahedral::Thirty),
        )),
    ))(input)
}

// fn chiral1(input: &str) -> IResult<&str, Chirality> {
//     alt((
//         map(tag("@"), |_| Chirality),
//         map(tag("@@"), |_| Chirality),
//         map(preceded(tag("@TH"), u8), |n| match n {
//             1..=2 => Ok(Chirality::Tetrahedral(n)),
//             _ => Err(Error::new(input.clone(), ErrorKind::MapRes)),
//         }),
//         map(preceded(tag("@AL"), u8), |n| match n {
//             1..=2 => Ok(Chirality::Allenal(n)),
//             _ => Err(Error::new(input.clone(), ErrorKind::MapRes)),
//         }),
//         map(preceded(tag("@SP"), u8), |n| match n {
//             1..=3 => Ok(Chirality::SquarePlanar(n)),
//             _ => Err(Error::new(input.clone(), ErrorKind::MapRes)),
//         }),
//         map(preceded(tag("@TB"), u8), |n| match n {
//             1..=20 => Ok(Chirality::TrigonalBipyramidal(n)),
//             _ => Err(Error::new(input.clone(), ErrorKind::MapRes)),
//         }),
//         map(preceded(tag("@OH"), u8), |n| match n {
//             1..=30 => Ok(Chirality::Octahedral(n)),
//             _ => Err(Error::new(input.clone(), ErrorKind::MapRes)),
//         }),
//         // '@TB' DIGIT DIGIT | '@OH' DIGIT DIGIT
//     ))(input)
//     // '@OH1'
//     // '@OH2'
//     // '@OH3'
//     // …
//     // '@OH30'
// }

// fn chirality(input: &str) -> IResult<&str, Chirality> {
//     map_res(raw_chirality, |sym: &str| {
//         let other_str = std::str::from_utf8(sym).map_err(|_| "Unparsable UTF-8")?;

//         let chirality: Result<Chirality, &'static str> = match other_str {
//             "@" => Ok(Chirality::Anticlockwise),
//             "@@" => Ok(Chirality::Clockwise),
//             "@TH1" | "@TH2" => Ok(Chirality::Tetrahedral(other_str[3..].parse().unwrap())),
//             "@AL1" | "@AL2" => Ok(Chirality::Allenal(other_str[3..].parse().unwrap())),
//             "@SP1" | "@SP2" | "@SP3" => {
//                 Ok(Chirality::SquarePlanar(other_str[3..].parse().unwrap()))
//             }
//             "@TB1" | "@TB2" | "@TB3" | "@TB4" | "@TB5" | "@TB6" | "@TB7" | "@TB8" | "@TB9"
//             | "@TB10" | "@TB11" | "@TB12" | "@TB13" | "@TB14" | "@TB15" | "@TB16" | "@TB17"
//             | "@TB18" | "@TB19" | "@TB20" => Ok(Chirality::TrigonalBipyramidal(
//                 other_str[3..].parse().unwrap(),
//             )),
//             "@OH1" | "@OH2" | "@OH3" | "@OH4" | "@OH5" | "@OH6" | "@OH7" | "@OH8" | "@OH9"
//             | "@OH10" | "@OH11" | "@OH12" | "@OH13" | "@OH14" | "@OH15" | "@OH16" | "@OH17"
//             | "@OH18" | "@OH19" | "@OH20" | "@OH21" | "@OH22" | "@OH23" | "@OH24" | "@OH25"
//             | "@OH26" | "@OH27" | "@OH28" | "@OH29" | "@OH30" => {
//                 Ok(Chirality::Octahedral(other_str[3..].parse().unwrap()))
//             }
//             _ => unreachable!(),
//         };

//         chirality
//     })(input)
// }

// fn raw_chirality(input: &str) -> IResult<&str, &str> {
//     alt((
//         alt((tag(b"@TH1"), tag(b"@TH2"))),
//         alt((tag(b"@AL1"), tag(b"@AL2"))),
//         alt((tag(b"@SP1"), tag(b"@SP2"), tag(b"@SP3"))),
//         alt((
//             tag(b"@TB1"),
//             tag(b"@TB2"),
//             tag(b"@TB3"),
//             tag(b"@TB4"),
//             tag(b"@TB5"),
//             tag(b"@TB6"),
//             tag(b"@TB7"),
//             tag(b"@TB8"),
//             tag(b"@TB9"),
//             tag(b"@TB10"),
//             tag(b"@TB11"),
//             tag(b"@TB12"),
//             tag(b"@TB13"),
//             tag(b"@TB14"),
//             tag(b"@TB15"),
//             tag(b"@TB16"),
//             tag(b"@TB17"),
//             tag(b"@TB18"),
//             tag(b"@TB19"),
//             tag(b"@TB20"),
//         )),
//         alt((
//             tag(b"@OH10"),
//             tag(b"@OH11"),
//             tag(b"@OH12"),
//             tag(b"@OH13"),
//             tag(b"@OH14"),
//             tag(b"@OH15"),
//             tag(b"@OH16"),
//             tag(b"@OH17"),
//             tag(b"@OH18"),
//             tag(b"@OH19"),
//             tag(b"@OH20"),
//             tag(b"@OH1"),
//             tag(b"@OH2"),
//             tag(b"@OH3"),
//             tag(b"@OH4"),
//             tag(b"@OH5"),
//             tag(b"@OH6"),
//             tag(b"@OH7"),
//             tag(b"@OH8"),
//             tag(b"@OH9"),
//         )),
//         alt((
//             tag(b"@OH21"),
//             tag(b"@OH22"),
//             tag(b"@OH23"),
//             tag(b"@OH24"),
//             tag(b"@OH25"),
//             tag(b"@OH26"),
//             tag(b"@OH27"),
//             tag(b"@OH28"),
//             tag(b"@OH29"),
//             tag(b"@OH30"),
//         )),
//         tag(b"@@"),
//         tag(b"@"),
//     ))(input)
// }
