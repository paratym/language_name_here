use crate::lex::SrcPosition;

#[derive(Debug)]
pub enum LexErr {
    Syntax { pos: SrcPosition, msg: &'static str },
    Io(std::io::Error),
}

pub type LexResult<T> = Result<T, LexErr>;

impl From<std::io::Error> for LexErr {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
