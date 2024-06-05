use crate::tokenizer::{SrcPosition, TokErr};
use std::{convert::Infallible, io};

#[derive(Debug)]
pub enum ParseErr {
    Io(io::Error),
    Syntax { pos: SrcPosition, msg: &'static str },
    TokenStreamEmpty,
}

pub type ParseResult<T> = Result<T, ParseErr>;

impl From<Infallible> for ParseErr {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}

impl From<io::Error> for ParseErr {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<TokErr> for ParseErr {
    fn from(err: TokErr) -> Self {
        match err {
            TokErr::Syntax { pos, msg } => Self::Syntax { pos, msg },
            TokErr::Io(e) => Self::from(e),
            TokErr::ReaderEmpty => Self::TokenStreamEmpty,
        }
    }
}
