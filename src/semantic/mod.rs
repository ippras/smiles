use std::ops::{Deref, DerefMut};

pub use self::error::Error;

use self::error::Result;
use crate::syntax::{
    ast::{Branch, Edge, Node, Root, SyntaxNodeExt, Tree},
    SyntaxKind::*,
};
use itertools::Itertools;
use petgraph::{
    algo::astar,
    graph::{EdgeIndex, EdgeReference, NodeIndex},
    visit::{EdgeFiltered, EdgeRef, GraphBase, IntoNodeIdentifiers, NodeFiltered},
    Graph, Undirected,
};
use smol_str::ToSmolStr;

// Molecule graph
#[derive(Clone, Debug, Default)]
pub struct MoleculeGraph(Graph<Atom, Bond, Undirected>);

impl MoleculeGraph {
    pub fn carbons(
        &self,
    ) -> NodeFiltered<&Graph<Atom, Bond, Undirected>, impl Fn(NodeIndex) -> bool + '_> {
        NodeFiltered::from_fn(self, |index| {
            matches!(self[index].element, None | Some(Element::C))
        })
    }

    pub fn temp<'a>(
        &'a self,
    ) -> EdgeFiltered<&Graph<Atom, Bond, Undirected>, impl Fn(EdgeReference<'a, Bond>) -> bool + '_>
    {
        EdgeFiltered::from_fn(self, |edge| {
            matches!(self[edge.source()].element, None | Some(Element::C))
                && matches!(self[edge.target()].element, None | Some(Element::C))
        })
    }

    pub fn bonds<'a>(
        &'a self,
        f: impl Fn(Bond) -> bool + 'a,
    ) -> EdgeFiltered<&Graph<Atom, Bond, Undirected>, impl Fn(EdgeReference<'a, Bond>) -> bool + '_>
    {
        EdgeFiltered::from_fn(self, move |edge| {
            matches!(self[edge.source()].element, None | Some(Element::C))
                && matches!(self[edge.target()].element, None | Some(Element::C))
                && f(*edge.weight())
        })
    }

    pub fn the_longest_carbon_chain(&self) -> Vec<NodeIndex> {
        let carbons = self.carbons();
        let paths = carbons
            .node_identifiers()
            .permutations(2)
            .filter_map(|endpoints| {
                astar(
                    &carbons,
                    endpoints[0],
                    |finish| finish == endpoints[1],
                    |_| 1,
                    |_| 0,
                )
            });
        paths.max_by_key(|&(cost, _)| cost).unwrap_or_default().1
    }

    fn hydrogen_filling(&mut self) {
        for index in self.node_indices() {
            let atom = self.node_weight(index).unwrap();
            let count = match atom.element {
                Some(Element::C) => 4,
                Some(Element::P) => 5,
                Some(Element::O) => 2,
                _ => unimplemented!("Atom is unimplemented"),
            };
            let current: usize = self
                .edges(index)
                .map(|bond| match bond.weight() {
                    Bond::Single => 1,
                    Bond::Double => 2,
                    Bond::Triple => 3,
                    Bond::Quadruple => 4,
                    _ => unimplemented!("Bond is unimplemented"),
                })
                .sum();
            for _ in 0..count - current {
                self.hydrogen(index);
            }
        }
    }

    fn hydrogen(&mut self, from: NodeIndex) -> EdgeIndex {
        let to = self.add_node(Atom {
            element: Some(Element::H),
            ..Default::default()
        });
        self.add_edge(from, to, Bond::Single)
    }
}

