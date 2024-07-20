use crate::{
    ast::{
        first_match, Alias, AstNode, EvalScope, ExecScope, Expr, Ident, ParseErr, ParseResult,
        ScopeAlias, Stmt,
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
    pub scope: Option<ScopeAlias>,
    pub access: Option<Access>,
}

#[derive(Debug)]
pub struct VisDecl {
    pub vis: Vis,
    pub stmt: Rc<Stmt>,
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
pub struct FnSigDecl {
    pub rcv: Option<Expr>,
    pub arg: Expr,
    pub ret: Expr,
}

#[derive(Debug)]
pub struct FnDecl {
    pub ident: Ident,
    pub sig: Option<FnSigDecl>,
    pub body: Option<ExecScope>,
}

#[derive(Debug)]
pub struct EnumVarDecl {
    pub ident: Ident,
    pub key: Option<Expr>,
    pub typ: Option<Expr>,
    pub val: Option<Expr>,
}

// #[derive(Debug)]
// pub struct EnumDecl {
//     pub ident: Ident,
//     pub typ: Option<Expr>,
//     pub vars: Vec<EnumVarDecl>,
// }

#[derive(Debug)]
pub struct IfaceDecl {
    pub ident: Ident,
    pub typ: Option<Expr>,
    pub scope: EvalScope,
}

#[derive(Debug)]
pub struct ModDecl {
    pub ident: Ident,
    pub typ: Option<Expr>,
    pub scope: Option<EvalScope>,
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
    Fn(FnDecl),
    Iface(IfaceDecl),
    Mod(ModDecl),
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
                    scope = ScopeAlias::expect(tok)?.into()
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

        let stmt = Stmt::expect(tok)?.into();
        Ok(Some(Self { vis, stmt }))
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
        let eval = if let Some(eval) = AliasEval::parse(tok)? {
            eval
        } else {
            return Ok(None);
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

impl AstNode for FnSigDecl {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        let one = if let Some(slot) = Expr::parse(tok)? {
            slot
        } else {
            return Ok(None);
        };

        tok.expect(&Token::Arrow)?;
        let two = Expr::expect(tok)?;
        if !tok.next_is(&Token::Arrow)? {
            return Ok(Some(Self {
                rcv: None,
                arg: one,
                ret: two,
            }));
        }

        tok.expect(&Token::Arrow)?;
        Ok(Some(Self {
            rcv: one.into(),
            arg: two,
            ret: Expr::expect(tok)?,
        }))
    }
}

impl AstNode for FnDecl {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if !tok.next_is(&Token::Fn)? {
            return Ok(None);
        }

        tok.expect(&Token::Fn)?;
        let ident = Ident::expect(tok)?;
        let sig = if tok.next_is(&Token::Colon)? {
            tok.expect(&Token::Colon)?;
            FnSigDecl::expect(tok)?.into()
        } else {
            None
        };

        let body = ExecScope::parse(tok)?;
        if body.is_none() {
            tok.expect(&Token::Semicolon)?;
        }

        Ok(Some(Self { ident, sig, body }))
    }
}

impl AstNode for IfaceDecl {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if !tok.next_is(&Token::Iface)? {
            return Ok(None);
        }

        tok.expect(&Token::Iface)?;
        let ident = Ident::expect(tok)?;
        let typ = if tok.next_is(&Token::Colon)? {
            tok.expect(&Token::Colon)?;
            Expr::expect(tok)?.into()
        } else {
            None
        };

        let scope = EvalScope::expect(tok)?;
        Ok(Some(Self { ident, typ, scope }))
    }
}

impl AstNode for ModDecl {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if !tok.next_is(&Token::Mod)? {
            return Ok(None);
        }

        tok.expect(&Token::Mod)?;
        let ident = Ident::expect(tok)?;
        let typ = if tok.next_is(&Token::Colon)? {
            tok.expect(&Token::Colon)?;
            Expr::expect(tok)?.into()
        } else {
            None
        };

        let scope = EvalScope::parse(tok)?;
        if scope.is_none() {
            tok.expect(&Token::Semicolon)?;
        };

        Ok(Some(Self { ident, typ, scope }))
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
            tok, Self, VisDecl, AliasDecl, FnDecl, IfaceDecl, ModDecl, Annotation
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

impl From<FnDecl> for Decl {
    fn from(value: FnDecl) -> Self {
        Self::Fn(value)
    }
}

impl From<IfaceDecl> for Decl {
    fn from(value: IfaceDecl) -> Self {
        Self::Iface(value)
    }
}

impl From<ModDecl> for Decl {
    fn from(value: ModDecl) -> Self {
        Self::Mod(value)
    }
}

impl From<Annotation> for Decl {
    fn from(value: Annotation) -> Self {
        Self::Annotation(value.into())
    }
}
