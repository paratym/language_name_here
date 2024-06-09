use crate::{
    ast::{
        Alias, AstNode, CompoundType, ConstScope, ExecScope, LhsExpr, ParseErr, ParseResult,
        RhsExpr,
    },
    tokenizer::{Token, Tokenizer},
};
use std::rc::Rc;

#[derive(Debug)]
pub enum AliasEval {
    Let,
    Mut,
    Const,
    Type,
}

#[derive(Debug)]
pub struct AliasDecl {
    pub eval: AliasEval,
    pub lhs: LhsExpr,
    pub bounds: Option<RhsExpr>,
    pub rhs: RhsExpr,
}

#[derive(Debug)]
pub enum FnSlotDecl {
    Named(RhsExpr, Option<LhsExpr>),
    Inline(CompoundType),
}

#[derive(Debug)]
pub struct FnDecl {
    pub alias: Alias,
    pub rcv: Option<FnSlotDecl>,
    pub arg: FnSlotDecl,
    pub ret: FnSlotDecl,
    pub body: Option<ExecScope>,
}

#[derive(Debug)]
pub struct IfaceDecl {
    pub alias: Alias,
    pub decls: Vec<Decl>,
}

#[derive(Debug)]
pub struct ModDecl {
    pub alias: Alias,
    pub scope: Option<ConstScope>,
}

#[derive(Debug)]
pub struct UseDecl {}

#[derive(Debug)]
pub enum Annotation {
    Expr { tag: Alias, expr: RhsExpr },
    Decl { tag: Alias, decl: Decl },
}

#[derive(Debug)]
pub enum Decl {
    Alias(AliasDecl),
    Fn(FnDecl),
    Interface(IfaceDecl),
    Mod(ModDecl),
    Use(UseDecl),
    Annotation(Rc<Annotation>),
}

impl AstNode for AliasEval {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        let token = tok.next_token()?;
        Ok(match token.tok {
            Token::Let => Self::Let,
            Token::Mut => Self::Mut,
            Token::Const => Self::Const,
            Token::Type => Self::Type,
            _ => {
                return Err(ParseErr::Syntax {
                    pos: token.pos,
                    msg: "expected 'let', 'mut', 'const', or 'type'",
                })
            }
        })
    }
}

impl AstNode for AliasDecl {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        let eval = AliasEval::parse(tok)?;
        let lhs = LhsExpr::parse(tok)?;

        let bounds = if tok.peek_token()?.tok == Token::Colon {
            tok.expect_token(&Token::Colon)?;
            Some(RhsExpr::parse(tok)?)
        } else {
            None
        };

        tok.expect_token(&Token::Equal)?;
        let rhs = RhsExpr::parse(tok)?;
        tok.expect_token(&Token::Semicolon)?;

        Ok(Self {
            eval,
            lhs,
            bounds,
            rhs,
        })
    }
}

impl AstNode for FnSlotDecl {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for FnDecl {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for IfaceDecl {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for ModDecl {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        tok.expect_token(&Token::Mod)?;
        let alias = Alias::parse(tok)?;
        let scope = if tok.peek_token()?.tok == Token::RCurlyBrace {
            tok.expect_token(&Token::RCurlyBrace)?;
            Some(ConstScope::parse(tok)?)
        } else {
            tok.expect_token(&Token::Semicolon)?;
            None
        };

        Ok(Self { alias, scope })
    }
}

impl AstNode for UseDecl {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for Annotation {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        tok.expect_token(&Token::Bang)?;
        tok.expect_token(&Token::LSqrBrace)?;

        let tag = Alias::parse(tok)?;
        let token = tok.peek_token()?;
        if token.tok == Token::RSqrBrace {
            tok.expect_token(&Token::RSqrBrace)?;
            let decl = Decl::parse(tok)?;
            Ok(Self::Decl { tag, decl })
        } else if token.tok == Token::Equal {
            tok.expect_token(&Token::Equal)?;
            let expr = RhsExpr::parse(tok)?;
            tok.expect_token(&Token::RSqrBrace)?;
            Ok(Self::Expr { tag, expr })
        } else {
            Err(ParseErr::Syntax {
                pos: token.pos,
                msg: "expected '=' or ']'",
            })
        }
    }
}

impl AstNode for Decl {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        Ok(match tok.peek_token()?.tok {
            Token::Let | Token::Mut | Token::Const | Token::Type => {
                Self::Alias(AliasDecl::parse(tok)?)
            }
            Token::Fn => Self::Fn(FnDecl::parse(tok)?),
            Token::Interface => Self::Interface(IfaceDecl::parse(tok)?),
            Token::Mod => Self::Mod(ModDecl::parse(tok)?),
            Token::Bang => Self::Annotation(Rc::new(Annotation::parse(tok)?)),
            _ => {
                return Err(ParseErr::Syntax {
                    pos: *tok.pos(),
                    msg: "expected declaration",
                })
            }
        })
    }
}
