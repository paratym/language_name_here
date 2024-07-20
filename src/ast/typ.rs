use crate::{
    ast::{AstNode, Expr, ParseResult, RefExpr},
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
pub struct UnionType {
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
        let typ = match tok.peek()? {
            Some(Token::Bool) => Self::Bool,
            Some(Token::Char) => Self::Char,
            Some(Token::Str) => Self::Str,
            Some(Token::Isize) => Self::Isize,
            Some(Token::I8) => Self::I8,
            Some(Token::I16) => Self::I16,
            Some(Token::I24) => Self::I24,
            Some(Token::I32) => Self::I32,
            Some(Token::I64) => Self::I64,
            Some(Token::Usize) => Self::Usize,
            Some(Token::U8) => Self::U8,
            Some(Token::U16) => Self::U16,
            Some(Token::U24) => Self::U24,
            Some(Token::U32) => Self::U32,
            Some(Token::U64) => Self::U64,
            Some(Token::F32) => Self::F32,
            Some(Token::F64) => Self::F64,
            _ => return Ok(None),
        };

        tok.next_tok()?;
        Ok(Some(typ))
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

impl AstNode for UnionType {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if !tok.next_is(&Token::Union)? {
            return Ok(None);
        }

        tok.expect(&Token::Union)?;
        let typ = Expr::expect(tok)?;

        Ok(Some(Self { typ }))
    }
}

impl AstNode for ArrayType {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if !tok.next_is(&Token::LSqrBrace)? {
            return Ok(None);
        }

        tok.expect(&Token::LSqrBrace)?;
        let len = Expr::expect(tok)?;
        tok.expect(&Token::RSqrBrace)?;

        Ok(Some(Self { len }))
    }
}

impl AstNode for FnType {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if !tok.next_is(&Token::Arrow)? {
            return Ok(None);
        }

        tok.expect(&Token::Arrow)?;
        let ret = Expr::expect(tok)?;
        Ok(Some(Self { ret }))
    }
}
