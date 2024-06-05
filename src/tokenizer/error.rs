use crate::tokenizer::SrcPosition;

#[derive(Debug)]
pub enum TokErr {
    Io(std::io::Error),
    Syntax { pos: SrcPosition, msg: &'static str },
    ReaderEmpty,
}

pub type TokResult<T> = Result<T, TokErr>;

impl From<std::io::Error> for TokErr {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
