use crate::{ast::ParseErr, tok::SrcPosition};

#[derive(Debug)]
pub enum IoErr {
    Io(std::io::Error),
    Syntax {
        path: String,
        pos: SrcPosition,
        msg: String,
    },
}

pub type IoResult<T> = Result<T, IoErr>;

impl IoErr {
    pub fn from_parse_err(err: ParseErr, path: String) -> Self {
        match err {
            ParseErr::Io(e) => Self::Io(e),
            ParseErr::Syntax { pos, msg } => Self::Syntax { path, pos, msg },
        }
    }
}

impl From<std::io::Error> for IoErr {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
