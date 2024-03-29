use crate::syntax::{
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxToken,
};
use itertools::Either;
use rowan::{NodeOrToken, SyntaxText};
use std::{iter::empty, ops::Deref};

/// Cast
pub(crate) trait Cast: Sized {
    fn cast(node: SyntaxNode) -> Option<Self>;
}

/// Root
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct Root(SyntaxNode);

impl Root {
    pub(crate) fn tree(&self) -> Option<Tree> {
        self.0.children().find_map(Tree::cast)
    }
}

impl Cast for Root {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == ROOT {
            Some(Self(node))
        } else {
            None
        }
    }
}

/// Tree
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct Tree(SyntaxNode);

impl Tree {
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

impl Cast for Tree {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == TREE {
            Some(Self(node))
        } else {
            None
        }
    }
}

/// Branch
pub(crate) enum Branch {
    Indexed(Indexed),
    Unindexed(Unindexed),
}

impl Cast for Branch {
    fn cast(node: SyntaxNode) -> Option<Self> {
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

    pub(crate) fn index(&self) -> Option<SyntaxText> {
        Some(self.0.node(INDEX)?.text())
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
pub(crate) struct Node(SyntaxNode);

impl Cast for Node {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == NODE {
            Some(Self(node))
        } else {
            None
        }
    }
}

impl Deref for Node {
    type Target = SyntaxNode;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Edge
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct Edge(SyntaxNode);

impl Cast for Edge {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == EDGE {
            Some(Self(node))
        } else {
            None
        }
    }
}

impl Deref for Edge {
    type Target = SyntaxNode;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Syntax node ext
pub(crate) trait SyntaxNodeExt {
    fn cast<T: Cast>(self) -> Option<T>;

    fn node(&self, kind: SyntaxKind) -> Option<SyntaxNode>;

    fn token(&self, kind: SyntaxKind) -> Option<SyntaxToken>;
}

impl SyntaxNodeExt for SyntaxNode {
    fn cast<T: Cast>(self) -> Option<T> {
        Cast::cast(self)
    }

    fn node(&self, kind: SyntaxKind) -> Option<SyntaxNode> {
        self.children().find(|node| node.kind() == kind)
    }

    fn token(&self, kind: SyntaxKind) -> Option<SyntaxToken> {
        self.children_with_tokens()
            .filter_map(NodeOrToken::into_token)
            .find(|token| token.kind() == kind)
    }
}
