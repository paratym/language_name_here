use crate::{
    ast::{AstNode, LhsExpr, ParseErr, ParseResult, RhsExpr},
    tokenizer::{SrcToken, Token, Tokenizer},
};
use std::rc::Rc;

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
    pub entries: Vec<RhsExpr>,
}

#[derive(Debug)]
pub struct CompoundLit {
    pub fields: Vec<(LhsExpr, RhsExpr)>,
}

impl AstNode for NumLit {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        let token = tok.next_token()?;
        let lit = if let Token::NumLit(num) = token.tok {
            num
        } else {
            return Err(ParseErr::Syntax {
                pos: token.pos,
                msg: "expected number literal",
            });
        };

        todo!()
    }
}

impl AstNode for CharLit {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        let token = tok.next_token()?;
        let lit = if let Token::CharLit(chr) = token.tok {
            chr
        } else {
            return Err(ParseErr::Syntax {
                pos: token.pos,
                msg: "expected character literal",
            });
        };

        todo!()
    }
}

impl AstNode for StrLit {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        let token = tok.next_token()?;
        let lit = if let Token::StrLit(str) = token.tok {
            str
        } else {
            return Err(ParseErr::Syntax {
                pos: token.pos,
                msg: "expected string literal",
            });
        };

        todo!()
    }
}

impl AstNode for ArrayLit {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        tok.expect_token(&Token::LSqrBrace)?;
        let mut lit = Self {
            entries: Vec::new(),
        };

        loop {
            if tok.peek_token()?.tok == Token::RSqrBrace {
                return Ok(lit);
            }

            lit.entries.push(RhsExpr::parse(tok)?);

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
        Ok(lit)
    }
}

impl AstNode for CompoundLit {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        tok.expect_token(&Token::LParen)?;
        let mut lit = Self { fields: Vec::new() };

        loop {
            if tok.peek_token()?.tok == Token::Rparen {
                break;
            }

            let field = LhsExpr::parse(tok)?;
            tok.expect_token(&Token::Equal)?;
            let value = RhsExpr::parse(tok)?;
            lit.fields.push((field, value));

            match tok.peek_token()?.tok {
                Token::Rparen => break,
                Token::Comma => tok.expect_token(&Token::Comma)?,
                _ => {
                    let token = tok.next_token()?;
                    return Err(ParseErr::Syntax {
                        pos: token.pos,
                        msg: "expected ',' or ')'",
                    });
                }
            };
        }

        tok.expect_token(&Token::Rparen)?;
        Ok(lit)
    }
}
