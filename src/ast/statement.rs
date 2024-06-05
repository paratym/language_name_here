use crate::{
    ast::{AliasDecl, AstNode, ExecScope, LhsExpr, ParseResult, RhsExpr},
    tokenizer::Tokenizer,
};
use std::rc::Rc;

#[derive(Debug)]
pub struct AssignStmt {
    pub lhs: LhsExpr,
    pub rhs: RhsExpr,
}

#[derive(Debug)]
pub struct IfStmt {
    pub cond: RhsExpr,
    pub body: ExecScope,
    pub chain: Option<ElseStmt>,
}

#[derive(Debug)]
pub enum ElseStmt {
    ElseIf(Rc<IfStmt>),
    Else(ExecScope),
}

#[derive(Debug)]
pub enum CtrlStmt {
    Continue,
    Break,
    Defer(RhsExpr),
    Return(RhsExpr),
}

#[derive(Debug)]
pub struct WhileStmt {
    pub cond: RhsExpr,
    pub body: ExecScope,
}

#[derive(Debug)]
pub struct MatchStmt {
    pub val: RhsExpr,
    pub branches: Vec<(LhsExpr, ExecScope)>,
}

#[derive(Debug)]
pub enum Stmt {
    AliasDecl(AliasDecl),
    Assign(AssignStmt),
    Ctrl(CtrlStmt),
    If(IfStmt),
    While(WhileStmt),
    Match(MatchStmt),
}

impl AstNode for AssignStmt {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for IfStmt {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for ElseStmt {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for CtrlStmt {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for WhileStmt {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for MatchStmt {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for Stmt {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}
