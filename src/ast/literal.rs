use crate::{
    ast::{AstNode, ParseErr, ParseResult, RhsExpr},
    tokenizer::{Token, Tokenizer},
};
use std::rc::Rc;

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

#[derive(Debug)]
pub struct ArrayLit {
    pub fields: Vec<RhsExpr>,
}

impl AstNode for BoolLit {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
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
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        let token = tok.peek_token()?;
        let lit = if let Token::NumLit(num_ref) = &token.tok {
            let num = num_ref.clone();
            tok.next_token()?;
            num
        } else {
            return Ok(None);
        };

        todo!()
    }
}

impl AstNode for CharLit {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        let token = tok.peek_token()?;
        let lit = if let Token::CharLit(chr_ref) = &token.tok {
            let chr = chr_ref.clone();
            tok.next_token()?;
            chr
        } else {
            return Ok(None);
        };

        todo!()
    }
}

impl AstNode for StrLit {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        let token = tok.peek_token()?;
        let lit = if let Token::StrLit(str_ref) = &token.tok {
            let string = str_ref.clone();
            tok.next_token()?;
            string
        } else {
            return Ok(None);
        };

        todo!()
    }
}

impl AstNode for ArrayLit {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        if tok.peek_token()?.tok != Token::LSqrBrace {
            return Ok(None);
        }

        tok.expect_token(&Token::LSqrBrace)?;

        let mut fields = Vec::new();
        loop {
            if tok.peek_token()?.tok == Token::RSqrBrace {
                break;
            }

            fields.push(RhsExpr::expect(tok)?);

            match tok.peek_token()?.tok {
                Token::RSqrBrace => break,
                Token::Comma => tok.expect_token(&Token::Comma)?,
                _ => {
                    let token = tok.next_token()?;
                    return Err(ParseErr::Syntax {
                        pos: token.pos,
                        msg: "expected ',' or ']'",
                    });
                }
            };
        }

        tok.expect_token(&Token::RSqrBrace)?;
        Ok(Some(Self { fields }))
    }
}
