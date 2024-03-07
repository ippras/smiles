#![feature(box_patterns)]

use smiles::{
    atom::Atom,
    bracket::{Bracket, ElementSymbol, Symbol},
    chain::{chain, Chain, Link},
    organic::{Aliphatic, Organic},
    smiles,
};

mod atoms {
    use super::*;

    #[test]
    /// Atomic Symbol
    fn atomic_symbol() {
        let (_, chain) = smiles("[C]").unwrap();
        assert_eq!(
            chain.link().atom,
            Atom::Bracket(Bracket {
                isotope: None,
                symbol: Symbol::Element(ElementSymbol::C),
                chiral: None,
                hydrogens: None,
                charge: None,
                class: None,
            })
        )
    }
}
