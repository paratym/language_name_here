use crate::{
    ast::{
        first_match, Alias, AliasEval, AstNode, ExecPath, Expr, Ident, IdentExecPath, ParseResult,
        Vis,
    },
    tok::{Token, Tokenizer},
};
use std::io::BufRead;

#[derive(Debug)]
pub struct RefExpr {
    pub src: Option<Alias>,
    pub eval: Option<AliasEval>,
}

#[derive(Debug)]
pub struct DerefExpr;

#[derive(Debug)]
pub struct SpreadExpr {
    pub val: Expr,
}

#[derive(Debug)]
pub enum ArrayField {
    Spread(SpreadExpr),
    Val(Expr),
}

#[derive(Debug)]
pub struct ArrayLit {
    pub fields: Vec<ArrayField>,
}

#[derive(Debug)]
pub enum StructField {
    GlobalVis(Vis),
    Inherit(IdentExecPath),
    Spread(SpreadExpr),
    Def {
        vis: Option<Vis>,
        ident: Ident,
        typ: Option<Expr>,
        val: Option<Expr>,
    },
}

#[derive(Debug)]
pub struct StructDef {
    pub fields: Vec<StructField>,
}

impl AstNode for RefExpr {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
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
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        Ok(if tok.peek_token()?.tok == Token::Asterisk {
            tok.expect_token(&Token::Asterisk)?;
            Some(DerefExpr)
        } else {
            None
        })
    }
}

impl AstNode for SpreadExpr {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if tok.peek_token()?.tok != Token::Elipsis {
            return Ok(None);
        }

        tok.expect_token(&Token::Elipsis)?;
        let val = Expr::expect(tok)?;
        Ok(Some(Self { val }))
    }
}

impl AstNode for ArrayField {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        Ok(first_match!(tok, Self, SpreadExpr, Expr))
    }
}

impl From<SpreadExpr> for ArrayField {
    fn from(value: SpreadExpr) -> Self {
        Self::Spread(value)
    }
}

impl From<Expr> for ArrayField {
    fn from(value: Expr) -> Self {
        Self::Val(value)
    }
}

impl AstNode for ArrayLit {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if tok.peek_token()?.tok != Token::LSqrBrace {
            return Ok(None);
        }

        tok.expect_token(&Token::LSqrBrace)?;

        let mut fields = Vec::new();
        loop {
            if tok.peek_token()?.tok == Token::RSqrBrace {
                break;
            }

            if !fields.is_empty() {
                tok.expect_token(&Token::Comma)?;
            }

            fields.push(ArrayField::expect(tok)?);
        }

        tok.expect_token(&Token::RSqrBrace)?;
        Ok(Some(Self { fields }))
    }
}

impl AstNode for StructField {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if let Some(field) = first_match!(tok, Self, IdentExecPath, SpreadExpr) {
            return Ok(field.into());
        }

        let vis = Vis::parse(tok)?;
        if tok.peek_token()?.tok == Token::Asterisk {
            if let Some(vis) = vis {
                tok.expect_token(&Token::Asterisk)?;
                return Ok(Self::GlobalVis(vis).into());
            }
        }

        let ident = Ident::expect(tok)?;
        let typ = if tok.peek_token()?.tok == Token::Colon {
            tok.expect_token(&Token::Colon)?;
            Expr::expect(tok)?.into()
        } else {
            None
        };

        let val = if tok.peek_token()?.tok == Token::Equal {
            tok.expect_token(&Token::Equal)?;
            Expr::expect(tok)?.into()
        } else {
            None
        };

        Ok(Some(Self::Def {
            vis,
            ident,
            typ,
            val,
        }))
    }
}

impl From<IdentExecPath> for StructField {
    fn from(value: IdentExecPath) -> Self {
        Self::Inherit(value)
    }
}

impl From<SpreadExpr> for StructField {
    fn from(value: SpreadExpr) -> Self {
        Self::Spread(value)
    }
}

impl AstNode for StructDef {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if tok.peek_token()?.tok == Token::LParen {
            tok.expect_token(&Token::LParen)?;
        } else {
            return Ok(None);
        }

        let mut fields = Vec::new();
        loop {
            if tok.peek_token()?.tok == Token::Rparen {
                break;
            }

            if !fields.is_empty() {
                tok.expect_token(&Token::Comma)?;
            }

            fields.push(StructField::expect(tok)?)
        }

        tok.expect_token(&Token::Rparen)?;
        Ok(Some(Self { fields }))
    }
}
