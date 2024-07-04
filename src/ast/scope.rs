use crate::{
    ast::{AstNode, Decl, ParseResult, Stmt},
    tokenizer::{Token, Tokenizer},
};

#[derive(Debug)]
pub struct EvalScope {
    pub decls: Vec<Decl>,
}

#[derive(Debug)]
pub struct ExecScope {
    pub stmts: Vec<Stmt>,
}

impl AstNode for EvalScope {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        Ok(parse_scope(tok)?.map(|decls| Self { decls }))
    }
}

impl AstNode for ExecScope {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        Ok(parse_scope(tok)?.map(|stmts| Self { stmts }))
    }
}

fn parse_scope<N: AstNode>(tok: &mut Tokenizer) -> ParseResult<Option<Vec<N>>> {
    if tok.peek_token()?.tok == Token::LCurlyBrace {
        tok.expect_token(&Token::LCurlyBrace)?;
    } else {
        return Ok(None);
    }

    let mut items = Vec::new();
    loop {
        if tok.peek_token()?.tok == Token::RCurlyBrace {
            break;
        }

        items.push(N::expect(tok)?);
    }

    tok.expect_token(&Token::RCurlyBrace)?;
    Ok(Some(items))
}
