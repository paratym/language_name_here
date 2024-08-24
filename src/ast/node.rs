use crate::{
    ast::{AstBuildError, AstBuildErrorKind, AstBuildResult},
    parser::Rule,
};
use pest::{
    error::LineColLocation,
    iterators::{Pair, Pairs},
};
use std::{any::type_name, fmt::Debug};

pub trait AstNode: Sized + Debug {
    fn parse(pairs: &mut Pairs<Rule>) -> AstBuildResult<Option<Self>>;

    fn expect(pairs: &mut Pairs<Rule>) -> AstBuildResult<Self> {
        let Some(node) = Self::parse(pairs)? else {
            let next = pairs.peek();

            return Err(AstBuildError {
                pos: next.as_ref().map(Pair::line_col).map(LineColLocation::Pos),
                kind: AstBuildErrorKind::ExpectedNode {
                    expected_node: type_name::<Self>(),
                    rejected_rule: next.as_ref().map(Pair::as_rule),
                },
            });
        };

        Ok(node)
    }
}

macro_rules! match_next {
    ($pairs:expr, $rule:expr) => {
        match crate::parser::PairsExt::next_if($pairs, $rule) {
            Some(pair) => pair,
            None => return Ok(None),
        }
    };
}

pub(crate) use match_next;
