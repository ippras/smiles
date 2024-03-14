use crate::syntax::{SyntaxKind::*, SyntaxNode};

/// Root
#[derive(Eq, Hash, PartialEq)]
#[repr(transparent)]
struct Root(SyntaxNode);

impl Root {
    #[allow(unused)]
    fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == ROOT {
            Some(Self(node))
        } else {
            None
        }
    }
}

// /// Bond
// #[derive(Eq, Hash, PartialEq)]
// #[repr(transparent)]
// struct Bond(SyntaxNode);

// impl Bond {
//     #[allow(unused)]
//     fn cast(node: SyntaxNode) -> Option<Self> {
//         if node.kind() == BOND {
//             Some(Self(node))
//         } else {
//             None
//         }
//     }
// }

/// Element
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
pub struct Atom {
    isotope: Option<u16>,
    element: Element,
    parity: Option<Parity>,
    charge: i8,
}

/// Bond
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
pub enum Parity {
    Clockwise,
    Counterclockwise,
}
