use crate::{
    ast::{AstNode, LhsExpr, ParseResult, RhsExpr},
    tokenizer::Tokenizer,
};
use std::rc::Rc;

#[derive(Debug)]
pub struct NumLit {
    pub negative: bool,
    pub whole: u64,
    pub frac: u64,
}

#[derive(Debug)]
pub struct CharLit {
    pub val: char,
}

#[derive(Debug)]
pub struct StrLit {
    pub val: Rc<str>,
}

#[derive(Debug)]
pub struct ArrayLit {
    pub entries: Vec<RhsExpr>,
}

#[derive(Debug)]
pub struct CompoundLit {
    pub fields: Vec<(LhsExpr, RhsExpr)>,
}

impl AstNode for NumLit {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for CharLit {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for StrLit {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for ArrayLit {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for CompoundLit {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}
