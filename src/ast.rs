use crate::syntax::{SyntaxKind::*, SyntaxNode, SyntaxToken};

/// Root
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub(crate) struct Root(SyntaxNode);

impl Root {
    #[allow(unused)]
    pub(crate) fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == ROOT {
            Some(Self(node))
        } else {
            None
        }
    }

    pub(crate) fn tree(&self) -> Option<Tree> {
        self.0.children().find_map(Tree::cast)
    }
}

/// Tree
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub(crate) struct Tree(SyntaxNode);

impl Tree {
    #[allow(unused)]
    pub(crate) fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == TREE {
            Some(Self(node))
        } else {
            None
        }
    }

    pub(crate) fn node(&self) -> Option<Node> {
        self.0.children().find_map(Node::cast)
    }

    pub(crate) fn branches(&self) -> impl Iterator<Item = Branch> {
        self.0.children().flat_map(|node| {
            if node.kind() == BRANCHES {
                Either::Left(node.children().filter_map(Branch::cast))
            } else {
                Either::Right(empty())
            }
        })
    }
}

/// Branch
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub(crate) struct Branch(SyntaxNode);

impl Branch {
    #[allow(unused)]
    pub(crate) fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == BRANCH {
            Some(Self(node))
        } else {
            None
        }
    }

    pub(crate) fn edge(&self) -> Option<Edge> {
        self.0.children().find_map(Edge::cast)
    }

    pub(crate) fn tree(&self) -> Option<Tree> {
        self.0.children().find_map(Tree::cast)
    }
}

/// Branch
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub(crate) struct Node(SyntaxNode);

impl Node {
    #[allow(unused)]
    pub(crate) fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == NODE {
            Some(Self(node))
        } else {
            None
        }
    }
}

/// Edge
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub(crate) struct Edge(SyntaxNode);

impl Edge {
    #[allow(unused)]
    pub(crate) fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == EDGE {
            Some(Self(node))
        } else {
            None
        }
    }
}

/// Implicit
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub(crate) struct Implicit(SyntaxToken);

impl Implicit {
    #[allow(unused)]
    pub(crate) fn cast(token: SyntaxToken) -> Option<Self> {
        if token.kind() == IMPLICIT {
            Some(Self(token))
        } else {
            None
        }
    }
}

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
                NUMBER => Some(Bond::Triple),
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
