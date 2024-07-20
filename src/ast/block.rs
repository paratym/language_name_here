use crate::{
    ast::{AstNode, Decl, ParseErr, ParseResult, Stmt},
    tok::{Token, Tokenizer},
};
use std::io::BufRead;

#[derive(Debug, Default)]
pub struct GlobalEvalScope {
    pub decls: Vec<Decl>,
}

#[derive(Debug)]
pub struct EvalScope {
    pub decls: Vec<Decl>,
}

#[derive(Debug)]
pub struct ExecScope {
    pub stmts: Vec<Stmt>,
}

impl GlobalEvalScope {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn merge(&mut self, other: Self) {
        self.decls.extend(other.decls)
    }
}

impl AstNode for GlobalEvalScope {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        Ok(parse_scope(tok, false)?.map(|decls| Self { decls }))
    }
}

impl AstNode for EvalScope {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        Ok(parse_scope(tok, true)?.map(|decls| Self { decls }))
    }
}

impl AstNode for ExecScope {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        Ok(parse_scope(tok, true)?.map(|stmts| Self { stmts }))
    }
}

fn parse_scope<N: AstNode>(
    tok: &mut Tokenizer<impl BufRead>,
    inline: bool,
) -> ParseResult<Option<Vec<N>>> {
    if inline {
        if tok.peek_token()?.tok != Token::LCurlyBrace {
            return Ok(None);
        }

        tok.expect_token(&Token::LCurlyBrace)?;
    }

    if inline {
        tok.expect_token(&Token::RCurlyBrace)?;
    }

    Ok(Some(items))
}
