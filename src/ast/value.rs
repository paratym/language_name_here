use crate::{
    ast::{
        first_match, Alias, AliasEval, AstNode, Expr, Ident, IdentExecPath, ParseErr, ParseResult,
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
    Vis(Vis),
    Key(Expr),
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
        if !tok.next_is(&Token::Ampersand)? {
            return Ok(None);
        }

        tok.expect(&Token::Ampersand)?;
        let src = if tok.next_is(&Token::Caret)? {
            tok.expect(&Token::Caret)?;
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
        if !tok.next_is(&Token::Asterisk)? {
            return Ok(None);
        }

        tok.expect(&Token::Asterisk)?;
        Ok(Some(Self))
    }
}

impl AstNode for SpreadExpr {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if !tok.next_is(&Token::Elipsis)? {
            return Ok(None);
        }

        tok.expect(&Token::Elipsis)?;
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
        if !tok.next_is(&Token::LSqrBrace)? {
            return Ok(None);
        }

        tok.expect(&Token::LSqrBrace)?;
        let mut fields = Vec::new();
        let mut separated = false;

        loop {
            match tok.peek()? {
                Some(Token::RSqrBrace) => {
                    tok.expect(&Token::RSqrBrace)?;
                    break;
                }
                Some(Token::Comma) if !separated => {
                    tok.expect(&Token::Comma)?;
                    separated = true;
                }
                _ if !fields.is_empty() && !separated => {
                    return Err(ParseErr::Syntax {
                        pos: *tok.pos(),
                        msg: "expected ',' or ']'".into(),
                    })
                }
                _ => {
                    fields.push(ArrayField::expect(tok)?);
                    separated = false;
                }
            }
        }

        Ok(Some(Self { fields }))
    }
}

impl AstNode for StructField {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if let Some(field) = first_match!(tok, Self, IdentExecPath, SpreadExpr) {
            return Ok(field.into());
        }

        if tok.next_is(&Token::Key)? {
            tok.expect(&Token::Key)?;
            let expr = Expr::expect(tok)?;
            return Ok(Some(Self::Key(expr)));
        }

        let vis = Vis::parse(tok)?;
        if tok.next_is(&Token::Asterisk)? {
            if let Some(vis) = vis {
                tok.expect(&Token::Asterisk)?;
                return Ok(Self::Vis(vis).into());
            }
        }

        let ident = Ident::expect(tok)?;
        let typ = if tok.next_is(&Token::Colon)? {
            tok.expect(&Token::Colon)?;
            Expr::expect(tok)?.into()
        } else {
            None
        };

        let val = if tok.next_is(&Token::Equal)? {
            tok.expect(&Token::Equal)?;
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
        if !tok.next_is(&Token::LParen)? {
            return Ok(None);
        }

        tok.expect(&Token::LParen)?;
        let mut fields = Vec::new();
        let mut separated = false;

        loop {
            match tok.peek()? {
                Some(Token::Rparen) => {
                    tok.expect(&Token::Rparen)?;
                    break;
                }
                Some(Token::Comma) if !separated => {
                    tok.expect(&Token::Comma)?;
                    separated = true;
                    continue;
                }
                _ if !separated && !fields.is_empty() => {
                    return Err(ParseErr::Syntax {
                        pos: *tok.pos(),
                        msg: "exected ',' or ')'".into(),
                    });
                }
                _ => {
                    fields.push(StructField::expect(tok)?);
                    separated = false;
                }
            }
        }

        Ok(Some(Self { fields }))
    }
}
