//! BRACKET ATOMS
//!
//! [Bracket Atoms](http://opensmiles.org/opensmiles.html#inatoms)

use super::class;
use crate::{
    charge::charge,
    chirality::{chiral, Chiral},
    hydrogens::hydrogens,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, u16},
    combinator::{map, map_res, opt},
    error::{Error, ErrorKind},
    sequence::{delimited, tuple},
    IResult,
};

/// Bracket atom
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Copy, Clone, Hash)]
pub struct Bracket {
    pub isotope: Option<u16>,
    pub symbol: Symbol,
    pub chiral: Option<Chiral>,
    pub hydrogens: u8,
    pub charge: i8,
    pub class: Option<u64>,
}

pub fn bracket_atom(input: &str) -> IResult<&str, Bracket> {
    map(
        delimited(
            char('['),
            tuple((
                opt(isotope),
                symbol,
                opt(chiral),
                hydrogens,
                charge,
                opt(class),
            )),
            char(']'),
        ),
        |(isotope, symbol, chiral, hydrogens, charge, class)| Bracket {
            isotope,
            symbol,
            chiral,
            hydrogens,
            charge,
            class,
        },
    )(input)
}

/// Symbol
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Symbol {
    Element(ElementSymbol),
    Aromatic(AromaticSymbol),
    Unknown,
}

pub fn symbol(input: &str) -> IResult<&str, Symbol> {
    alt((
        map(element_symbols, Symbol::Element),
        map(aromatic_symbols, Symbol::Aromatic),
        map(char('*'), |_| Symbol::Unknown),
    ))(input)
}

pub fn isotope(input: &str) -> IResult<&str, u16> {
    map_res(u16, |isotope| {
        // A general-purpose SMILES parser must accept at least three digits for
        // the isotope and values from 0 to 999.
        if isotope > 999 {
            return Err(Error::new(input, ErrorKind::MapRes));
        }
        Ok(isotope)
    })(input)
}

/// Element symbol
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ElementSymbol {
    H,
    He,
    Li,
    Be,
    B,
    C,
    N,
    O,
    F,
    Ne,
    Na,
    Mg,
    Al,
    Si,
    P,
    S,
    Cl,
    Ar,
    K,
    Ca,
    Sc,
    Ti,
    V,
    Cr,
    Mn,
    Fe,
    Co,
    Ni,
    Cu,
    Zn,
    Ga,
    Ge,
    As,
    Se,
    Br,
    Kr,
    Rb,
    Sr,
    Y,
    Zr,
    Nb,
    Mo,
    Tc,
    Ru,
    Rh,
    Pd,
    Ag,
    Cd,
    In,
    Sn,
    Sb,
    Te,
    I,
    Xe,
    Cs,
    Ba,
    Hf,
    Ta,
    W,
    Re,
    Os,
    Ir,
    Pt,
    Au,
    Hg,
    Tl,
    Pb,
    Bi,
    Po,
    At,
    Rn,
    Fr,
    Ra,
    Rf,
    Db,
    Sg,
    Bh,
    Hs,
    Mt,
    Ds,
    Rg,
    Cn,
    Fl,
    Lv,
    La,
    Ce,
    Pr,
    Nd,
    Pm,
    Sm,
    Eu,
    Gd,
    Tb,
    Dy,
    Ho,
    Er,
    Tm,
    Yb,
    Lu,
    Ac,
    Th,
    Pa,
    U,
    Np,
    Pu,
    Am,
    Cm,
    Bk,
    Cf,
    Es,
    Fm,
    Md,
    No,
    Lr,
}

