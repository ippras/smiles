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

mod ast;
mod errors;
mod language;
mod lexer;
mod parser;
mod syntax;

#[cfg(test)]
mod test {
    use crate::{
        parser::Parser,
        syntax::{SyntaxKind, SyntaxKind::*, SyntaxNode, SyntaxToken},
    };
    use petgraph::{
        graph::{node_index, NodeIndex, UnGraph},
        visit::{depth_first_search, Bfs, Control, Dfs, DfsEvent},
        Graph, Undirected,
    };
    use rowan::NodeOrToken;

    #[test]
    fn lexer() {
        let parser = Parser::new("[14OH-1:5555]");
        for lexeme in parser.lexer {
            println!("{lexeme:?}");
        }
    }

    #[test]
    fn parser() {
        let parser = Parser::new("C(O)(=C)=CC");
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
        let parser = Parser::new("C(OCC)(=C)=CC");
        let parse = parser.parse().unwrap();
        let root = parse.syntax();
        for node in root.children() {
            if node.kind() == NODE {
                walk_node(&mut graph, &node);
            }
        }
        fn walk_node(
            graph: &mut Graph<String, Option<usize>, Undirected>,
            node: &SyntaxNode,
        ) -> NodeIndex {
            let vertex = node
                .children()
                .find(|child| child.kind() == VERTEX)
                .unwrap();
            println!("{vertex:?}");
            let from = graph.add_node(vertex.to_string());
            if let Some(edges) = node.children().find(|child| child.kind() == EDGES) {
                println!("{edges:?}");
                if let Some(edge) = edges.children().find(|child| child.kind() == MAIN) {
                    let bond = edge.children().find(|child| child.kind() == BOND);
                    let weight = bond.map(|bond| {
                        match bond.first_child_or_token().map(|child| child.kind()) {
                            Some(MINUS) => 1,
                            Some(EQUALS) => 2,
                            Some(NUMBER) => 3,
                            Some(DOLLAR) => 4,
                            Some(COLON) => 5,
                            Some(BACKSLASH) => 6,
                            Some(SLASH) => 7,
                            _ => unreachable!(),
                        }
                    });
                    let node = edge.children().find(|child| child.kind() == NODE).unwrap();
                    let to = walk_node(graph, &node);
                    graph.add_edge(from, to, weight);
                }
                for edge in edges.children().filter(|child| child.kind() == BRANCH) {
                    let bond = edge.children().find(|child| child.kind() == BOND);
                    let weight = bond.map(|bond| {
                        match bond.first_child_or_token().map(|child| child.kind()) {
                            Some(MINUS) => 1,
                            Some(EQUALS) => 2,
                            Some(NUMBER) => 3,
                            Some(DOLLAR) => 4,
                            Some(COLON) => 5,
                            Some(BACKSLASH) => 6,
                            Some(SLASH) => 7,
                            _ => unreachable!(),
                        }
                    });
                    let node = edge.children().find(|child| child.kind() == NODE).unwrap();
                    let to = walk_node(graph, &node);
                    graph.add_edge(from, to, weight);
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
