use lazy_static::lazy_static;
use pest::{
    iterators::{Pair, Pairs},
    pratt_parser::PrattParser,
};
use pest_derive::Parser as PestParser;

#[derive(PestParser)]
#[grammar = "grammar.pest"]
pub struct Parser;

lazy_static! {
    pub static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        PrattParser::new()
            .op(Op::infix(pipe_r, Left) | Op::infix(pipe_l, Right))
            .op(Op::infix(exec_path, Left))
            .op(Op::infix(eval_path, Left))
            .op(Op::postfix(unwrap))
            .op(Op::postfix(reference) | Op::postfix(dereference))
    };
}

pub trait PairsExt<'i> {
    fn next_if(&mut self, rule: Rule) -> Option<Pair<Rule>>;
}

impl<'i> PairsExt<'i> for Pairs<'i, Rule> {
    fn next_if(&mut self, rule: Rule) -> Option<Pair<Rule>> {
        if self.peek().is_some_and(|pair| pair.as_rule() == rule) {
            self.next()
        } else {
            None
        }
    }
}
