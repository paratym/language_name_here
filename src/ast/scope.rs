use crate::{
    ast::{AstNode, Decl, ParseErr, ParseResult, Stmt},
    tokenizer::{Token, Tokenizer},
};

#[derive(Debug)]
pub struct ConstScope {
    pub decls: Vec<Decl>,
}

#[derive(Debug)]
pub struct ExecScope {
    pub stmts: Vec<Stmt>,
}

impl AstNode for ConstScope {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        Ok(ConstScope {
            decls: parse_scoped_items(tok, false)?,
        })
    }
}

impl AstNode for ExecScope {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        Ok(ExecScope {
            stmts: parse_scoped_items(tok, true)?,
        })
    }
}

fn parse_scoped_items<N: AstNode>(tok: &mut Tokenizer, inline: bool) -> ParseResult<Vec<N>> {
    if inline {
        let pos = *tok.pos();
        let token = tok.next_token()?;

        if token.tok != Token::LCurlyBrace {
            return Err(ParseErr::Syntax {
                pos,
                msg: "exptected '{'",
            });
        }
    }

    let mut items = Vec::new();
    let mut scope_closed = false;

    loop {
        match N::parse(tok) {
            Ok(item) => items.push(item),
            Err(ParseErr::TokenStreamEmpty) => break,
            Err(e) => return Err(e),
        }

        if tok.peek_token()?.tok == Token::RCurlyBrace {
            tok.next_token()?;
            scope_closed = true;
            break;
        }
    }

    if inline && !scope_closed {
        return Err(ParseErr::Syntax {
            pos: *tok.pos(),
            msg: "exptected '}'",
        });
    }

    Ok(items)
}
