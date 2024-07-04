use crate::{
    ast::{
        first_match, ArrayLit, AstNode, DerefExpr, ExecScope, ParseResult, RefExpr, RhsExpr,
        StructDef,
    },
    tokenizer::{Token, Tokenizer},
};
use std::sync::Arc;

#[derive(Debug)]
pub struct Alias {
    pub alias: Arc<str>,
}

#[derive(Debug)]
pub enum EvalPath {
    Scope(RhsExpr),
    Call(CallExpr),
}

#[derive(Debug)]
pub enum ExecPath {
    Ref(RefExpr),
    Deref(DerefExpr),
    Static(Alias),
    Dynamic(ExecScope),
}

#[derive(Debug)]
pub enum CallExpr {
    Arg(ExecScope),
    Array(ArrayLit),
    Struct(StructDef),
}

#[derive(Debug)]
pub struct DestructureExpr {}

impl AstNode for Alias {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        Ok(if let Token::Alias(alias_ref) = &tok.peek_token()?.tok {
            let alias = alias_ref.clone();
            tok.next_token()?;
            Some(Self { alias })
        } else {
            None
        })
    }
}

impl AstNode for EvalPath {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        if tok.peek_token()?.tok != Token::DoubleColon {
            return Ok(None);
        }

        tok.expect_token(&Token::DoubleColon)?;
        Ok(first_match!(tok, Self, CallExpr, RhsExpr))
    }
}

impl From<RhsExpr> for EvalPath {
    fn from(value: RhsExpr) -> Self {
        Self::Scope(value)
    }
}

impl From<CallExpr> for EvalPath {
    fn from(value: CallExpr) -> Self {
        Self::Call(value)
    }
}

impl AstNode for ExecPath {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        if tok.peek_token()?.tok != Token::Dot {
            return Ok(None);
        }

        tok.expect_token(&Token::Dot)?;
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
        Self::Static(value)
    }
}

impl From<ExecScope> for ExecPath {
    fn from(value: ExecScope) -> Self {
        Self::Dynamic(value)
    }
}

impl AstNode for CallExpr {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
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

impl AstNode for DestructureExpr {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        if tok.peek_token()?.tok != Token::Dot {
            return Ok(None);
        }

        tok.expect_token(&Token::Dot)?;
        // loop {
        //     if tok.peek_token()?.tok == Token::Rparen {
        //         break;
        //     }
        //
        //     expr.fields.push(LhsExpr::parse(tok)?);
        //
        //     let token = tok.peek_token()?;
        //     match token.tok {
        //         Token::Rparen => break,
        //         Token::Comma => tok.expect_token(&Token::Comma)?,
        //         _ => {
        //             return Err(ParseErr::Syntax {
        //                 pos: token.pos,
        //                 msg: "expected ',' or ')'",
        //             })
        //         }
        //     };
        // }
        //
        // tok.expect_token(&Token::Rparen)?;
        // Ok(expr)
        todo!()
    }
}
