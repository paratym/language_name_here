use crate::{
    ast::{AstNode, ParseErr, ParseResult},
    tok::{SrcToken, Token, Tokenizer},
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

macro_rules! match_lit {
    ($tok:expr, $lit:ident) => {{
        if !matches!($tok.peek()?, Some(Token::$lit(_))) {
            return Ok(None);
        }

        match $tok.next_tok()? {
            Some(SrcToken {
                tok: Token::$lit(lit),
                ..
            }) => lit,
            _ => unreachable!(),
        }
    }};
}

impl AstNode for BoolLit {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        let val = match tok.peek()? {
            Some(Token::True) => true,
            Some(Token::False) => false,
            _ => return Ok(None),
        };

        tok.next_tok()?;
        Ok(Some(Self { val }))
    }
}

impl AstNode for NumLit {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        let lit = match_lit!(tok, NumLit);
        let mut chars = lit.chars().peekable();

        let negative = match chars.peek() {
            Some('-' | '+') => chars.next() == Some('-'),
            _ => false,
        };

        let mut radix = 10;
        if chars.peek() == Some(&'0') {
            chars.next();

            if chars.peek().is_some_and(char::is_ascii_alphabetic) {
                radix = match chars.next() {
                    Some('b') => 2,
                    Some('o') => 8,
                    Some('x') => 16,
                    _ => {
                        return Err(ParseErr::Syntax {
                            pos: *tok.pos(),
                            msg: "unrecognized radix specifier".into(),
                        })
                    }
                }
            }
        }

        let whole = if chars.peek().is_some() {
            let whole_str = chars
                .by_ref()
                .take_while(|ch| *ch != '.')
                .collect::<String>();

            u64::from_str_radix(whole_str.as_str(), radix).map_err(|e| ParseErr::Syntax {
                pos: *tok.pos(),
                msg: format!("parse int error {:?}", e),
            })?
        } else {
            0
        };

        let frac = if chars.peek().is_some() {
            let frac_str = chars.collect::<String>();
            u64::from_str_radix(frac_str.as_str(), radix).map_err(|e| ParseErr::Syntax {
                pos: *tok.pos(),
                msg: format!("parse int error {:?}", e),
            })?
        } else {
            0
        };

        Ok(Some(Self {
            negative,
            whole,
            frac,
        }))
    }
}

impl AstNode for CharLit {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        let lit = match_lit!(tok, CharLit);
        todo!()
    }
}

impl AstNode for StrLit {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        let lit = match_lit!(tok, StrLit);
        todo!()
    }
}
