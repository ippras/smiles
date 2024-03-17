use crate::{
    errors::{Error, SemanticError, UnknownElement},
    syntax::ast::{Edge, Node},
};
use std::str::FromStr;

/// Element
#[derive(Clone, Copy, Debug)]
pub enum Element {
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
    Rf,
    Db,
    Sg,
    Bh,
    Hs,
    Mt,
    Ds,
    Rg,
    Cn,
    Nh,
    Fl,
    Mc,
    Lv,
    Ts,
    Og,
}

impl FromStr for Element {
    type Err = UnknownElement;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match &*value.to_lowercase() {
            "h" => Ok(Self::H),
            "he" => Ok(Self::He),
            "li" => Ok(Self::Li),
            "be" => Ok(Self::Be),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            "n" => Ok(Self::N),
            "o" => Ok(Self::O),
            "f" => Ok(Self::F),
            "ne" => Ok(Self::Ne),
            "na" => Ok(Self::Na),
            "mg" => Ok(Self::Mg),
            "al" => Ok(Self::Al),
            "si" => Ok(Self::Si),
            "p" => Ok(Self::P),
            "s" => Ok(Self::S),
            "cl" => Ok(Self::Cl),
            "ar" => Ok(Self::Ar),
            "k" => Ok(Self::K),
            "ca" => Ok(Self::Ca),
            "sc" => Ok(Self::Sc),
            "ti" => Ok(Self::Ti),
            "v" => Ok(Self::V),
            "cr" => Ok(Self::Cr),
            "mn" => Ok(Self::Mn),
            "fe" => Ok(Self::Fe),
            "co" => Ok(Self::Co),
            "ni" => Ok(Self::Ni),
            "cu" => Ok(Self::Cu),
            "zn" => Ok(Self::Zn),
            "ga" => Ok(Self::Ga),
            "ge" => Ok(Self::Ge),
            "as" => Ok(Self::As),
            "se" => Ok(Self::Se),
            "br" => Ok(Self::Br),
            "kr" => Ok(Self::Kr),
            "rb" => Ok(Self::Rb),
            "sr" => Ok(Self::Sr),
            "y" => Ok(Self::Y),
            "zr" => Ok(Self::Zr),
            "nb" => Ok(Self::Nb),
            "mo" => Ok(Self::Mo),
            "tc" => Ok(Self::Tc),
            "ru" => Ok(Self::Ru),
            "rh" => Ok(Self::Rh),
            "pd" => Ok(Self::Pd),
            "ag" => Ok(Self::Ag),
            "cd" => Ok(Self::Cd),
            "in" => Ok(Self::In),
            "sn" => Ok(Self::Sn),
            "sb" => Ok(Self::Sb),
            "te" => Ok(Self::Te),
            "i" => Ok(Self::I),
            "xe" => Ok(Self::Xe),
            "cs" => Ok(Self::Cs),
            "ba" => Ok(Self::Ba),
            "la" => Ok(Self::La),
            "ce" => Ok(Self::Ce),
            "pr" => Ok(Self::Pr),
            "nd" => Ok(Self::Nd),
            "pm" => Ok(Self::Pm),
            "sm" => Ok(Self::Sm),
            "eu" => Ok(Self::Eu),
            "gd" => Ok(Self::Gd),
            "tb" => Ok(Self::Tb),
            "dy" => Ok(Self::Dy),
            "ho" => Ok(Self::Ho),
            "er" => Ok(Self::Er),
            "tm" => Ok(Self::Tm),
            "yb" => Ok(Self::Yb),
            "lu" => Ok(Self::Lu),
            "hf" => Ok(Self::Hf),
            "ta" => Ok(Self::Ta),
            "w" => Ok(Self::W),
            "re" => Ok(Self::Re),
            "os" => Ok(Self::Os),
            "ir" => Ok(Self::Ir),
            "pt" => Ok(Self::Pt),
            "au" => Ok(Self::Au),
            "hg" => Ok(Self::Hg),
            "tl" => Ok(Self::Tl),
            "pb" => Ok(Self::Pb),
            "bi" => Ok(Self::Bi),
            "po" => Ok(Self::Po),
            "at" => Ok(Self::At),
            "rn" => Ok(Self::Rn),
            "fr" => Ok(Self::Fr),
            "ra" => Ok(Self::Ra),
            "ac" => Ok(Self::Ac),
            "th" => Ok(Self::Th),
            "pa" => Ok(Self::Pa),
            "u" => Ok(Self::U),
            "np" => Ok(Self::Np),
            "pu" => Ok(Self::Pu),
            "am" => Ok(Self::Am),
            "cm" => Ok(Self::Cm),
            "bk" => Ok(Self::Bk),
            "cf" => Ok(Self::Cf),
            "es" => Ok(Self::Es),
            "fm" => Ok(Self::Fm),
            "md" => Ok(Self::Md),
            "no" => Ok(Self::No),
            "lr" => Ok(Self::Lr),
            "rf" => Ok(Self::Rf),
            "db" => Ok(Self::Db),
            "sg" => Ok(Self::Sg),
            "bh" => Ok(Self::Bh),
            "hs" => Ok(Self::Hs),
            "mt" => Ok(Self::Mt),
            "ds" => Ok(Self::Ds),
            "rg" => Ok(Self::Rg),
            "cn" => Ok(Self::Cn),
            "nh" => Ok(Self::Nh),
            "fl" => Ok(Self::Fl),
            "mc" => Ok(Self::Mc),
            "lv" => Ok(Self::Lv),
            "ts" => Ok(Self::Ts),
            "og" => Ok(Self::Og),
            "*" => Ok(Self::Og),
            _ => Err(UnknownElement),
        }
    }
}

/// Atom
#[derive(Clone, Copy, Debug)]
pub struct Atom {
    isotope: Option<u16>,
    element: Option<Element>,
    parity: Option<Parity>,
    charge: i8,
}

impl TryFrom<Node> for Atom {
    type Error = SemanticError;

    fn try_from(value: Node) -> Result<Self, Self::Error> {
        Ok(Atom {
            isotope: value.isotope()?,
            element: value.element()?,
            parity: None,
            charge: value.charge()?.unwrap_or_default(),
        })
    }
}

/// Bond
#[derive(Clone, Copy, Debug)]
pub enum Bond {
    Single,
    Double,
    Triple,
    Quadruple,
    Aromatic,
    Up,
    Down,
}

/// Parity
#[derive(Clone, Copy, Debug)]
pub enum Parity {
    Clockwise,
    Counterclockwise,
}
