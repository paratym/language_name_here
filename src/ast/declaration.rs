use crate::{
    ast::{
        Alias, AstNode, ConstScope, ExecScope, LhsExpr, ParseErr, ParseResult, RhsExpr, StructType,
    },
    tokenizer::{Token, Tokenizer},
};
use std::rc::Rc;

#[derive(Debug)]
pub enum AliasEval {
    Let,
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
pub struct FnSignature {
    pub alias: Alias,
    pub rcv: Option<RhsExpr>,
    pub arg: RhsExpr,
    pub ret: RhsExpr,
}

#[derive(Debug)]
pub enum FnSlotDecl {
    Named(RhsExpr, Option<LhsExpr>),
    Inline(StructType),
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
    pub scope: ConstScope,
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
        todo!()
    }
}

impl AstNode for AliasDecl {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        // tok.expect_token(&Token::Let);
        // let token = tok.peek_token()?.clone().tok;
        // let eval = if matches!(token, Token::Mut | Token::Const | Token::Type) {
        //     tok.expect_token(&token);
        //     Some(token)
        // } else {
        //     None
        // };
        //
        // if let Some(token) = eval.clone() {
        //     tok.expect_token(&token)?;
        // }
        //
        // let alias = Alias::parse(tok)?;
        // let bounds = if tok.peek_token()?.tok == Token::Colon {
        //     tok.expect_token(&Token::Colon);
        //     todo!()
        // } else {
        //     None
        // };
        //
        // tok.expect_token(&Token::Equal)?;
        // let rhs = Expr::parse(tok)?;
        //
        // Ok(Self {
        //     eval,
        //     alias,
        //     bounds,
        //     rhs,
        todo!()
    }
}

impl AstNode for FnSignature {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
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
        todo!()
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
            Token::Let => Self::Alias(AliasDecl::parse(tok)?),
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
