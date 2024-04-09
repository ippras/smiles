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
        semantic::{Atom, Bond, MoleculeGraph},
        syntax::{
            ast::{Branch, Root, SyntaxNodeExt, Tree},
            SyntaxNode, SyntaxToken,
        },
    };
    use petgraph::{
        graph::{node_index, NodeIndex, UnGraph},
        visit::{
            depth_first_search, Bfs, Control, Dfs, DfsEvent, IntoEdgeReferences,
            IntoNodeIdentifiers,
        },
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
        // let parser = Parser::new("CCCCCCCCCCCCCC(=O)O");
        let parser = Parser::new("C(OCCCC)C=CCCCCCCCC(=O)O");
        let parse = parser.parse().unwrap();
        let root = parse.syntax().cast::<Root>().unwrap();
        let graph = MoleculeGraph::try_from(root).unwrap();

        println!("{graph:?}");

        for index in graph.carbons().node_identifiers() {
            println!("{:2}: {:?}", index.index(), graph[index].element);
        }

        let the_longest_carbon_chain = graph.the_longest_carbon_chain();
        println!("{the_longest_carbon_chain:?}");

        for edge in graph.bonds(|bond| bond == Bond::Double).edge_references() {
            println!("{edge:?}");
        }

        // for atom in graph.node_weights() {
        //     println!("node: {:?}", atom.element);
        // }

        // let mut dfs = Dfs::new(&graph, node_index(0));
        // while let Some(index) = dfs.next(&graph) {
        //     println!("index: {index:?}, value: {:?}", graph[index]);
        // }
    }
}
