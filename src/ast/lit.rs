use crate::{
    ast::{AstNode, ParseResult},
    tok::{Token, Tokenizer},
};
use std::{io::BufRead, rc::Rc};

#[derive(Debug)]
pub struct BoolLit {
    pub val: bool,
}

#[derive(Debug)]
pub struct NumLit {
    pub negative: bool,
    pub whole: u64,
    pub frac: u64,
}

#[derive(Debug)]
pub struct CharLit {
    pub val: char,
}

#[derive(Debug)]
pub struct StrLit {
    pub val: Rc<str>,
}

impl AstNode for BoolLit {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        let val = match tok.peek_token()?.tok {
            Token::True => true,
            Token::False => false,
            _ => return Ok(None),
        };

        tok.next_token()?;
        Ok(Some(Self { val }))
    }
}

impl AstNode for NumLit {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        let token = tok.peek_token()?;
        let lit = if let Token::NumLit(num_ref) = &token.tok {
            num_ref.clone()
        } else {
            return Ok(None);
        };

        tok.next_token()?;
        todo!()
    }
}

impl AstNode for CharLit {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        let token = tok.peek_token()?;
        let lit = if let Token::CharLit(chr_ref) = &token.tok {
            chr_ref.clone()
        } else {
            return Ok(None);
        };

        tok.next_token()?;
        todo!()
    }
}

impl AstNode for StrLit {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        let token = tok.peek_token()?;
        let lit = if let Token::StrLit(str_ref) = &token.tok {
            str_ref.clone()
        } else {
            return Ok(None);
        };

        tok.next_token()?;
        todo!()
    }
}
