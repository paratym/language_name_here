use crate::{
    ast::{AstNode, Decl, ParseResult, Stmt},
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

impl AstNode for EvalScope {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        Ok(parse_scope(tok)?.map(|decls| Self { decls }))
    }
}

impl AstNode for ExecScope {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        Ok(parse_scope(tok)?.map(|stmts| Self { stmts }))
    }
}

fn parse_scope<N: AstNode>(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Vec<N>>> {
    if !tok.next_is(&Token::LCurlyBrace)? {
        return Ok(None);
    }

    tok.expect(&Token::LCurlyBrace)?;
    let mut items = Vec::new();

    loop {
        match N::parse(tok) {
            Ok(Some(item)) => items.push(item),
            Ok(None) => break,
            Err(e) => return Err(e),
        }
    }

    tok.expect(&Token::RCurlyBrace)?;
    Ok(Some(items))
}
