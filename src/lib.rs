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
        syntax::{SyntaxNode, SyntaxToken},
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
    fn test() {
        let parser = Parser::new("[C]--8=C-C#C");
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
}
