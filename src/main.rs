use ast::AstNode;
use std::{fs::File, io::BufReader};

pub mod ast;
pub mod tokenizer;
pub mod util;

fn main() {
    let file = BufReader::new(File::open("./tour/package.idk").unwrap());
    let mut tok = tokenizer::Tokenizer::new(Box::new(file));
    let pkg = ast::EvalScope::parse(&mut tok);
    println!("{:?}", pkg);
}
