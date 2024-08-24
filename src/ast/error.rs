use crate::parser::Rule;
use pest::error::LineColLocation;

#[derive(Debug)]
pub enum AstBuildErrorKind {
    ExpectedNode {
        expected_node: &'static str,
        rejected_rule: Option<Rule>,
    },
    NumOverflow {},
}

#[derive(Debug)]
pub struct AstBuildError {
    pub kind: AstBuildErrorKind,
    pub pos: Option<LineColLocation>,
}

pub type AstBuildResult<T> = Result<T, AstBuildError>;

macro_rules! unreachable_grammar {
    ($self:ty) => {
        unreachable!(
            "unreachable grammar encountered in node: {}",
            std::any::type_name::<$self>()
        )
    };
}

pub(crate) use unreachable_grammar;

macro_rules! assert_exhausted {
    ($pairs:expr, $self:ty) => {
        debug_assert!(
            $pairs.len() == 0,
            "unused pairs left in node: {}",
            std::any::type_name::<$self>(),
        )
    };
}

pub(crate) use assert_exhausted;

impl From<lexical::Error> for AstBuildErrorKind {
    fn from(value: lexical::Error) -> Self {
        todo!()
    }
}
