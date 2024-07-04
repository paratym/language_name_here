use crate::{
    ast::{Alias, AliasEval, AstNode, ParseResult, RhsExpr, VisExpr},
    tokenizer::{Token, Tokenizer},
};

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
pub struct RefExpr {
    pub src: Option<Alias>,
    pub eval: Option<AliasEval>,
}

#[derive(Debug)]
pub struct DerefExpr;

#[derive(Debug)]
pub struct RefType {
    pub expr: RefExpr,
    pub typ: RhsExpr,
}

#[derive(Debug)]
pub struct StructField {
    pub vis: Option<VisExpr>,
    pub lhs: RhsExpr,
    pub typ: Option<RhsExpr>,
    pub val: Option<RhsExpr>,
}

#[derive(Debug)]
pub struct StructDef {
    pub fields: Vec<StructField>,
}

#[derive(Debug)]
pub struct ArrayType {
    pub len: RhsExpr,
}

#[derive(Debug)]
pub struct FnType {
    pub ret: RhsExpr,
}

impl AstNode for PrimitiveType {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
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

impl AstNode for RefExpr {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        if tok.peek_token()?.tok != Token::Ampersand {
            return Ok(None);
        }

        tok.expect_token(&Token::Ampersand)?;
        let src = if tok.peek_token()?.tok == Token::Caret {
            tok.expect_token(&Token::Caret)?;
            Some(Alias::expect(tok)?)
        } else {
            None
        };

        Ok(Some(Self {
            src,
            eval: AliasEval::parse(tok)?,
        }))
    }
}

impl AstNode for DerefExpr {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        Ok(if tok.peek_token()?.tok == Token::Asterisk {
            tok.expect_token(&Token::Asterisk)?;
            Some(DerefExpr)
        } else {
            None
        })
    }
}

impl AstNode for RefType {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        Ok(Some(Self {
            expr: RefExpr::expect(tok)?,
            typ: RhsExpr::expect(tok)?,
        }))
    }
}

impl AstNode for StructField {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        let vis = VisExpr::parse(tok)?;
        let lhs = RhsExpr::expect(tok)?;

        let typ = if tok.peek_token()?.tok == Token::Colon {
            tok.expect_token(&Token::Colon)?;
            RhsExpr::expect(tok)?.into()
        } else {
            None
        };

        let val = if tok.peek_token()?.tok == Token::Equal {
            tok.expect_token(&Token::Equal)?;
            RhsExpr::expect(tok)?.into()
        } else {
            None
        };

        Ok(Some(Self { vis, lhs, typ, val }))
    }
}

impl AstNode for StructDef {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        tok.expect_token(&Token::LParen)?;
        let mut fields = Vec::new();

        loop {
            if tok.peek_token()?.tok == Token::Rparen {
                break;
            }

            fields.push(StructField::expect(tok)?);

            if tok.peek_token()?.tok == Token::Rparen {
                break;
            }

            tok.expect_token(&Token::Comma)?;
        }

        tok.expect_token(&Token::Rparen)?;
        Ok(Some(Self { fields }))
    }
}

impl AstNode for ArrayType {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        if tok.peek_token()?.tok != Token::LSqrBrace {
            return Ok(None);
        }

        tok.expect_token(&Token::LSqrBrace);
        let len = RhsExpr::expect(tok)?;
        tok.expect_token(&Token::RSqrBrace)?;

        Ok(Some(Self { len }))
    }
}

impl AstNode for FnType {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        if tok.peek_token()?.tok != Token::Arrow {
            return Ok(None);
        }

        tok.expect_token(&Token::Arrow)?;
        let ret = RhsExpr::expect(tok)?;
        Ok(Some(Self { ret }))
    }
}
