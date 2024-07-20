use std::{any::type_name, fmt::Debug, io::BufRead};

use crate::{
    ast::{ParseErr, ParseResult},
    tok::Tokenizer,
};

pub trait AstNode: Debug + Sized {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>>;

    fn expect(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Self> {
        Self::parse(tok)?.ok_or(ParseErr::Syntax {
            pos: *tok.pos(),
            msg: format!("expected {}", type_name::<Self>()),
        })
    }
}

macro_rules! first_match {
    ($tok:ident, $self:ty, $first:ty, $($rest:ty),+) => {
        if let Some(first) = <$first as AstNode>::parse($tok)? {
            Some(<$self>::from(first))
        } $(else if let Some(next) = <$rest as AstNode>::parse($tok)? {
            Some(<$self>::from(next))
        })+ else {
            None
        }
    };
}

macro_rules! first_match_chain {
    ($tok:ident, $self:ty, $base:expr, $first:ty, $($rest:ty),*) => {
       if let Some(first) = <$first as AstNode>::parse($tok)? {
            (true, <$self>::from(($base, first)))
        } $(else if let Some(next) = <$rest as AstNode>::parse($tok)? {
            (true, <$self>::from(($base, next)))
        })* else {
            (false, <$self>::from($base))
        }
    };
}

pub(super) use first_match;
pub(super) use first_match_chain;
