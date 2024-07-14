use crate::{
    ast::{first_match, AliasDecl, AstNode, ExecScope, Expr, ParseErr, ParseResult, UseDecl},
    tok::{Token, Tokenizer},
};
use std::{io::BufRead, rc::Rc};

#[derive(Debug)]
pub struct AssignStmt {
    pub lhs: Expr,
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
    AliasDecl(AliasDecl),
    UseDecl(UseDecl),
    Assign(AssignStmt),
    Unary(UnaryStmt),
    Expr(Expr),
    If(IfStmt),
    While(WhileStmt),
    // Match(MatchStmt),
}

impl AstNode for AssignStmt {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        let lhs = if let Some(expr) = Expr::parse(tok)? {
            expr
        } else {
            return Ok(None);
        };

        tok.expect_token(&Token::Equal)?;
        let rhs = Expr::expect(tok)?;
        tok.expect_token(&Token::Semicolon)?;

        Ok(Some(Self { lhs, rhs }))
    }
}

impl AstNode for UnaryOp {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        Ok(Some(match tok.peek_token()?.tok {
            Token::Return => Self::Return,
            Token::Defer => Self::Defer,
            Token::Continue => Self::Continue,
            Token::Break => Self::Break,
            _ => return Ok(None),
        }))
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
        tok.expect_token(&Token::Semicolon)?;

        Ok(Some(Self { op, rhs }))
    }
}

impl AstNode for IfStmt {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if tok.peek_token()?.tok == Token::If {
            tok.expect_token(&Token::If)?;
        } else {
            return Ok(None);
        }

        let cond = Expr::expect(tok)?;
        let body = ExecScope::expect(tok)?;
        let chain = ElseStmt::parse(tok)?;

        Ok(Some(Self { cond, body, chain }))
    }
}

impl AstNode for ElseStmt {
    fn parse(tok: &mut Tokenizer<impl BufRead>) -> ParseResult<Option<Self>> {
        if tok.peek_token()?.tok == Token::Else {
            tok.expect_token(&Token::Else)?;
        } else {
            return Ok(None);
        }

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
        if tok.peek_token()?.tok == Token::While {
            tok.expect_token(&Token::While)?;
        } else {
            return Ok(None);
        }

        Ok(Some(Self {
            cond: Expr::expect(tok)?,
            body: ExecScope::expect(tok)?,
        }))
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
        Ok(first_match!(
            tok, Self, UnaryStmt, IfStmt, WhileStmt, AssignStmt
        ))
    }
}

impl From<AssignStmt> for Stmt {
    fn from(value: AssignStmt) -> Self {
        Self::Assign(value)
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
