use crate::{
    ast::{
        assert_exhausted, match_next, unreachable_grammar, Alias, AstBuildResult, AstNode, Block,
        Expr,
    },
    parser::Rule,
};
use pest::iterators::{Pair, Pairs};

#[derive(Debug)]
pub struct Bounds {
    pub expr: Expr,
}

#[derive(Debug)]
pub enum AliasEval {
    Let,
    Var,
    Const,
    Type,
}

#[derive(Debug)]
pub struct AliasDecl {
    pub eval: AliasEval,
    pub alias: Alias,
    pub bounds: Option<Bounds>,
    pub rhs: Expr,
}

#[derive(Debug)]
pub struct FnDecl {
    pub alias: Alias,
    pub bounds: Option<Bounds>,
    pub body: Block,
}

#[derive(Debug)]
pub enum Decl {
    Alias(AliasDecl),
    Fn(FnDecl),
}

impl AstNode for Bounds {
    fn parse(pairs: &mut Pairs<Rule>) -> AstBuildResult<Option<Self>> {
        let mut pairs = match_next!(pairs, Rule::bounds).into_inner();
        let expr = Expr::expect(&mut pairs)?;

        assert_exhausted!(pairs, Self);
        Ok(Some(Self { expr }))
    }
}

impl AstNode for AliasEval {
    fn parse(pairs: &mut Pairs<Rule>) -> AstBuildResult<Option<Self>> {
        let mut pairs = match_next!(pairs, Rule::alias_eval).into_inner();
        let eval = match pairs.next().as_ref().map(Pair::as_rule) {
            Some(Rule::kw_let) => Self::Let,
            Some(Rule::kw_var) => Self::Var,
            Some(Rule::kw_const) => Self::Const,
            Some(Rule::kw_type) => Self::Type,
            _ => unreachable_grammar!(Self),
        };

        assert_exhausted!(pairs, Self);
        Ok(Some(eval))
    }
}

impl AstNode for AliasDecl {
    fn parse(pairs: &mut Pairs<Rule>) -> AstBuildResult<Option<Self>> {
        let mut pairs = match_next!(pairs, Rule::alias_decl).into_inner();

        let eval = AliasEval::expect(&mut pairs)?;
        let alias = Alias::expect(&mut pairs)?;
        let bounds = Bounds::parse(&mut pairs)?;
        let rhs = Expr::expect(&mut pairs)?;

        assert_exhausted!(pairs, Self);
        Ok(Some(Self {
            eval,
            alias,
            bounds,
            rhs,
        }))
    }
}

impl AstNode for FnDecl {
    fn parse(pairs: &mut Pairs<Rule>) -> AstBuildResult<Option<Self>> {
        let mut pairs = match_next!(pairs, Rule::fn_decl).into_inner();

        let alias = Alias::expect(&mut pairs)?;
        let bounds = Bounds::parse(&mut pairs)?;
        let body = Block::expect(&mut pairs)?;

        assert_exhausted!(pairs, Self);
        Ok(Some(Self {
            alias,
            bounds,
            body,
        }))
    }
}

impl AstNode for Decl {
    fn parse(pairs: &mut Pairs<Rule>) -> AstBuildResult<Option<Self>> {
        let mut pairs = match_next!(pairs, Rule::decl).into_inner();

        let decl = match pairs.peek().as_ref().map(Pair::as_rule) {
            Some(Rule::alias_decl) => AliasDecl::expect(&mut pairs).map(Self::Alias),
            Some(Rule::fn_decl) => FnDecl::expect(&mut pairs).map(Self::Fn),
            _ => unreachable_grammar!(Self),
        };

        assert_exhausted!(pairs, Self);
        decl.map(Into::into)
    }
}
