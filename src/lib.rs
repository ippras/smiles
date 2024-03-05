//! [opensmiles.org](http://opensmiles.org/opensmiles.html)

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::{
        complete::{char, digit0, digit1},
        is_digit,
    },
    combinator::{map, map_res, opt},
    error::{Error, ErrorKind, FromExternalError, ParseError},
    multi::many0,
    sequence::{delimited, preceded, tuple},
    Err, IResult, Parser,
};
use std::str::{FromStr, Utf8Error};

// pub fn old_parse_digits<'a, I, O, E>(
//     mut parser: impl Parser<I, &'a str, E>,
// ) -> impl FnMut(I) -> IResult<I, O, E>
// where
//     I: Clone,
//     O: FromStr,
//     E: FromExternalError<I, Utf8Error> + FromExternalError<I, <O as FromStr>::Err>,
// {
//     move |input| {
//         let (input, output) = parser.parse(input)?;
//         // let output = std::str::from_utf8(output).map_err(|error| {
//         //     Err::Error(E::from_external_error(
//         //         input.clone(),
//         //         ErrorKind::MapRes,
//         //         error,
//         //     ))
//         // })?;
//         let output = output.parse().map_err(|error| {
//             Err::Error(E::from_external_error(
//                 input.clone(),
//                 ErrorKind::MapRes,
//                 error,
//             ))
//         })?;
//         Ok((input, output))
//     }
// }

// fn digits_gg<const M: usize, const N: usize, T: FromStr>(input: &str) -> IResult<&str, T> {
//     map_res(
//         map_res(take_while_m_n(M, N, is_digit), std::str::from_utf8),
//         str::parse,
//     )(input)
// }

// fn old_digit(input: &str) -> IResult<&str, u8> {
//     map_res(
//         map_res(
//             alt((
//                 take_while_m_n(1, 1, is_digit),
//                 preceded(tag(b"%"), take_while_m_n(2, 2, is_digit)),
//             )),
//             |s: &[u8]| std::str::from_utf8(s),
//         ),
//         |s: &str| s.parse::<u8>(),
//     )(input)
// }

// SMILES STRINGS

// smiles ::= terminator | chain terminator
// terminator ::= SPACE | TAB | LINEFEED | CARRIAGE_RETURN | END_OF_STRING

// pub fn terminator(input: &str) -> IResult<&str, ()> {
// }

fn number<I: AsRef<str>, O: FromStr>(input: I) -> Result<O, Error<I>> {
    Ok(input
        .as_ref()
        .parse()
        .map_err(|error| Error::from_external_error(input, ErrorKind::MapRes, error))?)
}

