use crate::{
    ast::{Alias, AliasEval, AstNode, Expr, ParseResult, RefExpr},
    tok::{Token, Tokenizer},
};
use std::io::BufRead;

#[derive(Debug)]
pub enum PrimitiveType {
    Bool,
    Char,
    Str,
    Isize,
    I8,
    I16,
    I24,
    I32,
    I64,
    Usize,
    U8,
    U16,
    U24,
    U32,
    U64,
    F32,
    F64,
}

#[derive(Debug)]
pub struct RefType {
    pub expr: RefExpr,
    pub typ: Expr,
}

#[derive(Debug)]
pub struct ArrayType {
    pub len: Expr,
}

#[derive(Debug)]
pub struct FnType {
    pub ret: Expr,
}

impl AstNode for PrimitiveType {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        Ok(Some(match tok.peek_token()?.tok {
            Token::Bool => Self::Bool,
            Token::Char => Self::Char,
            Token::Str => Self::Str,
            Token::Isize => Self::Isize,
            Token::I8 => Self::I8,
            Token::I16 => Self::I16,
            Token::I24 => Self::I24,
            Token::I32 => Self::I32,
            Token::I64 => Self::I64,
            Token::Usize => Self::Usize,
            Token::U8 => Self::U8,
            Token::U16 => Self::U16,
            Token::U24 => Self::U24,
            Token::U32 => Self::U32,
            Token::U64 => Self::U64,
            Token::F32 => Self::F32,
            Token::F64 => Self::F64,
            _ => return Ok(None),
        }))
    }
}

impl AstNode for RefType {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        Ok(if let Some(expr) = RefExpr::parse(tok)? {
            Some(Self {
                expr,
                typ: Expr::expect(tok)?,
            })
        } else {
            None
        })
    }
}

impl AstNode for ArrayType {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if tok.peek_token()?.tok != Token::LSqrBrace {
            return Ok(None);
        }

        tok.expect_token(&Token::LSqrBrace)?;
        let len = Expr::expect(tok)?;
        tok.expect_token(&Token::RSqrBrace)?;

        Ok(Some(Self { len }))
    }
}

impl AstNode for FnType {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if tok.peek_token()?.tok != Token::Arrow {
            return Ok(None);
        }

        tok.expect_token(&Token::Arrow)?;
        let ret = Expr::expect(tok)?;
        Ok(Some(Self { ret }))
    }
}
