use crate::{
    ast::{
        first_match, Alias, AstNode, CallExpr, ExecScope, Expr, GlobalScope, Ident, ParseErr,
        ParseResult,
    },
    tok::{Token, Tokenizer},
};
use std::{io::BufRead, rc::Rc};

#[derive(Debug)]
pub enum Access {
    Get,
    Set,
}

#[derive(Debug)]
pub struct Vis {
    pub scope: Option<GlobalScope>,
    pub access: Option<Access>,
}

#[derive(Debug)]
pub struct VisDecl {
    pub vis: Vis,
    pub decl: Rc<Decl>,
}

#[derive(Debug)]
pub enum AliasEval {
    Let,
    Var,
    Const,
    Type,
}

#[derive(Debug)]
pub struct AliasDecl {
    pub eval: AliasEval,
    pub ident: Ident,
    pub bounds: Option<Expr>,
    pub rhs: Option<Expr>,
}

#[derive(Debug)]
pub enum ScopeEval {
    Fn,
    Iface,
    Mod,
}

#[derive(Debug)]
pub struct ScopeAliasDecl {
    pub eval: ScopeEval,
    pub ident: Ident,
    pub bounds: Option<Rc<Expr>>,
    pub scope: Option<ExecScope>,
}

#[derive(Debug)]
pub struct UseDecl {}

#[derive(Debug)]
pub enum Annotation {
    Expr { tag: Alias, expr: Expr },
    Decl { tag: Alias, decl: Decl },
}

#[derive(Debug)]
pub enum Decl {
    Vis(VisDecl),
    Alias(AliasDecl),
    Scope(ScopeAliasDecl),
    Use(UseDecl),
    Annotation(Rc<Annotation>),
}

impl AstNode for Access {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        let access = match tok.peek()? {
            Some(Token::Get) => Self::Get,
            Some(Token::Set) => Self::Set,
            _ => return Ok(None),
        };

        tok.next_tok()?;
        Ok(Some(access))
    }
}

impl AstNode for Vis {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if !tok.next_is(&Token::Pub)? {
            return Ok(None);
        }

        tok.expect(&Token::Pub)?;
        let mut scope = None;
        let mut access = None;

        loop {
            if !tok.next_is(&Token::Colon)? {
                return Ok(Some(Self { scope, access }));
            }

            tok.expect(&Token::Colon)?;

            match tok.peek()? {
                Some(Token::Pkg | Token::Mod) if scope.is_none() => {
                    scope = GlobalScope::expect(tok)?.into()
                }
                Some(Token::Get | Token::Set) if access.is_none() => {
                    access = Access::expect(tok)?.into()
                }
                _ => {
                    return Err(ParseErr::Syntax {
                        pos: *tok.pos(),
                        msg: "expected unique scope or access alias".into(),
                    })
                }
            };
        }
    }
}

impl AstNode for VisDecl {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        let vis = if let Some(vis) = Vis::parse(tok)? {
            vis
        } else {
            return Ok(None);
        };

        let decl = Decl::expect(tok)?.into();
        Ok(Some(Self { vis, decl }))
    }
}

impl AstNode for AliasEval {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        let eval = match tok.peek()? {
            Some(Token::Let) => Self::Let,
            Some(Token::Var) => Self::Var,
            Some(Token::Const) => Self::Const,
            Some(Token::Type) => Self::Type,
            _ => return Ok(None),
        };

        tok.next_tok()?;
        Ok(Some(eval))
    }
}

impl AstNode for AliasDecl {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        let eval = match AliasEval::parse(tok)? {
            Some(eval) => eval,
            None => return Ok(None),
        };

        let ident = Ident::expect(tok)?;
        let bounds = if tok.next_is(&Token::Colon)? {
            tok.expect(&Token::Colon)?;
            Some(Expr::expect(tok)?)
        } else {
            None
        };

        let rhs = if tok.next_is(&Token::Equal)? {
            tok.expect(&Token::Equal)?;
            Some(Expr::expect(tok)?)
        } else {
            None
        };

        tok.expect(&Token::Semicolon)?;
        Ok(Some(Self {
            eval,
            ident,
            bounds,
            rhs,
        }))
    }
}

impl AstNode for ScopeEval {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        let eval = match tok.peek()? {
            Some(Token::Fn) => Self::Fn,
            Some(Token::Iface) => Self::Iface,
            Some(Token::Mod) => Self::Mod,
            _ => return Ok(None),
        };

        tok.next_tok()?;
        Ok(Some(eval))
    }
}

impl AstNode for ScopeAliasDecl {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        let eval = match ScopeEval::parse(tok)? {
            Some(eval) => eval,
            None => return Ok(None),
        };

        let ident = Ident::expect(tok)?;
        let bound = tok.next_is(&Token::Colon)?;
        if bound {
            tok.expect(&Token::Colon)?;
        }

        let (bounds, scope) = match Expr::parse(tok)? {
            Some(Expr::Call {
                rcv,
                arg: CallExpr::Scope(scope),
            }) if bound => (rcv.into(), scope.into()),
            Some(expr) if bound => (Some(expr.into()), None),
            Some(Expr::Scope(scope)) if !bound => (None, scope.into()),
            None if !bound => (None, None),
            _ => {
                return Err(ParseErr::Syntax {
                    pos: *tok.pos(),
                    msg: if bound {
                        "expected expression".into()
                    } else {
                        "expected scope".into()
                    },
                })
            }
        };

        if tok.next_is(&Token::Semicolon)? {
            tok.expect(&Token::Semicolon)?;
        }

        Ok(Some(Self {
            eval,
            ident,
            bounds,
            scope,
        }))
    }
}

impl AstNode for Annotation {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if !tok.next_is(&Token::Bang)? {
            return Ok(None);
        }

        tok.expect(&Token::Bang)?;
        tok.expect(&Token::LSqrBrace)?;
        let tag = Alias::expect(tok)?;

        if tok.next_is(&Token::RSqrBrace)? {
            tok.expect(&Token::RSqrBrace)?;
            let decl = Decl::expect(tok)?;
            return Ok(Some(Self::Decl { tag, decl }));
        }

        tok.expect(&Token::Equal)?;
        let expr = Expr::expect(tok)?;
        tok.expect(&Token::RSqrBrace)?;

        Ok(Some(Self::Expr { tag, expr }))
    }
}
impl AstNode for Decl {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        Ok(first_match!(
            tok,
            Self,
            VisDecl,
            AliasDecl,
            ScopeAliasDecl,
            Annotation
        ))
    }
}

impl From<VisDecl> for Decl {
    fn from(value: VisDecl) -> Self {
        Self::Vis(value)
    }
}

impl From<AliasDecl> for Decl {
    fn from(value: AliasDecl) -> Self {
        Self::Alias(value)
    }
}

impl From<ScopeAliasDecl> for Decl {
    fn from(value: ScopeAliasDecl) -> Self {
        Self::Scope(value)
    }
}

impl From<Annotation> for Decl {
    fn from(value: Annotation) -> Self {
        Self::Annotation(value.into())
    }
}