pub fn element_symbols(input: &str) -> IResult<&str, ElementSymbol> {
    alt((
        alt((
            map(tag("He"), |_| ElementSymbol::He),
            map(tag("Li"), |_| ElementSymbol::Li),
            map(tag("Be"), |_| ElementSymbol::Be),
            map(tag("Ne"), |_| ElementSymbol::Ne),
            map(tag("Na"), |_| ElementSymbol::Na),
            map(tag("Mg"), |_| ElementSymbol::Mg),
            map(tag("Al"), |_| ElementSymbol::Al),
            map(tag("Si"), |_| ElementSymbol::Si),
            map(tag("Cl"), |_| ElementSymbol::Cl),
            map(tag("Ar"), |_| ElementSymbol::Ar),
            map(tag("Ca"), |_| ElementSymbol::Ca),
            map(tag("Sc"), |_| ElementSymbol::Sc),
            map(tag("Ti"), |_| ElementSymbol::Ti),
            map(tag("Cr"), |_| ElementSymbol::Cr),
            map(tag("Mn"), |_| ElementSymbol::Mn),
            map(tag("Fe"), |_| ElementSymbol::Fe),
            map(tag("Co"), |_| ElementSymbol::Co),
            map(tag("Ni"), |_| ElementSymbol::Ni),
            map(tag("Cu"), |_| ElementSymbol::Cu),
            map(tag("Zn"), |_| ElementSymbol::Zn),
            map(tag("Ga"), |_| ElementSymbol::Ga),
        )),
        alt((
            map(tag("Ge"), |_| ElementSymbol::Ge),
            map(tag("As"), |_| ElementSymbol::As),
            map(tag("Se"), |_| ElementSymbol::Se),
            map(tag("Br"), |_| ElementSymbol::Br),
            map(tag("Kr"), |_| ElementSymbol::Kr),
            map(tag("Rb"), |_| ElementSymbol::Rb),
            map(tag("Sr"), |_| ElementSymbol::Sr),
            map(tag("Zr"), |_| ElementSymbol::Zr),
            map(tag("Nb"), |_| ElementSymbol::Nb),
            map(tag("Mo"), |_| ElementSymbol::Mo),
            map(tag("Tc"), |_| ElementSymbol::Tc),
            map(tag("Ru"), |_| ElementSymbol::Ru),
            map(tag("Rh"), |_| ElementSymbol::Rh),
            map(tag("Pd"), |_| ElementSymbol::Pd),
            map(tag("Ag"), |_| ElementSymbol::Ag),
            map(tag("Cd"), |_| ElementSymbol::Cd),
            map(tag("In"), |_| ElementSymbol::In),
            map(tag("Sn"), |_| ElementSymbol::Sn),
            map(tag("Sb"), |_| ElementSymbol::Sb),
            map(tag("Te"), |_| ElementSymbol::Te),
            map(tag("Xe"), |_| ElementSymbol::Xe),
        )),
        alt((
            map(tag("Cs"), |_| ElementSymbol::Cs),
            map(tag("Ba"), |_| ElementSymbol::Ba),
            map(tag("Hf"), |_| ElementSymbol::Hf),
            map(tag("Ta"), |_| ElementSymbol::Ta),
            map(tag("Re"), |_| ElementSymbol::Re),
            map(tag("Os"), |_| ElementSymbol::Os),
            map(tag("Ir"), |_| ElementSymbol::Ir),
            map(tag("Pt"), |_| ElementSymbol::Pt),
            map(tag("Au"), |_| ElementSymbol::Au),
            map(tag("Hg"), |_| ElementSymbol::Hg),
            map(tag("Tl"), |_| ElementSymbol::Tl),
            map(tag("Pb"), |_| ElementSymbol::Pb),
            map(tag("Bi"), |_| ElementSymbol::Bi),
            map(tag("Po"), |_| ElementSymbol::Po),
            map(tag("At"), |_| ElementSymbol::At),
            map(tag("Rn"), |_| ElementSymbol::Rn),
            map(tag("Fr"), |_| ElementSymbol::Fr),
            map(tag("Ra"), |_| ElementSymbol::Ra),
            map(tag("Rf"), |_| ElementSymbol::Rf),
            map(tag("Db"), |_| ElementSymbol::Db),
            map(tag("Sg"), |_| ElementSymbol::Sg),
        )),
        alt((
            map(tag("Bh"), |_| ElementSymbol::Bh),
            map(tag("Hs"), |_| ElementSymbol::Hs),
            map(tag("Mt"), |_| ElementSymbol::Mt),
            map(tag("Ds"), |_| ElementSymbol::Ds),
            map(tag("Rg"), |_| ElementSymbol::Rg),
            map(tag("Cn"), |_| ElementSymbol::Cn),
            map(tag("Fl"), |_| ElementSymbol::Fl),
            map(tag("Lv"), |_| ElementSymbol::Lv),
            map(tag("La"), |_| ElementSymbol::La),
            map(tag("Ce"), |_| ElementSymbol::Ce),
            map(tag("Pr"), |_| ElementSymbol::Pr),
            map(tag("Nd"), |_| ElementSymbol::Nd),
            map(tag("Pm"), |_| ElementSymbol::Pm),
            map(tag("Sm"), |_| ElementSymbol::Sm),
            map(tag("Eu"), |_| ElementSymbol::Eu),
            map(tag("Gd"), |_| ElementSymbol::Gd),
            map(tag("Tb"), |_| ElementSymbol::Tb),
            map(tag("Dy"), |_| ElementSymbol::Dy),
            map(tag("Ho"), |_| ElementSymbol::Ho),
            map(tag("Er"), |_| ElementSymbol::Er),
            map(tag("Tm"), |_| ElementSymbol::Tm),
        )),
        alt((
            map(tag("Yb"), |_| ElementSymbol::Yb),
            map(tag("Lu"), |_| ElementSymbol::Lu),
            map(tag("Ac"), |_| ElementSymbol::Ac),
            map(tag("Th"), |_| ElementSymbol::Th),
            map(tag("Pa"), |_| ElementSymbol::Pa),
            map(tag("Np"), |_| ElementSymbol::Np),
            map(tag("Pu"), |_| ElementSymbol::Pu),
            map(tag("Am"), |_| ElementSymbol::Am),
            map(tag("Cm"), |_| ElementSymbol::Cm),
            map(tag("Bk"), |_| ElementSymbol::Bk),
            map(tag("Cf"), |_| ElementSymbol::Cf),
            map(tag("Es"), |_| ElementSymbol::Es),
            map(tag("Fm"), |_| ElementSymbol::Fm),
            map(tag("Md"), |_| ElementSymbol::Md),
            map(tag("No"), |_| ElementSymbol::No),
            map(tag("Lr"), |_| ElementSymbol::Lr),
        )),
        alt((
            map(tag("H"), |_| ElementSymbol::H),
            map(tag("B"), |_| ElementSymbol::B),
            map(tag("C"), |_| ElementSymbol::C),
            map(tag("N"), |_| ElementSymbol::N),
            map(tag("O"), |_| ElementSymbol::O),
            map(tag("F"), |_| ElementSymbol::F),
            map(tag("P"), |_| ElementSymbol::P),
            map(tag("S"), |_| ElementSymbol::S),
            map(tag("K"), |_| ElementSymbol::K),
            map(tag("V"), |_| ElementSymbol::V),
            map(tag("Y"), |_| ElementSymbol::Y),
            map(tag("I"), |_| ElementSymbol::I),
            map(tag("W"), |_| ElementSymbol::W),
            map(tag("U"), |_| ElementSymbol::U),
        )),
    ))(input)
}

/// Aromatic symbol
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AromaticSymbol {
    B,
    C,
    N,
    O,
    P,
    S,
    Se,
    As,
}

pub fn aromatic_symbols(input: &str) -> IResult<&str, AromaticSymbol> {
    alt((
        alt((
            map(tag("se"), |_| AromaticSymbol::Se),
            map(tag("as"), |_| AromaticSymbol::As),
        )),
        alt((
            map(tag("b"), |_| AromaticSymbol::B),
            map(tag("c"), |_| AromaticSymbol::C),
            map(tag("n"), |_| AromaticSymbol::N),
            map(tag("o"), |_| AromaticSymbol::O),
            map(tag("p"), |_| AromaticSymbol::P),
            map(tag("s"), |_| AromaticSymbol::S),
        )),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol_cases() {
        assert_eq!(Ok(("", Symbol::Element(ElementSymbol::He))), symbol("[U]"));
        assert_eq!(Ok(("", Symbol::Element(ElementSymbol::He))), symbol("He"));
        assert_eq!(Ok(("", Symbol::Unknown)), symbol("*"));
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
}
