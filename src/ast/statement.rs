use crate::{
    ast::{AliasDecl, AstNode, ExecScope, LhsExpr, ParseErr, ParseResult, RhsExpr},
    tokenizer::{Token, Tokenizer},
};
use std::rc::Rc;

#[derive(Debug)]
pub struct AssignStmt {
    pub lhs: LhsExpr,
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

#[derive(Debug)]
pub struct MatchStmt {
    pub val: RhsExpr,
    pub branches: Vec<(LhsExpr, RhsExpr)>,
}

#[derive(Debug)]
pub enum Stmt {
    AliasDecl(AliasDecl),
    Assign(AssignStmt),
    Expr(RhsExpr),
    Ctrl(CtrlStmt),
    If(IfStmt),
    While(WhileStmt),
    Match(MatchStmt),
}

impl AstNode for AssignStmt {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        let lhs = LhsExpr::parse(tok)?;
        tok.expect_token(&Token::Equal)?;
        let rhs = RhsExpr::parse(tok)?;

        Ok(Self { lhs, rhs })
    }
}

impl AstNode for IfStmt {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        tok.expect_token(&Token::If)?;
        let cond = RhsExpr::parse(tok)?;
        let body = ExecScope::parse(tok)?;
        let chain = if tok.peek_token()?.tok == Token::Else {
            Some(ElseStmt::parse(tok)?)
        } else {
            None
        };

        Ok(Self { cond, body, chain })
    }
}

impl AstNode for ElseStmt {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        tok.expect_token(&Token::Else)?;
        match tok.peek_token()?.tok {
            Token::If => Ok(Self::ElseIf(Rc::new(IfStmt::parse(tok)?))),
            Token::RCurlyBrace => Ok(Self::Else(ExecScope::parse(tok)?)),
            _ => {
                let token = tok.next_token()?;
                Err(ParseErr::Syntax {
                    pos: token.pos,
                    msg: "expected 'if' or '{'",
                })
            }
        }
    }
}

impl AstNode for CtrlStmt {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        let token = tok.next_token()?;
        let stmt = match token.tok {
            Token::Continue => Self::Continue,
            Token::Break => Self::Break,
            Token::Defer => Self::Defer(RhsExpr::parse(tok)?),
            Token::Return => Self::Return(RhsExpr::parse(tok)?),
            _ => {
                return Err(ParseErr::Syntax {
                    pos: token.pos,
                    msg: "expected 'continue', 'break', 'defer', or 'return'",
                });
            }
        };

        tok.expect_token(&Token::Semicolon)?;
        Ok(stmt)
    }
}

impl AstNode for WhileStmt {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        tok.expect_token(&Token::While)?;
        Ok(Self {
            cond: RhsExpr::parse(tok)?,
            body: ExecScope::parse(tok)?,
        })
    }
}

impl AstNode for MatchStmt {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        tok.expect_token(&Token::Match)?;
        let mut stmt = Self {
            val: RhsExpr::parse(tok)?,
            branches: Vec::new(),
        };

        tok.expect_token(&Token::LCurlyBrace)?;
        loop {
            if tok.peek_token()?.tok == Token::RCurlyBrace {
                break;
            }

            let lhs = LhsExpr::parse(tok)?;
            tok.expect_token(&Token::Arrow)?;
            let rhs = RhsExpr::parse(tok)?;
            stmt.branches.push((lhs, rhs));

            let token = tok.peek_token()?;
            match token.tok {
                Token::RCurlyBrace => break,
                Token::Comma => tok.expect_token(&Token::Comma)?,
                _ => {
                    return Err(ParseErr::Syntax {
                        pos: token.pos,
                        msg: "expected ',' or '}'",
                    })
                }
            };
        }

        tok.expect_token(&Token::RCurlyBrace)?;
        Ok(stmt)
    }
}

impl AstNode for Stmt {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        match tok.peek_token()?.tok {
            Token::If => Ok(Self::If(IfStmt::parse(tok)?)),
            Token::Continue | Token::Break | Token::Defer | Token::Return => {
                Ok(Self::Ctrl(CtrlStmt::parse(tok)?))
            }
            Token::While => Ok(Self::While(WhileStmt::parse(tok)?)),
            Token::Match => Ok(Self::Match(MatchStmt::parse(tok)?)),
            _ => todo!(),
        }
    }
}
