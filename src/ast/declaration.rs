use crate::{
    ast::{
        first_match, Alias, AstNode, EvalPath, EvalScope, ExecScope, ParseErr, ParseResult, RhsExpr,
    },
    tokenizer::{Token, Tokenizer},
};
use std::rc::Rc;

#[derive(Debug)]
pub enum ScopeAlias {
    Pkg,
    Mod,
}

#[derive(Debug)]
pub enum Access {
    Get,
    Set,
}

#[derive(Debug)]
pub struct VisExpr {
    pub scope: Option<ScopeAlias>,
    pub access: Option<Access>,
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
    pub lhs: RhsExpr,
    pub bounds: Option<RhsExpr>,
    pub rhs: RhsExpr,
}

#[derive(Debug)]
pub enum FnSlotDecl {
    Infer(RhsExpr),
    Full(RhsExpr, RhsExpr),
}

#[derive(Debug)]
pub struct FnSigDecl {
    pub rcv: Option<FnSlotDecl>,
    pub arg: FnSlotDecl,
    pub ret: FnSlotDecl,
}

#[derive(Debug)]
pub struct FnDecl {
    pub lhs: RhsExpr,
    pub sig: Option<FnSigDecl>,
    pub body: Option<ExecScope>,
}

#[derive(Debug)]
pub struct IfaceDecl {
    pub lhs: RhsExpr,
    pub bounds: Option<RhsExpr>,
    pub scope: EvalScope,
}

#[derive(Debug)]
pub struct ModDecl {
    pub lhs: RhsExpr,
    pub bounds: Option<RhsExpr>,
    pub scope: Option<EvalScope>,
}

#[derive(Debug)]
pub struct UseDecl {
    pub path: EvalPath,
}

#[derive(Debug)]
pub enum Annotation {
    Expr { tag: Alias, expr: RhsExpr },
    Decl { tag: Alias, decl: Decl },
}

#[derive(Debug)]
pub enum Decl {
    Alias(AliasDecl),
    Fn(FnDecl),
    Iface(IfaceDecl),
    Mod(ModDecl),
    Use(UseDecl),
    Annotation(Rc<Annotation>),
}

impl AstNode for ScopeAlias {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        let scope = match tok.peek_token()?.tok {
            Token::Pkg => Self::Pkg,
            Token::Mod => Self::Mod,
            _ => return Ok(None),
        };

        tok.next_token()?;
        Ok(Some(scope))
    }
}

impl AstNode for Access {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        let access = match tok.peek_token()?.tok {
            Token::Get => Self::Get,
            Token::Set => Self::Set,
            _ => return Ok(None),
        };

        tok.next_token()?;
        Ok(Some(access))
    }
}

impl AstNode for VisExpr {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        if tok.peek_token()?.tok == Token::Pub {
            return Ok(None);
        }

        tok.expect_token(&Token::Pub)?;
        let mut scope = None;
        let mut access = None;

        loop {
            if tok.peek_token()?.tok != Token::Colon {
                return Ok(Some(Self { scope, access }));
            }

            tok.expect_token(&Token::Colon)?;
            let token = tok.next_token()?;
            match token.tok {
                Token::Pkg | Token::Mod if scope.is_none() => {
                    scope = ScopeAlias::expect(tok)?.into()
                }
                Token::Get | Token::Set if access.is_none() => access = Access::expect(tok)?.into(),
                _ => {
                    return Err(ParseErr::Syntax {
                        pos: token.pos,
                        msg: "expected unique scope or access alias",
                    })
                }
            };
        }
    }
}

impl AstNode for AliasEval {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        let eval = match tok.peek_token()?.tok {
            Token::Let => Self::Let,
            Token::Var => Self::Var,
            Token::Const => Self::Const,
            Token::Type => Self::Type,
            _ => return Ok(None),
        };

        tok.next_token()?;
        Ok(Some(eval))
    }
}

impl AstNode for AliasDecl {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        let eval = if let Some(eval) = AliasEval::parse(tok)? {
            eval
        } else {
            return Ok(None);
        };

        let lhs = RhsExpr::expect(tok)?;
        let bounds = if tok.peek_token()?.tok == Token::Colon {
            tok.expect_token(&Token::Colon)?;
            Some(RhsExpr::expect(tok)?)
        } else {
            None
        };

        tok.expect_token(&Token::Equal)?;
        let rhs = RhsExpr::expect(tok)?;
        tok.expect_token(&Token::Semicolon)?;

