use crate::{
    ast::{Alias, AstNode, ParseResult, RhsExpr},
    tokenizer::Tokenizer,
};

#[derive(Debug)]
pub struct RefType {
    pub mutable: bool,
    pub typ: RhsExpr,
}

#[derive(Debug)]
pub struct ArrayType {
    pub typ: RhsExpr,
    pub len: RhsExpr,
}

#[derive(Debug)]
pub struct CompoundType {
    pub fields: Vec<(Alias, RhsExpr)>,
}

#[derive(Debug)]
pub struct StructFieldDef {
    pub alias: Alias,
    pub typ: RhsExpr,
    pub default: Option<RhsExpr>,
}

#[derive(Debug)]
pub struct StructType {
    pub fields: Vec<StructFieldDef>,
}

#[derive(Debug)]
pub struct FnType {
    pub arg: RhsExpr,
    pub ret: RhsExpr,
}

impl AstNode for RefType {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for ArrayType {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for CompoundType {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for StructType {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for FnType {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}
