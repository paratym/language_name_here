use crate::{
    ast::{
        assert_exhausted, match_next, unreachable_grammar, AstBuildResult, AstNode, BoolLit,
        ChrLit, FloatLit, IfStmt, IntLit, Stmt, StrLit, WhileStmt,
    },
    parser::{Rule, PRATT_PARSER},
};
use pest::iterators::{Pair, Pairs};
use std::rc::Rc;

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub expr: Option<Rc<Expr>>,
}

#[derive(Debug)]
pub struct Alias {
    pub alias: String,
}

#[derive(Debug)]
pub enum PostOp {
    Ref,
    Deref,
    Unwrap,
}

#[derive(Debug)]
pub enum BinOp {
    EvalPath,
    ExecPath,
    PipeR,
    PipeL,
}

#[derive(Debug)]
pub enum Expr {
    Alias(Alias),
    Block(Block),

    Bool(BoolLit),
    Int(IntLit),
    Float(FloatLit),
    Chr(ChrLit),
    Str(StrLit),

    IfStmt(Rc<IfStmt>),
    WhileStmt(Rc<WhileStmt>),

    UnaryExpr {
        expr: Rc<Self>,
        op: PostOp,
    },
    BinaryExpr {
        lhs: Rc<Self>,
        rhs: Rc<Self>,
        op: BinOp,
    },
}

impl AstNode for Block {
    fn parse(pairs: &mut Pairs<Rule>) -> AstBuildResult<Option<Self>> {
        let mut pairs = match_next!(pairs, Rule::block).into_inner();

        let stmts = pairs
            .by_ref()
            .map_while(|pair| Stmt::parse(&mut Pairs::single(pair)).transpose())
            .collect::<Result<Vec<_>, _>>()?;

        let expr = Expr::parse(&mut pairs)?.map(Into::into);

        assert_exhausted!(pairs, Self);
        Ok(Some(Self { stmts, expr }))
    }
}

impl AstNode for Alias {
    fn parse(pairs: &mut Pairs<Rule>) -> AstBuildResult<Option<Self>> {
        let pair = match_next!(pairs, Rule::alias);
        let alias = pair.as_str().to_string();
        Ok(Some(Self { alias }))
    }
}

impl AstNode for PostOp {
    fn parse(pairs: &mut Pairs<Rule>) -> AstBuildResult<Option<Self>> {
        let op = match pairs.peek().as_ref().map(Pair::as_rule) {
            Some(Rule::reference) => Self::Ref,
            Some(Rule::dereference) => Self::Deref,
            Some(Rule::unwrap) => Self::Unwrap,
            _ => return Ok(None),
        };

        pairs.next();
        assert_exhausted!(pairs, Self);

        Ok(Some(op))
    }
}

impl AstNode for BinOp {
    fn parse(pairs: &mut Pairs<Rule>) -> AstBuildResult<Option<Self>> {
        let op = match pairs.peek().as_ref().map(Pair::as_rule) {
            Some(Rule::eval_path) => BinOp::EvalPath,
            Some(Rule::exec_path) => BinOp::ExecPath,
            Some(Rule::pipe_r) => BinOp::PipeR,
            Some(Rule::pipe_l) => BinOp::PipeL,
            _ => return Ok(None),
        };

        pairs.next();
        assert_exhausted!(pairs, Self);

        Ok(Some(op))
    }
}

impl AstNode for Expr {
    fn parse(pairs: &mut Pairs<Rule>) -> AstBuildResult<Option<Self>> {
        let pairs = match_next!(pairs, Rule::expr).into_inner();

        PRATT_PARSER
            .map_primary(|pair| {
                let rule = pair.as_rule();
                let mut pairs = Pairs::single(pair);

                match rule {
                    Rule::alias => Alias::expect(&mut pairs).map(Self::Alias),
                    Rule::block => Block::expect(&mut pairs).map(Self::Block),

                    Rule::bool => BoolLit::expect(&mut pairs).map(Self::Bool),
                    Rule::int => IntLit::expect(&mut pairs).map(Self::Int),
                    Rule::float => FloatLit::expect(&mut pairs).map(Self::Float),
                    Rule::chr => ChrLit::expect(&mut pairs).map(Self::Chr),
                    Rule::str => StrLit::expect(&mut pairs).map(Self::Str),

                    Rule::if_stmt => IfStmt::expect(&mut pairs).map(Into::into).map(Self::IfStmt),
                    Rule::while_stmt => WhileStmt::expect(&mut pairs)
                        .map(Into::into)
                        .map(Self::WhileStmt),

                    Rule::expr => Self::expect(&mut pairs),
                    _ => unreachable_grammar!(Self),
                }
            })
            .map_postfix(|expr, op_pair| {
                let expr = expr?.into();

                let mut op_pairs = Pairs::single(op_pair);
                let op = PostOp::expect(&mut op_pairs)?;

                Ok(Self::UnaryExpr { expr, op })
            })
            .map_infix(|lhs, op_pair, rhs| {
                let lhs = lhs?.into();
                let rhs = rhs?.into();

                let mut op_pairs = Pairs::single(op_pair);
                let op = BinOp::expect(&mut op_pairs)?;

                Ok(Self::BinaryExpr { lhs, rhs, op })
            })
            .parse(pairs)
            .map(Some)
    }
}
