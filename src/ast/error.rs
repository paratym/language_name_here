use crate::tok::{SrcPosition, TokErr};
use std::convert::Infallible;

#[derive(Debug)]
pub enum ParseErr {
    Io(std::io::Error),
    Syntax { pos: SrcPosition, msg: String },
}

pub type ParseResult<T> = Result<T, ParseErr>;

impl From<Infallible> for ParseErr {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}

impl From<std::io::Error> for ParseErr {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<TokErr> for ParseErr {
    fn from(err: TokErr) -> Self {
        match err {
            TokErr::Syntax { pos, msg } => Self::Syntax { pos, msg },
            TokErr::Io(e) => Self::from(e),
        }
    }
}
