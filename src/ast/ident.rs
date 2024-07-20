use crate::{
    ast::{AstNode, ParseResult},
    tok::{SrcToken, Token, Tokenizer},
};
use std::{io::BufRead, rc::Rc, sync::Arc};

use super::{first_match, first_match_chain};

#[derive(Debug)]
pub enum ScopeAlias {
    Pkg,
    Mod,
    Std,
    Ext,
}

#[derive(Debug)]
pub struct Alias {
    pub alias: Arc<str>,
}

#[derive(Debug)]
pub struct IdentEvalPath {
    pub path: Ident,
}

#[derive(Debug)]
pub struct IdentExecPath {
    pub path: Ident,
}

#[derive(Debug)]
pub enum Ident {
    Scope(ScopeAlias),
    Alias(Alias),
    EvalPath {
        rcv: Rc<Self>,
        path: Rc<IdentEvalPath>,
    },
    ExecPath {
        rcv: Rc<Self>,
        path: Rc<IdentExecPath>,
    },
}

impl AstNode for ScopeAlias {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        let scope = match tok.peek()? {
            Some(Token::Pkg) => Self::Pkg,
            Some(Token::Mod) => Self::Mod,
            _ => return Ok(None),
        };

        tok.next_tok()?;
        Ok(Some(scope))
    }
}

impl AstNode for Alias {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        match tok.peek()? {
            Some(Token::Alias(_alias)) => {
                let alias = _alias.clone();
                tok.next_tok()?;
                Ok(Some(Self { alias }))
            }
            _ => Ok(None),
        }
    }
}

impl AstNode for IdentEvalPath {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if !tok.next_is(&Token::DoubleColon)? {
            return Ok(None);
        }

        tok.expect(&Token::DoubleColon)?;
        let path = Ident::expect(tok)?;
        Ok(Some(Self { path }))
    }
}

impl AstNode for IdentExecPath {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if !tok.next_is(&Token::Dot)? {
            return Ok(None);
        }

        tok.expect(&Token::Dot)?;
        let path = Ident::expect(tok)?;
        Ok(Some(Self { path }))
    }
}

impl AstNode for Ident {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        let rcv = first_match!(tok, Self, ScopeAlias, Alias);
        let mut expr = match rcv {
            Some(expr) => expr,
            None => return Ok(None),
        };

        loop {
            let path = first_match_chain!(tok, Self, expr, IdentEvalPath, IdentExecPath);
            match path {
                (true, path) => expr = path,
                (false, expr) => return Ok(expr.into()),
            }
        }
    }
}

impl From<ScopeAlias> for Ident {
    fn from(value: ScopeAlias) -> Self {
        Self::Scope(value)
    }
}

impl From<Alias> for Ident {
    fn from(value: Alias) -> Self {
        Self::Alias(value)
    }
}

impl From<(Self, IdentEvalPath)> for Ident {
    fn from(value: (Self, IdentEvalPath)) -> Self {
        Self::EvalPath {
            rcv: value.0.into(),
            path: value.1.into(),
        }
    }
}

impl From<(Self, IdentExecPath)> for Ident {
    fn from(value: (Self, IdentExecPath)) -> Self {
        Self::ExecPath {
            rcv: value.0.into(),
            path: value.1.into(),
        }
    }
}
