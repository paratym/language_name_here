use crate::{ast::ParseResult, tokenizer::Tokenizer};

pub trait AstNode: Sized {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self>;
}