mod atoms;
mod bonds;
mod charge;
mod chirality;
mod class;
mod hydrogens;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bonds::{chain, BranchedAtom, Chain};

    #[test]
    fn symbol_cases() {
        matches!(Chain::BranchedAtom {
            branched_atom: BranchedAtom { atom: Atom{} }
        }, );
        assert_eq!(Ok(("", Symbol::Element(ElementSymbol::He))), chain("[U]"));
        assert_eq!(Ok(("", Symbol::Element(ElementSymbol::He))), chain("[He]"));
        assert_eq!(Ok(("", Symbol::Unknown)), chain("*"));
        // [U] Uranium
        // [Pb] Lead
        // [He] Helium
        // [*] Unknown
    }

    // #[test]
    // fn isotope_opt_cases() {
    //     assert_eq!(Ok(("".as_bytes(), Some(0u16))), isotope_opt(b"0"));
    //     assert_eq!(Ok(("".as_bytes(), Some(125u16))), isotope_opt(b"125"));
    //     assert_eq!(Ok(("X".as_bytes(), Some(125u16))), isotope_opt(b"125X"));
    //     assert_eq!(Ok(("7".as_bytes(), Some(125u16))), isotope_opt(b"1257"));
    // }

    // #[test]
    // fn bracket_atom_cases() {
    //     assert_eq!(
    //         Ok((
    //             "".as_bytes(),
    //             BracketAtom {
    //                 isotope: Some(16),
    //                 symbol: Symbol::ElementSymbol(Element::Carbon),
    //                 chiral: None,
    //                 hcount: 0,
    //                 charge: -2,
    //             }
    //         )),
    //         bracket_atom(b"[16C--]")
    //     );
    //     assert_eq!(
    //         Ok((
    //             "CC".as_bytes(),
    //             BracketAtom {
    //                 isotope: Some(16),
    //                 symbol: Symbol::ElementSymbol(Element::Carbon),
    //                 chiral: None,
    //                 hcount: 1,
    //                 charge: 3,
    //             }
    //         )),
    //         bracket_atom(b"[16CH+3]CC")
    //     );
    // }

    // #[test]
    // fn ring_bond_digit_cases() {
    //     assert_eq!(Ok(("".as_bytes(), 0u8)), old_digit(b"0"));
    //     assert_eq!(Ok(("".as_bytes(), 12u8)), old_digit(b"%12"));
    //     assert_eq!(Ok(("5".as_bytes(), 12u8)), old_digit(b"%125"));
    // }

    // #[test]
    // fn chirality_cases() {
    //     assert_eq!(
    //         Ok(("".as_bytes(), Chirality::Anticlockwise)),
    //         chirality(b"@")
    //     );
    //     assert_eq!(
    //         Ok(("".as_bytes(), Chirality::Tetrahedral(1))),
    //         chirality(b"@TH1")
    //     );
    //     assert_eq!(
    //         Ok(("".as_bytes(), Chirality::Allenal(2))),
    //         chirality(b"@AL2")
    //     );
    //     assert_eq!(
    //         Ok(("".as_bytes(), Chirality::SquarePlanar(3))),
    //         chirality(b"@SP3")
    //     );
    //     assert_eq!(
    //         Ok(("".as_bytes(), Chirality::TrigonalBipyramidal(1))),
    //         chirality(b"@TB1")
    //     );
    //     assert_eq!(
    //         Ok(("".as_bytes(), Chirality::TrigonalBipyramidal(11))),
    //         chirality(b"@TB11")
    //     );
    //     assert_eq!(
    //         Ok(("".as_bytes(), Chirality::Octahedral(1))),
    //         chirality(b"@OH1")
    //     );
    //     assert_eq!(
    //         Ok(("".as_bytes(), Chirality::Octahedral(11))),
    //         chirality(b"@OH11")
    //     );
    // }

    // #[test]
    // fn atom_cases() {
    //     assert_eq!(
    //         Ok((
    //             "".as_bytes(),
    //             Atom::Bracket(BracketAtom {
    //                 isotope: Some(16),
    //                 symbol: Symbol::ElementSymbol(Element::Carbon),
    //                 chiral: None,
    //                 hcount: 0,
    //                 charge: 0,
    //             })
    //         )),
    //         atom(b"[16C]")
    //     );
    //     assert_eq!(Ok(("".as_bytes(), Atom::Unknown)), atom(b"*"));
    // }

    // #[test]
    // fn chain_ethane() {
    //     assert_eq!(
    //         Ok((
    //             "".as_bytes(),
    //             Chain {
    //                 chain: Some(Box::new(Chain {
    //                     chain: None,
    //                     bond_or_dot: None,
    //                     branched_atom: BranchedAtom {
    //                         atom: Atom::AliphaticOrganic(AliphaticOrganicAtom {
    //                             element: Element::Carbon
    //                         }),
    //                         ring_bonds: vec![],
    //                         branches: vec![]
    //                     }
    //                 })),
    //                 bond_or_dot: None,
    //                 branched_atom: BranchedAtom {
    //                     atom: Atom::AliphaticOrganic(AliphaticOrganicAtom {
    //                         element: Element::Carbon
    //                     }),
    //                     ring_bonds: vec![],
    //                     branches: vec![]
    //                 }
    //             }
    //         )),
    //         chain(b"CC")
    //     );
    // }

    // #[test]
    // fn chain_fluoromethane() {
    //     assert_eq!(
    //         Ok((
    //             "".as_bytes(),
    //             Chain {
    //                 chain: Some(Box::new(Chain {
    //                     chain: None,
    //                     bond_or_dot: None,
    //                     branched_atom: BranchedAtom {
    //                         atom: Atom::AliphaticOrganic(AliphaticOrganicAtom {
    //                             element: Element::Fluorine
    //                         }),
    //                         ring_bonds: vec![],
    //                         branches: vec![]
    //                     }
    //                 })),
    //                 bond_or_dot: None,
    //                 branched_atom: BranchedAtom {
    //                     atom: Atom::AliphaticOrganic(AliphaticOrganicAtom {
    //                         element: Element::Carbon
    //                     }),
    //                     ring_bonds: vec![],
    //                     branches: vec![]
    //                 }
    //             }
    //         )),
    //         chain(b"CF")
    //     );
    // }

    // #[test]
    // fn chain_ethene() {
    //     assert_eq!(
    //         Ok((
    //             "".as_bytes(),
    //             Chain {
    //                 chain: Some(Box::new(Chain {
    //                     chain: None,
    //                     bond_or_dot: None,
    //                     branched_atom: BranchedAtom {
    //                         atom: Atom::AliphaticOrganic(AliphaticOrganicAtom {
    //                             element: Element::Carbon
    //                         }),
    //                         ring_bonds: vec![],
    //                         branches: vec![]
    //                     }
    //                 })),
    //                 bond_or_dot: Some(BondOrDot::Bond(Bond::Double)),
    //                 branched_atom: BranchedAtom {
    //                     atom: Atom::AliphaticOrganic(AliphaticOrganicAtom {
    //                         element: Element::Carbon
    //                     }),
    //                     ring_bonds: vec![],
    //                     branches: vec![]
    //                 }
    //             }
    //         )),
    //         chain(b"C=C")
    //     );
    // }

    // // 1-Oxaspiro[2.5]octane
    // #[test]
    // fn ring_and_branch_chain() {
    //     let chain = chain(b"C1CCC2(CC1)CO2");
    //     assert!(chain.is_ok());
    //     assert!(chain.unwrap().0.is_empty());
    // }

    // // Isobutane
    // #[test]
    // fn branch_isobutane() {
    //     let chain = chain(b"CC(C)C");
    //     assert!(chain.is_ok());
    //     assert!(chain.unwrap().0.is_empty());
    // }

    // // Neopentane
    // #[test]
    // fn branch_neopentane() {
    //     let chain = chain(b"CC(C)(C)C");
    //     assert!(chain.is_ok());
    //     assert!(chain.unwrap().0.is_empty());
    // }

    // // Cyclopropyloxirane
    // #[test]
    // fn rings_chain() {
    //     let chain = chain(b"C1CC1C2CO2");
    //     println!("{:?}", chain);
    //     assert!(chain.is_ok());
    //     assert!(chain.unwrap().0.is_empty());
    // }

    // #[test]
    // fn chain_trigonal_bipyramidal() {
    //     assert_eq!(
    //         Ok((
    //             "".as_bytes(),
    //             Chain {
    //                 chain: Some(Box::new(Chain {
    //                     chain: Some(Box::new(Chain {
    //                         chain: None,
    //                         bond_or_dot: None,
    //                         branched_atom: BranchedAtom {
    //                             atom: Atom::AliphaticOrganic(AliphaticOrganicAtom {
    //                                 element: Element::Nitrogen
    //                             }),
    //                             ring_bonds: vec![],
    //                             branches: vec![]
    //                         }
    //                     })),
    //                     bond_or_dot: None,
    //                     branched_atom: BranchedAtom {
    //                         atom: Atom::Bracket(BracketAtom {
    //                             isotope: None,
    //                             symbol: Symbol::ElementSymbol(Element::Arsenic),
    //                             chiral: Some(Chirality::TrigonalBipyramidal(15)),
    //                             hcount: 0,
    //                             charge: 0,
    //                         }),
    //                         ring_bonds: vec![],
    //                         branches: vec![
    //                             Branch {
    //                                 bond_or_dot: None,
    //                                 chain: Chain {
    //                                     chain: None,
    //                                     bond_or_dot: None,
    //                                     branched_atom: BranchedAtom {
    //                                         atom: Atom::AliphaticOrganic(AliphaticOrganicAtom {
    //                                             element: Element::Chlorine
    //                                         }),
    //                                         ring_bonds: vec![],
    //                                         branches: vec![]
    //                                     }
    //                                 },
    //                             },
    //                             Branch {
    //                                 bond_or_dot: None,
    //                                 chain: Chain {
    //                                     chain: None,
    //                                     bond_or_dot: None,
    //                                     branched_atom: BranchedAtom {
    //                                         atom: Atom::AliphaticOrganic(AliphaticOrganicAtom {
    //                                             element: Element::Sulfur
    //                                         }),
    //                                         ring_bonds: vec![],
    //                                         branches: vec![]
    //                                     }
    //                                 },
    //                             },
    //                             Branch {
    //                                 bond_or_dot: None,
    //                                 chain: Chain {
    //                                     chain: None,
    //                                     bond_or_dot: None,
    //                                     branched_atom: BranchedAtom {
    //                                         atom: Atom::AliphaticOrganic(AliphaticOrganicAtom {
    //                                             element: Element::Bromine
    //                                         }),
    //                                         ring_bonds: vec![],
    //                                         branches: vec![]
    //                                     }
    //                                 },
    //                             },
    //                         ]
    //                     }
    //                 })),
    //                 bond_or_dot: None,
    //                 branched_atom: BranchedAtom {
    //                     atom: Atom::AliphaticOrganic(AliphaticOrganicAtom {
    //                         element: Element::Fluorine
    //                     }),
    //                     ring_bonds: vec![],
    //                     branches: vec![]
    //                 }
    //             }
    //         )),
    //         chain(b"F[As@TB15](Cl)(S)(Br)N")
    //     );
    // }

    // #[test]
    // fn chain_sodium_chloride() {
    //     assert_eq!(
    //         Ok((
    //             "".as_bytes(),
    //             Chain {
    //                 chain: Some(Box::new(Chain {
    //                     chain: None,
    //                     bond_or_dot: None,
    //                     branched_atom: BranchedAtom {
    //                         atom: Atom::Bracket(BracketAtom {
    //                             isotope: None,
    //                             symbol: Symbol::ElementSymbol(Element::Chlorine),
    //                             chiral: None,
    //                             hcount: 0,
    //                             charge: -1,
    //                         }),
    //                         ring_bonds: vec![],
    //                         branches: vec![]
    //                     }
    //                 })),
    //                 bond_or_dot: Some(BondOrDot::Dot(Dot)),
    //                 branched_atom: BranchedAtom {
    //                     atom: Atom::Bracket(BracketAtom {
    //                         isotope: None,
    //                         symbol: Symbol::ElementSymbol(Element::Sodium),
    //                         chiral: None,
    //                         hcount: 0,
    //                         charge: 1,
    //                     }),
    //                     ring_bonds: vec![],
    //                     branches: vec![]
    //                 }
    //             }
    //         )),
    //         chain(b"[Na+].[Cl-]")
    //     );
    // }
}
