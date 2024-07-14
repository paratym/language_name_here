use crate::tok::SrcPosition;

#[derive(Debug)]
pub enum TokErr {
    Io(std::io::Error),
    Syntax { pos: SrcPosition, msg: String },
}

pub type TokResult<T> = Result<T, TokErr>;

impl From<std::io::Error> for TokErr {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
