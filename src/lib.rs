//! SMILES STRINGS
//!
//! [opensmiles.org](http://opensmiles.org/opensmiles.html)

// https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html#Recursive-descent-and-left-recursion
// https://matklad.github.io/2020/04/15/from-pratt-to-dijkstra.html

#![feature(let_chains)]

// // smiles ::= terminator | chain terminator
// // terminator ::= SPACE | TAB | LINEFEED | CARRIAGE_RETURN | END_OF_STRING

// [Rh-](Cl)(Cl)(Cl)(Cl)$[Rh-](Cl)(Cl)(Cl)Cl

pub use self::parser::Parser;

mod errors;
mod language;
mod lexer;
mod parser;
mod semantic;
mod syntax;

#[cfg(test)]
mod test {
    use crate::{
        parser::Parser,
        semantic::{Atom, Bond},
        syntax::{
            ast::{Branch, Root, SyntaxNodeExt, Tree},
            SyntaxNode, SyntaxToken,
        },
    };
    use petgraph::{
        graph::{node_index, NodeIndex, UnGraph},
        visit::{depth_first_search, Bfs, Control, Dfs, DfsEvent},
        Graph, Undirected,
    };
    use rowan::NodeOrToken;

    // #[test]
    // fn lexer() {
    //     let parser = Parser::new("C(OCC)(=C)=CC");
    //     for lexeme in parser.lexer {
    //         println!("{lexeme:?}");
    //     }
    // }

    #[test]
    fn parser() {
        let parser = Parser::new("[9C-3](O*C)(=C)=CC");
        let parse = parser.parse().unwrap();
        let root = parse.syntax();
        for child in root.children() {
            println!("{child:?}");
            for child in child.children_with_tokens() {
                children(1, child);
            }
        }

        fn children(level: usize, child: NodeOrToken<SyntaxNode, SyntaxToken>) {
            for _ in 0..level {
                print!("    ");
            }
            println!("{child:?}");
            if let NodeOrToken::Node(node) = child {
                for child in node.children_with_tokens() {
                    children(level + 1, child);
                }
            }
        }
    }

    #[test]
    fn test() {
        let mut graph = Graph::new_undirected();
        let parser = Parser::new("C2(O*C)(=C)=CC2");
        let parse = parser.parse().unwrap();
        let root = parse.syntax().cast::<Root>().unwrap();
        walk(&mut graph, &root.tree().unwrap());

        fn walk(graph: &mut Graph<Atom, Option<Bond>, Undirected>, tree: &Tree) -> NodeIndex {
            let node = tree.node().unwrap();
            let from = graph.add_node(node.try_into().unwrap());
            for branch in tree.branches() {
                match branch {
                    Branch::Indexed(indexed) => {
                        // TODO
                    }
                    Branch::Unindexed(unindexed) => {
                        let tree = unindexed.tree().unwrap();
                        let to = walk(graph, &tree);
                        let edge = unindexed.edge().map(Into::into);
                        graph.add_edge(from, to, edge);
                    }
                }
            }
            from
        }

        println!("{graph:?}");

        for node in graph.node_weights() {
            println!("node: {node:?}");
        }

        // let mut dfs = Dfs::new(&graph, node_index(0));
        // while let Some(index) = dfs.next(&graph) {
        //     println!("index: {index:?}, value: {:?}", graph[index]);
        // }
    }
}