        Ok(Some(Self {
            eval,
            lhs,
            bounds,
            rhs,
        }))
    }
}

impl AstNode for FnSlotDecl {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        let rcv = if let Some(rcv) = RhsExpr::parse(tok)? {
            rcv
        } else {
            return Ok(None);
        };

        Ok(Some(if let Some(lhs) = RhsExpr::parse(tok)? {
            Self::Full(rcv, lhs)
        } else {
            Self::Infer(rcv)
        }))
    }
}

impl AstNode for FnSigDecl {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        todo!()
    }
}

impl AstNode for FnDecl {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        if tok.peek_token()?.tok != Token::Fn {
            return Ok(None);
        }

        tok.expect_token(&Token::Fn)?;
        let lhs = RhsExpr::expect(tok)?;
        let sig = if tok.peek_token()?.tok != Token::LCurlyBrace {
            FnSigDecl::expect(tok)?.into()
        } else {
            None
        };

        let body = ExecScope::parse(tok)?;
        if body.is_none() {
            tok.expect_token(&Token::Semicolon)?;
        }

        Ok(Some(Self { lhs, sig, body }))
    }
}

impl AstNode for IfaceDecl {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        if tok.peek_token()?.tok != Token::Iface {
            return Ok(None);
        }

        tok.expect_token(&Token::Iface)?;
        let lhs = RhsExpr::expect(tok)?;
        let bounds = if tok.peek_token()?.tok == Token::Colon {
            tok.expect_token(&Token::Colon)?;
            RhsExpr::expect(tok)?.into()
        } else {
            None
        };

        let scope = EvalScope::expect(tok)?;
        Ok(Some(Self { lhs, bounds, scope }))
    }
}

impl AstNode for ModDecl {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        if tok.peek_token()?.tok != Token::Mod {
            return Ok(None);
        }

        tok.expect_token(&Token::Mod)?;
        let lhs = RhsExpr::expect(tok)?;
        let bounds = if tok.peek_token()?.tok == Token::Colon {
            tok.expect_token(&Token::Colon)?;
            RhsExpr::expect(tok)?.into()
        } else {
            None
        };

        let scope = EvalScope::parse(tok)?;
        if scope.is_none() {
            tok.expect_token(&Token::Semicolon)?;
        };

        Ok(Some(Self { lhs, bounds, scope }))
    }
}

impl AstNode for UseDecl {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        if tok.peek_token()?.tok == Token::Use {
            tok.expect_token(&Token::Use)?;
        } else {
            return Ok(None);
        }

        Ok(Some(Self {
            path: EvalPath::expect(tok)?,
        }))
    }
}

impl AstNode for Annotation {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        if tok.peek_token()?.tok == Token::Bang {
            tok.expect_token(&Token::Bang)?;
        } else {
            return Ok(None);
        }

        tok.expect_token(&Token::LSqrBrace)?;
        let tag = Alias::expect(tok)?;

        let token = tok.peek_token()?;
        match token.tok {
            Token::RSqrBrace => {
                tok.expect_token(&Token::RSqrBrace)?;
                let decl = Decl::expect(tok)?;
                Ok(Some(Self::Decl { tag, decl }))
            }
            Token::Equal => {
                tok.expect_token(&Token::Equal)?;
                let expr = RhsExpr::expect(tok)?;
                tok.expect_token(&Token::RSqrBrace)?;
                Ok(Some(Self::Expr { tag, expr }))
            }
            _ => Err(ParseErr::Syntax {
                pos: token.pos,
                msg: "expected '=' or ']'",
            }),
        }
    }
}

impl AstNode for Decl {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        Ok(first_match!(
            tok, Self, AliasDecl, FnDecl, IfaceDecl, ModDecl, Annotation
        ))
    }
}

impl From<AliasDecl> for Decl {
    fn from(value: AliasDecl) -> Self {
        Self::Alias(value)
    }
}

impl From<FnDecl> for Decl {
    fn from(value: FnDecl) -> Self {
        Self::Fn(value)
    }
}

impl From<IfaceDecl> for Decl {
    fn from(value: IfaceDecl) -> Self {
        Self::Iface(value)
    }
}

impl From<ModDecl> for Decl {
    fn from(value: ModDecl) -> Self {
        Self::Mod(value)
    }
}

impl From<Annotation> for Decl {
    fn from(value: Annotation) -> Self {
        Self::Annotation(value.into())
    }
}
