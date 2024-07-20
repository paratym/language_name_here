use crate::{
    ast::{first_match, first_match_chain, AstNode, Decl, ExecScope, Expr, ParseErr, ParseResult},
    tok::{Token, Tokenizer},
};
use std::{io::BufRead, rc::Rc};

#[derive(Debug)]
pub struct AssignStmt {
    pub rhs: Expr,
}

#[derive(Debug)]
pub enum UnaryOp {
    Return,
    Defer,
    Continue,
    Break,
}

#[derive(Debug)]
pub struct UnaryStmt {
    pub op: UnaryOp,
    pub rhs: Expr,
}

#[derive(Debug)]
pub struct IfStmt {
    pub cond: Expr,
    pub body: ExecScope,
    pub chain: Option<ElseStmt>,
}

#[derive(Debug)]
pub enum ElseStmt {
    ElseIf(Rc<IfStmt>),
    Else(ExecScope),
}

#[derive(Debug)]
pub struct WhileStmt {
    pub cond: Expr,
    pub body: ExecScope,
}

// #[derive(Debug)]
// pub struct MatchStmt {
//     pub val: RhsExpr,
//     pub branches: Vec<(RhsExpr, RhsExpr)>,
// }

#[derive(Debug)]
pub enum Stmt {
    Decl(Decl),
    Expr(Expr),

    Assign { lhs: Expr, stmt: AssignStmt },
    Unary(UnaryStmt),
    If(IfStmt),
    While(WhileStmt),
}

impl AstNode for AssignStmt {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if !tok.next_is(&Token::Equal)? {
            return Ok(None);
        }

        tok.expect(&Token::Equal)?;
        let rhs = Expr::expect(tok)?;
        tok.expect(&Token::Semicolon)?;

        Ok(Some(Self { rhs }))
    }
}

impl AstNode for UnaryOp {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        let op = match tok.peek()? {
            Some(Token::Return) => Self::Return,
            Some(Token::Defer) => Self::Defer,
            Some(Token::Continue) => Self::Continue,
            Some(Token::Break) => Self::Break,
            _ => return Ok(None),
        };

        tok.next_tok()?;
        Ok(Some(op))
    }
}

impl AstNode for UnaryStmt {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        let op = if let Some(op) = UnaryOp::parse(tok)? {
            op
        } else {
            return Ok(None);
        };

        let rhs = Expr::expect(tok)?;
        tok.expect(&Token::Semicolon)?;

        Ok(Some(Self { op, rhs }))
    }
}

impl AstNode for IfStmt {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if !tok.next_is(&Token::If)? {
            return Ok(None);
        }

        tok.expect(&Token::If)?;
        let cond = Expr::expect(tok)?;
        let body = ExecScope::expect(tok)?;
        let chain = ElseStmt::parse(tok)?;

        Ok(Some(Self { cond, body, chain }))
    }
}

impl AstNode for ElseStmt {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if !tok.next_is(&Token::Else)? {
            return Ok(None);
        }

        tok.expect(&Token::Else)?;
        Ok(Some(if let Some(cond) = IfStmt::parse(tok)? {
            Self::ElseIf(cond.into())
        } else if let Some(scope) = ExecScope::parse(tok)? {
            Self::Else(scope)
        } else {
            return Err(ParseErr::Syntax {
                pos: *tok.pos(),
                msg: "expected an if statement or an execution scope".into(),
            });
        }))
    }
}

impl AstNode for WhileStmt {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if !tok.next_is(&Token::While)? {
            return Ok(None);
        }

        tok.expect(&Token::While)?;
        let cond = Expr::expect(tok)?;
        let body = ExecScope::expect(tok)?;

        Ok(Some(Self { cond, body }))
    }
}

// impl AstNode for MatchStmt {
//     fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
//         if tok.peek_token()?.tok == Token::Match {
//             tok.expect_token(&Token::Match)?;
//         } else {
//             return Ok(None);
//         }
//
//         let val = RhsExpr::expect(tok)?;
//         tok.expect_token(&Token::LCurlyBrace)?;
//
//         let mut branches = Vec::new();
//         loop {
//             if tok.peek_token()?.tok == Token::RCurlyBrace {
//                 break;
//             }
//
//             let lhs = LhsExpr::expect(tok)?;
//             tok.expect_token(&Token::Arrow)?;
//             let rhs = RhsExpr::expect(tok)?;
//             branches.push((lhs, rhs));
//
//             if tok.peek_token()?.tok == Token::RCurlyBrace {
//                 break;
//             } else {
//                 tok.expect_token(&Token::Comma)
//             };
//         }
//
//         tok.expect_token(&Token::RCurlyBrace)?;
//         Ok(Some(Self { val, branches }))
//     }
// }

impl AstNode for Stmt {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        let base = first_match!(tok, Self, Decl, UnaryStmt, IfStmt, WhileStmt, Expr);
        let expr = match base {
            Some(Stmt::Expr(expr)) => expr,
            _ => return Ok(base),
        };

        Ok(Some(first_match_chain!(tok, Self, expr, AssignStmt,).1))
    }
}

impl From<Decl> for Stmt {
    fn from(value: Decl) -> Self {
        Self::Decl(value)
    }
}

impl From<(Expr, AssignStmt)> for Stmt {
    fn from(value: (Expr, AssignStmt)) -> Self {
        Self::Assign {
            lhs: value.0,
            stmt: value.1,
        }
    }
}

impl From<UnaryStmt> for Stmt {
    fn from(value: UnaryStmt) -> Self {
        Self::Unary(value)
    }
}

impl From<IfStmt> for Stmt {
    fn from(value: IfStmt) -> Self {
        Self::If(value)
    }
}

impl From<WhileStmt> for Stmt {
    fn from(value: WhileStmt) -> Self {
        Self::While(value)
    }
}

impl From<Expr> for Stmt {
    fn from(value: Expr) -> Self {
        Self::Expr(value)
    }
}
