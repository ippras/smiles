//! SMILES STRINGS
//!
//! [opensmiles.org](http://opensmiles.org/opensmiles.html)

// https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html#Recursive-descent-and-left-recursion
// https://matklad.github.io/2020/04/15/from-pratt-to-dijkstra.html

#![feature(let_chains)]

use nom::{
    branch::alt,
    character::complete::multispace0,
    combinator::{map, map_parser, opt},
    error::{Error, ErrorKind, FromExternalError},
    sequence::terminated,
    IResult,
};
use std::str::FromStr;

// // smiles ::= terminator | chain terminator
// // terminator ::= SPACE | TAB | LINEFEED | CARRIAGE_RETURN | END_OF_STRING

// [Rh-](Cl)(Cl)(Cl)(Cl)$[Rh-](Cl)(Cl)(Cl)Cl

// pub mod ast;
// pub mod parser;
// pub mod syntax;

use anyhow::Result;
use balsa::{graph::Builder, read::read};
use gamma::{
    graph::{DefaultGraph, Graph},
    traversal::Step,
};

fn main() -> Result<()> {
    let mut builder = Builder::new();

    read("O=C(O)CCCCCCCCCCC", &mut builder).unwrap();
    let molecule = builder.build();
    println!("{molecule:#?}");

    let graph = DefaultGraph::try_from(molecule)?;
    // let molecule = read_smiles(&"O=C(O)CCCC=CCCCCCC", None).unwrap();
    // // let traversal = DepthFirst::new(&molecule, 0).expect("traversal error");
    // println!("{molecule:?}");
    // let atoms = molecule.ids().map(|id| molecule.atom(id));
    // println!("{:?}", atoms.collect::<Vec<_>>());
    // let double_bonds =
    // molecule
    //     .edges()
    //     .filter_map(|(sid, tid)| {
    //         let bond = molecule.bond_order(sid, tid).ok()?;
    //         if molecule.atom(sid).ok()?.element != Some(Element::C) {
    //             return None;
    //         }
    //         if molecule.atom(tid).ok()?.element != Some(Element::C) {
    //             return None;
    //         }
    //         if bond != 2.0 {
    //             return None;
    //         }
    //         Some((sid, tid))
    //     })
    //     .count();
    // println!("{:?}", double_bonds);
    Ok(())
}
