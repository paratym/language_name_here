use std::{fs::File, io::BufReader};
pub mod lex;

fn main() {
    let file = BufReader::new(File::open("aspirational_spec.idk").unwrap());
    let mut lexer = lex::Lexer::new(Box::new(file));
    while let Some(tok) = lexer.next_token().unwrap() {
        println!("{:?}", tok);
    }
}
