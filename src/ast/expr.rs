use crate::{
    ast::{
        first_match, first_match_chain, Alias, ArrayLit, ArrayType, AstNode, BoolLit, CharLit,
        DerefExpr, ExecScope, FnType, NumLit, ParseResult, PrimitiveType, RefExpr, RefType,
        ScopeAlias, StrLit, StructDef, UnionType,
    },
    tok::{Token, Tokenizer},
};
use std::{io::BufRead, rc::Rc};

#[derive(Debug)]
pub enum EvalPath {
    Scope(Expr),
    Construct(CallExpr),
}

#[derive(Debug)]
pub enum ExecPath {
    Ref(RefExpr),
    Deref(DerefExpr),
    Name(Alias),
    Dynamic(ExecScope),
}

#[derive(Debug)]
pub enum CallExpr {
    Arg(ExecScope),
    Array(ArrayLit),
    Struct(StructDef),
}

#[derive(Debug)]
pub enum Expr {
    Scope(ExecScope),
    ScopeAlias(ScopeAlias),
    Alias(Alias),
    EvalPath { rcv: Rc<Expr>, path: Rc<EvalPath> },
    ExecPath { rcv: Rc<Expr>, path: ExecPath },
    Call { rcv: Rc<Expr>, arg: CallExpr },

    BoolLit(BoolLit),
    NumLit(NumLit),
    CharLit(CharLit),
    StrLit(StrLit),
    ArrayLit(ArrayLit),
    Struct(StructDef),

    PrimitiveType(PrimitiveType),
    RefType(Rc<RefType>),
    UnionType(Rc<UnionType>),
    ArrayType { typ: Rc<Expr>, len: Rc<ArrayType> },
    FnType { arg: Rc<Expr>, ret: Rc<FnType> },
}

impl AstNode for EvalPath {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if !tok.next_is(&Token::DoubleColon)? {
            return Ok(None);
        }

        tok.expect(&Token::DoubleColon)?;
        Ok(first_match!(tok, Self, CallExpr, Expr))
    }
}

impl From<Expr> for EvalPath {
    fn from(value: Expr) -> Self {
        Self::Scope(value)
    }
}

impl From<CallExpr> for EvalPath {
    fn from(value: CallExpr) -> Self {
        Self::Construct(value)
    }
}

impl AstNode for ExecPath {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if !tok.next_is(&Token::Dot)? {
            return Ok(None);
        }

        tok.expect(&Token::Dot)?;
        Ok(first_match!(
            tok, Self, RefExpr, DerefExpr, Alias, ExecScope
        ))
    }
}

impl From<RefExpr> for ExecPath {
    fn from(value: RefExpr) -> Self {
        Self::Ref(value)
    }
}

impl From<DerefExpr> for ExecPath {
    fn from(value: DerefExpr) -> Self {
        Self::Deref(value)
    }
}

impl From<Alias> for ExecPath {
    fn from(value: Alias) -> Self {
        Self::Name(value)
    }
}

impl From<ExecScope> for ExecPath {
    fn from(value: ExecScope) -> Self {
        Self::Dynamic(value)
    }
}

impl AstNode for CallExpr {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        Ok(first_match!(tok, Self, ExecScope, ArrayLit, StructDef))
    }
}

impl From<ExecScope> for CallExpr {
    fn from(value: ExecScope) -> Self {
        Self::Arg(value)
    }
}

impl From<ArrayLit> for CallExpr {
    fn from(value: ArrayLit) -> Self {
        Self::Array(value)
    }
}

impl From<StructDef> for CallExpr {
    fn from(value: StructDef) -> Self {
        Self::Struct(value)
    }
}

impl AstNode for Expr {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        let rcv = first_match!(
            tok,
            Self,
            ExecScope,
            ScopeAlias,
            Alias,
            BoolLit,
            NumLit,
            CharLit,
            StrLit,
            ArrayLit,
            PrimitiveType,
            RefType,
            UnionType,
            StructDef
        );

        let mut expr = match rcv {
            Some(expr) => expr,
            None => return Ok(None),
        };

        loop {
            let chain = first_match_chain!(
                tok, Self, expr, EvalPath, ExecPath, ArrayType, CallExpr, FnType
            );

            match chain {
                (true, chain) => expr = chain,
                (false, expr) => break Ok(Some(expr)),
            }
        }
    }
}

impl From<ExecScope> for Expr {
    fn from(value: ExecScope) -> Self {
        Self::Scope(value)
    }
}

impl From<ScopeAlias> for Expr {
    fn from(value: ScopeAlias) -> Self {
        Self::ScopeAlias(value)
    }
}

impl From<Alias> for Expr {
    fn from(value: Alias) -> Self {
        Self::Alias(value)
    }
}

impl From<(Self, EvalPath)> for Expr {
    fn from(value: (Self, EvalPath)) -> Self {
        Self::EvalPath {
            rcv: value.0.into(),
            path: value.1.into(),
        }
    }
}

impl From<(Self, ExecPath)> for Expr {
    fn from(value: (Self, ExecPath)) -> Self {
        Self::ExecPath {
            rcv: value.0.into(),
            path: value.1,
        }
    }
}
impl From<(Self, CallExpr)> for Expr {
    fn from(value: (Self, CallExpr)) -> Self {
        Self::Call {
            rcv: value.0.into(),
            arg: value.1,
        }
    }
}

impl From<BoolLit> for Expr {
    fn from(value: BoolLit) -> Self {
        Self::BoolLit(value)
    }
}

impl From<NumLit> for Expr {
    fn from(value: NumLit) -> Self {
        Self::NumLit(value)
    }
}

impl From<CharLit> for Expr {
    fn from(value: CharLit) -> Self {
        Self::CharLit(value)
    }
}

impl From<StrLit> for Expr {
    fn from(value: StrLit) -> Self {
        Self::StrLit(value)
    }
}

impl From<ArrayLit> for Expr {
    fn from(value: ArrayLit) -> Self {
        Self::ArrayLit(value)
    }
}

impl From<PrimitiveType> for Expr {
    fn from(value: PrimitiveType) -> Self {
        Self::PrimitiveType(value)
    }
}

impl From<RefType> for Expr {
    fn from(value: RefType) -> Self {
        Self::RefType(value.into())
    }
}

impl From<UnionType> for Expr {
    fn from(value: UnionType) -> Self {
        Self::UnionType(value.into())
    }
}

impl From<StructDef> for Expr {
    fn from(value: StructDef) -> Self {
        Self::Struct(value)
    }
}

impl From<(Self, ArrayType)> for Expr {
    fn from(value: (Self, ArrayType)) -> Self {
        Self::ArrayType {
            typ: value.0.into(),
            len: value.1.into(),
        }
    }
}

impl From<(Self, FnType)> for Expr {
    fn from(value: (Self, FnType)) -> Self {
        Self::FnType {
            arg: value.0.into(),
            ret: value.1.into(),
        }
    }
}