impl Deref for MoleculeGraph {
    type Target = Graph<Atom, Bond, Undirected>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MoleculeGraph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TryFrom<Root> for MoleculeGraph {
    type Error = Error;

    fn try_from(value: Root) -> Result<Self, Self::Error> {
        let mut graph = MoleculeGraph(Graph::new_undirected());
        walk(&mut graph, &value.tree().ok_or(Error::TreeNotFound)?)?;
        return Ok(graph);

        fn walk(graph: &mut MoleculeGraph, tree: &Tree) -> Result<NodeIndex> {
            let node = tree.node().ok_or(Error::NodeNotFound)?;
            let from = graph.add_node(node.try_into()?);
            for branch in tree.branches() {
                match branch {
                    Branch::Indexed(indexed) => {
                        // TODO
                    }
                    Branch::Unindexed(unindexed) => {
                        let tree = unindexed.tree().ok_or(Error::TreeNotFound)?;
                        let to = walk(graph, &tree)?;
                        let edge = unindexed.edge().map_or_else(Default::default, Into::into);
                        graph.add_edge(from, to, edge);
                    }
                }
            }
            Ok(from)
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
#[derive(Clone, Copy, Debug, Default)]
pub struct Atom {
    pub isotope: Option<u16>,
    pub element: Option<Element>,
    pub parity: Option<Parity>,
    pub charge: i8,
}

impl TryFrom<Node> for Atom {
    type Error = Error;

    fn try_from(value: Node) -> Result<Self> {
        Ok(Atom {
            isotope: isotope(&value)?,
            element: element(&value)?,
            parity: None,
            charge: charge(&value)?,
        })
    }
}

/// Bond
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Bond {
    #[default]
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
        let text = value.text();
        if text == "-" {
            Bond::Single
        } else if text == "=" {
            Bond::Double
        } else if text == "#" {
            Bond::Triple
        } else if text == "$" {
            Bond::Quadruple
        } else if text == ":" {
            Bond::Aromatic
        } else if text == "/" {
            Bond::Up
        } else if text == "\\" {
            Bond::Down
        } else {
            unreachable!();
        }
    }
}

/// Parity
#[derive(Clone, Copy, Debug)]
pub enum Parity {
    Clockwise,
    Counterclockwise,
}

fn isotope(node: &Node) -> Result<Option<u16>> {
    match node.node(ISOTOPE) {
        Some(node) => Ok(Some(node.to_smolstr().parse()?)),
        None => Ok(None),
    }
}

fn element(node: &Node) -> Result<Option<Element>> {
    match node.node(ELEMENT) {
        Some(node) => match &*node.to_smolstr() {
            "H" => Ok(Some(Element::H)),
            "He" => Ok(Some(Element::He)),
            "Li" => Ok(Some(Element::Li)),
            "Be" => Ok(Some(Element::Be)),
            "B" => Ok(Some(Element::B)),
            "C" => Ok(Some(Element::C)),
            "N" => Ok(Some(Element::N)),
            "O" => Ok(Some(Element::O)),
            "F" => Ok(Some(Element::F)),
            "Ne" => Ok(Some(Element::Ne)),
            "Na" => Ok(Some(Element::Na)),
            "Mg" => Ok(Some(Element::Mg)),
            "Al" => Ok(Some(Element::Al)),
            "Si" => Ok(Some(Element::Si)),
            "P" => Ok(Some(Element::P)),
            "S" => Ok(Some(Element::S)),
            "Cl" => Ok(Some(Element::Cl)),
            "Ar" => Ok(Some(Element::Ar)),
            "K" => Ok(Some(Element::K)),
            "Ca" => Ok(Some(Element::Ca)),
            "Sc" => Ok(Some(Element::Sc)),
            "Ti" => Ok(Some(Element::Ti)),
            "V" => Ok(Some(Element::V)),
            "Cr" => Ok(Some(Element::Cr)),
            "Mn" => Ok(Some(Element::Mn)),
            "Fe" => Ok(Some(Element::Fe)),
            "Co" => Ok(Some(Element::Co)),
            "Ni" => Ok(Some(Element::Ni)),
            "Cu" => Ok(Some(Element::Cu)),
            "Zn" => Ok(Some(Element::Zn)),
            "Ga" => Ok(Some(Element::Ga)),
            "Ge" => Ok(Some(Element::Ge)),
            "As" => Ok(Some(Element::As)),
            "Se" => Ok(Some(Element::Se)),
            "Br" => Ok(Some(Element::Br)),
            "Kr" => Ok(Some(Element::Kr)),
            "Rb" => Ok(Some(Element::Rb)),
            "Sr" => Ok(Some(Element::Sr)),
            "Y" => Ok(Some(Element::Y)),
            "Zr" => Ok(Some(Element::Zr)),
            "Nb" => Ok(Some(Element::Nb)),
            "Mo" => Ok(Some(Element::Mo)),
            "Tc" => Ok(Some(Element::Tc)),
            "Ru" => Ok(Some(Element::Ru)),
            "Rh" => Ok(Some(Element::Rh)),
            "Pd" => Ok(Some(Element::Pd)),
            "Ag" => Ok(Some(Element::Ag)),
            "Cd" => Ok(Some(Element::Cd)),
            "In" => Ok(Some(Element::In)),
            "Sn" => Ok(Some(Element::Sn)),
            "Sb" => Ok(Some(Element::Sb)),
            "Te" => Ok(Some(Element::Te)),
            "I" => Ok(Some(Element::I)),
            "Xe" => Ok(Some(Element::Xe)),
            "Cs" => Ok(Some(Element::Cs)),
            "Ba" => Ok(Some(Element::Ba)),
            "La" => Ok(Some(Element::La)),
            "Ce" => Ok(Some(Element::Ce)),
            "Pr" => Ok(Some(Element::Pr)),
            "Nd" => Ok(Some(Element::Nd)),
            "Pm" => Ok(Some(Element::Pm)),
            "Sm" => Ok(Some(Element::Sm)),
            "Eu" => Ok(Some(Element::Eu)),
            "Gd" => Ok(Some(Element::Gd)),
            "Tb" => Ok(Some(Element::Tb)),
            "Dy" => Ok(Some(Element::Dy)),
            "Ho" => Ok(Some(Element::Ho)),
            "Er" => Ok(Some(Element::Er)),
            "Tm" => Ok(Some(Element::Tm)),
            "Yb" => Ok(Some(Element::Yb)),
            "Lu" => Ok(Some(Element::Lu)),
            "Hf" => Ok(Some(Element::Hf)),
            "Ta" => Ok(Some(Element::Ta)),
            "W" => Ok(Some(Element::W)),
            "Re" => Ok(Some(Element::Re)),
            "Os" => Ok(Some(Element::Os)),
            "Ir" => Ok(Some(Element::Ir)),
            "Pt" => Ok(Some(Element::Pt)),
            "Au" => Ok(Some(Element::Au)),
            "Hg" => Ok(Some(Element::Hg)),
            "Tl" => Ok(Some(Element::Tl)),
            "Pb" => Ok(Some(Element::Pb)),
            "Bi" => Ok(Some(Element::Bi)),
            "Po" => Ok(Some(Element::Po)),
            "At" => Ok(Some(Element::At)),
            "Rn" => Ok(Some(Element::Rn)),
            "Fr" => Ok(Some(Element::Fr)),
            "Ra" => Ok(Some(Element::Ra)),
            "Ac" => Ok(Some(Element::Ac)),
            "Th" => Ok(Some(Element::Th)),
            "Pa" => Ok(Some(Element::Pa)),
            "U" => Ok(Some(Element::U)),
            "Np" => Ok(Some(Element::Np)),
            "Pu" => Ok(Some(Element::Pu)),
            "Am" => Ok(Some(Element::Am)),
            "Cm" => Ok(Some(Element::Cm)),
            "Bk" => Ok(Some(Element::Bk)),
            "Cf" => Ok(Some(Element::Cf)),
            "Es" => Ok(Some(Element::Es)),
            "Fm" => Ok(Some(Element::Fm)),
            "Md" => Ok(Some(Element::Md)),
            "No" => Ok(Some(Element::No)),
            "Lr" => Ok(Some(Element::Lr)),
            "Rf" => Ok(Some(Element::Rf)),
            "Db" => Ok(Some(Element::Db)),
            "Sg" => Ok(Some(Element::Sg)),
            "Bh" => Ok(Some(Element::Bh)),
            "Hs" => Ok(Some(Element::Hs)),
            "Mt" => Ok(Some(Element::Mt)),
            "Ds" => Ok(Some(Element::Ds)),
            "Rg" => Ok(Some(Element::Rg)),
            "Cn" => Ok(Some(Element::Cn)),
            "Nh" => Ok(Some(Element::Nh)),
            "Fl" => Ok(Some(Element::Fl)),
            "Mc" => Ok(Some(Element::Mc)),
            "Lv" => Ok(Some(Element::Lv)),
            "Ts" => Ok(Some(Element::Ts)),
            "Og" => Ok(Some(Element::Og)),
            "*" => Ok(None),
            _ => unreachable!(),
        },
        None => Err(Error::ElementNotFound),
    }
}

fn charge(node: &Node) -> Result<i8> {
    match node.node(CHARGE).and_then(|charge| charge.node(SIGNED)) {
        Some(signed) if signed.node(UNSIGNED).is_some() => Ok(signed.to_smolstr().parse()?),
        Some(signed) if signed.token(MINUS).is_some() => Ok(-1),
        Some(_) => Ok(1),
        None => Ok(0),
    }
}

mod error;
