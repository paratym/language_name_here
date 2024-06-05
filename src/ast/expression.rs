use crate::{
    ast::{
        ArrayLit, ArrayType, AstNode, CharLit, CompoundLit, CompoundType, ExecScope, FnType,
        NumLit, ParseResult, RefType, StrLit, StructType,
    },
    tokenizer::Tokenizer,
};
use std::rc::Rc;

#[derive(Debug)]
pub struct Alias {
    pub val: Rc<str>,
}

#[derive(Debug)]
pub enum Arg {
    Expr(RhsExpr),
    Compound(CompoundLit),
}

#[derive(Debug)]
pub enum DotExpr {
    Field(Alias),
    Index(RhsExpr),
    Call(Arg),
}

#[derive(Debug)]
pub struct ConstructExpr {
    pub typ: RhsExpr,
    pub arg: Arg,
}

#[derive(Debug)]
pub struct SpreadExpr {
    pub val: RhsExpr,
}

#[derive(Debug)]
pub struct DestructureExpr {
    pub fields: Vec<Alias>,
}

#[derive(Debug)]
pub enum RhsExpr {
    Alias(Alias),
    Construct(Rc<ConstructExpr>),
    Dot(Rc<DotExpr>),
    Scope(ExecScope),

    NumLit(NumLit),
    CharLit(CharLit),
    StrLit(StrLit),
    ArrayLit(ArrayLit),
    CompoundLit(CompoundLit),

    RefType(Rc<RefType>),
    ArrayType(Rc<ArrayType>),
    CompoundType(CompoundType),
    StructType(StructType),
    FnType(Rc<FnType>),
}

#[derive(Debug)]
pub enum LhsExpr {
    Alias(Alias),
    Dot(DotExpr),
    Destructure(DestructureExpr),
}

impl AstNode for Alias {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for Arg {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for DotExpr {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for ConstructExpr {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for SpreadExpr {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for DestructureExpr {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for RhsExpr {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for LhsExpr {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}
