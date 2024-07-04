use crate::{
    ast::{first_match, AliasDecl, AstNode, ExecScope, ParseErr, ParseResult, RhsExpr},
    tokenizer::{Token, Tokenizer},
};
use std::rc::Rc;

#[derive(Debug)]
pub struct AssignStmt {
    pub lhs: RhsExpr,
    pub rhs: RhsExpr,
}

#[derive(Debug)]
pub struct IfStmt {
    pub cond: RhsExpr,
    pub body: ExecScope,
    pub chain: Option<ElseStmt>,
}

#[derive(Debug)]
pub enum ElseStmt {
    ElseIf(Rc<IfStmt>),
    Else(ExecScope),
}

#[derive(Debug)]
pub enum CtrlStmt {
    Continue,
    Break,
    Defer(RhsExpr),
    Return(RhsExpr),
}

#[derive(Debug)]
pub struct WhileStmt {
    pub cond: RhsExpr,
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
    Assign(AssignStmt),
    Expr(RhsExpr),
    Ctrl(CtrlStmt),
    If(IfStmt),
    While(WhileStmt),
    // Match(MatchStmt),
}

impl AstNode for AssignStmt {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        let lhs = if let Some(expr) = RhsExpr::parse(tok)? {
            expr
        } else {
            return Ok(None);
        };

        tok.expect_token(&Token::Equal)?;
        let rhs = RhsExpr::expect(tok)?;
        tok.expect_token(&Token::Semicolon)?;

        Ok(Some(Self { lhs, rhs }))
    }
}

impl AstNode for IfStmt {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        if tok.peek_token()?.tok == Token::If {
            tok.expect_token(&Token::If)?;
        } else {
            return Ok(None);
        }

        let cond = RhsExpr::expect(tok)?;
        let body = ExecScope::expect(tok)?;
        let chain = ElseStmt::parse(tok)?;

        Ok(Some(Self { cond, body, chain }))
    }
}

impl AstNode for ElseStmt {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
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
                msg: "expected an if statement or an execution scope",
            });
        }))
    }
}

impl AstNode for CtrlStmt {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        let stmt = match tok.peek_token()?.tok {
            Token::Continue => {
                tok.expect_token(&Token::Continue)?;
                Self::Continue
            }
            Token::Break => {
                tok.expect_token(&Token::Break)?;
                Self::Break
            }
            Token::Defer => {
                tok.expect_token(&Token::Defer)?;
                Self::Defer(RhsExpr::expect(tok)?)
            }
            Token::Return => {
                tok.expect_token(&Token::Return)?;
                Self::Return(RhsExpr::expect(tok)?)
            }
            _ => return Ok(None),
        };

        tok.expect_token(&Token::Semicolon)?;
        Ok(Some(stmt))
    }
}

impl AstNode for WhileStmt {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        if tok.peek_token()?.tok == Token::While {
            tok.expect_token(&Token::While)?;
        } else {
            return Ok(None);
        }

        Ok(Some(Self {
            cond: RhsExpr::expect(tok)?,
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
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        Ok(first_match!(
            tok, Self, AssignStmt, CtrlStmt, IfStmt, WhileStmt
        ))
    }
}

impl From<AssignStmt> for Stmt {
    fn from(value: AssignStmt) -> Self {
        Self::Assign(value)
    }
}

impl From<CtrlStmt> for Stmt {
    fn from(value: CtrlStmt) -> Self {
        Self::Ctrl(value)
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
