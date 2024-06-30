use crate::{
    ast::{Alias, AstNode, ParseErr, ParseResult, RhsExpr, VisExpr},
    tokenizer::{Token, Tokenizer},
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
pub struct FieldDef {
    pub vis: Option<VisExpr>,
    pub alias: Alias,
    pub typ: RhsExpr,
    pub default: Option<RhsExpr>,
}

#[derive(Debug)]
pub struct CompoundType {
    pub fields: Vec<FieldDef>,
}

#[derive(Debug)]
pub struct FnType {
    pub arg: RhsExpr,
    pub ret: RhsExpr,
}

impl AstNode for RefType {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        tok.expect_token(&Token::Ampersand)?;
        let mutable = tok.peek_token()?.tok == Token::Mut;
        if mutable {
            tok.expect_token(&Token::Mut)?;
        }

        Ok(Self {
            mutable,
            typ: RhsExpr::parse(tok)?,
        })
    }
}

impl AstNode for ArrayType {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for FieldDef {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        let vis = if tok.peek_token()?.tok == Token::Pub {
            Some(VisExpr::parse(tok)?)
        } else {
            None
        };

        let alias = Alias::parse(tok)?;
        tok.expect_token(&Token::Colon)?;
        let typ = RhsExpr::parse(tok)?;

        let default = if tok.peek_token()?.tok == Token::Equal {
            tok.expect_token(&Token::Equal)?;
            Some(RhsExpr::parse(tok)?)
        } else {
            None
        };

        Ok(Self {
            vis,
            alias,
            typ,
            default,
        })
    }
}

impl AstNode for CompoundType {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        tok.expect_token(&Token::LParen)?;
        let mut typ = Self { fields: Vec::new() };

        loop {
            if tok.peek_token()?.tok == Token::Rparen {
                return Ok(typ);
            }

            typ.fields.push(FieldDef::parse(tok)?);

            match tok.peek_token()?.tok {
                Token::Comma => tok.expect_token(&Token::Comma)?,
                Token::Rparen => break,
                _ => {
                    let token = tok.next_token()?;
                    return Err(ParseErr::Syntax {
                        pos: token.pos,
                        msg: "expected ',' or ')'",
                    });
                }
            };
        }

        tok.expect_token(&Token::Rparen)?;
        Ok(typ)
    }
}

impl AstNode for FnType {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}
