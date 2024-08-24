pub mod ast;
pub mod parser;

use ast::{AstNode, Decl};
use parser::{Parser, Rule};
use pest::Parser as _;

fn main() {
    let mut pairs = Parser::parse(Rule::src, include_str!("../tour/01-variables.idk")).unwrap();
    while let Some(decl) = Decl::parse(&mut pairs).unwrap() {
        println!("{decl:?}")
    }
}
