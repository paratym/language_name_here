use crate::{
    ast::{assert_exhausted, match_next, unreachable_grammar, AstBuildResult, AstNode, Decl, Expr},
    parser::Rule,
};
use pest::iterators::{Pair, Pairs};

#[derive(Debug)]
pub struct AssignStmt {
    pub lhs: Expr,
    pub rhs: Expr,
}

#[derive(Debug)]
pub enum CtrlOp {
    Return,
    Defer,
    Continue,
    Break,
}

#[derive(Debug)]
pub struct CtrlStmt {
    pub op: CtrlOp,
    pub rhs: Option<Expr>,
}

#[derive(Debug)]
pub struct IfStmt {
    pub lhs: Expr,
    pub rhs: Expr,
    pub chain: Option<Expr>,
}

#[derive(Debug)]
pub struct WhileStmt {
    pub lhs: Expr,
    pub rhs: Expr,
}

#[derive(Debug)]
pub enum Stmt {
    Assign(AssignStmt),
    Ctrl(CtrlStmt),
    If(IfStmt),
    While(WhileStmt),

    Decl(Decl),
}

impl AstNode for AssignStmt {
    fn parse(pairs: &mut Pairs<Rule>) -> AstBuildResult<Option<Self>> {
        let mut pairs = match_next!(pairs, Rule::assign_stmt).into_inner();

        let lhs = Expr::expect(&mut pairs)?;
        let rhs = Expr::expect(&mut pairs)?;

        assert_exhausted!(pairs, Self);
        Ok(Some(Self { lhs, rhs }))
    }
}

impl AstNode for CtrlOp {
    fn parse(pairs: &mut Pairs<Rule>) -> AstBuildResult<Option<Self>> {
        let mut pairs = match_next!(pairs, Rule::ctrl_op).into_inner();

        let op = match pairs.next().as_ref().map(Pair::as_rule) {
            Some(Rule::kw_return) => Self::Return,
            Some(Rule::kw_defer) => Self::Defer,
            Some(Rule::kw_continue) => Self::Continue,
            Some(Rule::kw_break) => Self::Break,
            _ => unreachable_grammar!(Self),
        };

        assert_exhausted!(pairs, Self);
        Ok(Some(op))
    }
}

impl AstNode for CtrlStmt {
    fn parse(pairs: &mut Pairs<Rule>) -> AstBuildResult<Option<Self>> {
        let mut pairs = match_next!(pairs, Rule::ctrl_stmt).into_inner();

        let op = CtrlOp::expect(&mut pairs)?;
        let rhs = Expr::parse(&mut pairs)?;

        assert_exhausted!(pairs, Self);
        Ok(Some(Self { op, rhs }))
    }
}

impl AstNode for IfStmt {
    fn parse(pairs: &mut Pairs<Rule>) -> AstBuildResult<Option<Self>> {
        let mut pairs = match_next!(pairs, Rule::if_stmt).into_inner();

        let lhs = Expr::expect(&mut pairs)?;
        let rhs = Expr::expect(&mut pairs)?;
        let chain = Expr::parse(&mut pairs)?;

        assert_exhausted!(pairs, Self);
        Ok(Some(Self { lhs, rhs, chain }))
    }
}

impl AstNode for WhileStmt {
    fn parse(pairs: &mut Pairs<Rule>) -> AstBuildResult<Option<Self>> {
        let mut pairs = match_next!(pairs, Rule::while_stmt).into_inner();

        let lhs = Expr::expect(&mut pairs)?;
        let rhs = Expr::expect(&mut pairs)?;

        assert_exhausted!(pairs, Self);
        Ok(Some(Self { lhs, rhs }))
    }
}

impl AstNode for Stmt {
    fn parse(pairs: &mut Pairs<Rule>) -> AstBuildResult<Option<Self>> {
        let mut pairs = match_next!(pairs, Rule::stmt).into_inner();

        let stmt = match pairs.peek().as_ref().map(Pair::as_rule) {
            Some(Rule::assign_stmt) => AssignStmt::expect(&mut pairs).map(Self::Assign),
            Some(Rule::ctrl_stmt) => CtrlStmt::expect(&mut pairs).map(Self::Ctrl),
            Some(Rule::if_stmt) => IfStmt::expect(&mut pairs).map(Self::If),
            Some(Rule::while_stmt) => WhileStmt::expect(&mut pairs).map(Self::While),

            Some(Rule::decl) => Decl::expect(&mut pairs).map(Self::Decl),

            _ => unreachable_grammar!(Self),
        };

        assert_exhausted!(pairs, Self);
        stmt.map(Into::into)
    }
}
