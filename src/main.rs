use std::{fs::File, io::BufReader};

pub mod ast;
pub mod parser;
pub mod tokenizer;

fn main() {
    let file = BufReader::new(File::open("aspirational_spec.idk").unwrap());
    let mut tokenizer = tokenizer::Tokenizer::new(Box::new(file));
    while let Some(tok) = tokenizer.next_token().unwrap() {
        println!("{:?}", tok);
    }
}
