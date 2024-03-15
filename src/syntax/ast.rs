use crate::syntax::{SyntaxKind::*, SyntaxNode, SyntaxToken};
use itertools::Either;
use rowan::NodeOrToken;
use std::iter::empty;

use super::SyntaxKind;

/// Root
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct Root(pub(crate) SyntaxNode);

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
pub(crate) struct Tree(pub(crate) SyntaxNode);

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
pub(crate) enum Branch {
    Indexed(Indexed),
    Unindexed(Unindexed),
}

impl Branch {
    #[allow(unused)]
    pub(crate) fn cast(node: SyntaxNode) -> Option<Self> {
        match node.kind() {
            INDEXED => Some(Self::Indexed(Indexed(node))),
            UNINDEXED => Some(Self::Unindexed(Unindexed(node))),
            _ => None,
        }
    }
}

/// Indexed branch
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct Indexed(SyntaxNode);

impl Indexed {
    pub(crate) fn edge(&self) -> Option<Edge> {
        self.0.children().find_map(Edge::cast)
    }

    pub(crate) fn index(&self) -> Option<usize> {
        self.0.node(INDEX)?.token(DIGIT)?.text().parse().ok()
    }
}

/// Unindexed branch
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct Unindexed(SyntaxNode);

impl Unindexed {
    pub(crate) fn edge(&self) -> Option<Edge> {
        self.0.children().find_map(Edge::cast)
    }

    pub(crate) fn tree(&self) -> Option<Tree> {
        self.0.children().find_map(Tree::cast)
    }
}

/// Branch
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct Node(pub(crate) SyntaxNode);

impl Node {
    #[allow(unused)]
    pub(crate) fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == NODE {
            Some(Self(node))
        } else {
            None
        }
    }

    // pub(crate) fn element(&self) -> Option<Element> {
    //     let implicit = self
    //         .0
    //         .children_with_tokens()
    //         .find_map(|element| element.into_token().and_then(Implicit::cast));
    //     self.0.children().find_map(Tree::cast)
    // }
}

/// Edge
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct Edge(pub(crate) SyntaxNode);

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

trait SyntaxNodeExt {
    fn node(&self, kind: SyntaxKind) -> Option<SyntaxNode>;

    fn token(&self, kind: SyntaxKind) -> Option<SyntaxToken>;
}

impl SyntaxNodeExt for SyntaxNode {
    fn node(&self, kind: SyntaxKind) -> Option<SyntaxNode> {
        self.children().find(|node| node.kind() == kind)
    }

    fn token(&self, kind: SyntaxKind) -> Option<SyntaxToken> {
        self.children_with_tokens()
            .filter_map(NodeOrToken::into_token)
            .find(|token| token.kind() == kind)
    }
}
