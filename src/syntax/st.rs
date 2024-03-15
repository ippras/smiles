use crate::syntax::{
    ast::{Edge, Implicit, Node},
    SyntaxKind::*,
};

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

/// Atom
#[derive(Clone, Copy, Debug)]
pub struct Atom {
    isotope: Option<u16>,
    element: Element,
    parity: Option<Parity>,
    charge: i8,
}

impl From<Node> for Atom {
    fn from(value: Node) -> Self {
        let implicit = value
            .0
            .children_with_tokens()
            .find_map(|element| element.into_token().and_then(Implicit::cast));
        println!("{implicit:?}");
        Atom {
            isotope: None,
            element: Element::Ac,
            parity: None,
            charge: 0,
        }
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

impl From<Edge> for Bond {
    fn from(value: Edge) -> Self {
        value
            .0
            .children_with_tokens()
            .find_map(|element| match element.kind() {
                MINUS => Some(Bond::Single),
                EQUALS => Some(Bond::Double),
                HASH => Some(Bond::Triple),
                DOLLAR => Some(Bond::Quadruple),
                COLON => Some(Bond::Aromatic),
                SLASH => Some(Bond::Up),
                BACKSLASH => Some(Bond::Down),
                _ => None,
            })
            .unwrap()
    }
}

/// Parity
#[derive(Clone, Copy, Debug)]
pub enum Parity {
    Clockwise,
    Counterclockwise,
}
