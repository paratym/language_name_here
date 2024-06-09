use crate::{
    ast::{
        ArrayLit, ArrayType, AstNode, CharLit, CompoundLit, CompoundType, ExecScope, FnType,
        NumLit, ParseErr, ParseResult, RefType, StrLit,
    },
    tokenizer::{Token, Tokenizer},
};
use std::rc::Rc;

#[derive(Debug)]
pub enum VisScope {
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
    pub vis: Option<VisScope>,
    pub access: Option<Access>,
}

#[derive(Debug)]
pub struct Alias {
    pub alias: Rc<str>,
}

#[derive(Debug)]
pub enum ScopeAlias {
    Pkg,
    Std,
    Ext,
    Local(RhsExpr),
}

#[derive(Debug)]
pub struct PathExpr {
    pub scope: ScopeAlias,
    pub path: Vec<Alias>,
}

#[derive(Debug)]
pub enum Arg {
    Expr(RhsExpr),
    Compound(CompoundLit),
}

#[derive(Debug)]
pub enum DotExpr {
    Ref,
    RefMut,
    Deref,
    Field(Alias),
    Index(RhsExpr),
    Call(Arg),
}

#[derive(Debug)]
pub struct ConstructExpr {
    pub typ: RhsExpr,
    pub arg: Arg,
}

#[derive(Debug)]
pub struct SpreadExpr {
    pub val: RhsExpr,
}

#[derive(Debug)]
pub struct DestructureExpr {
    pub fields: Vec<LhsExpr>,
}

#[derive(Debug)]
pub enum RhsExpr {
    Alias(Alias),
    Construct(Rc<ConstructExpr>),
    Dot(Rc<DotExpr>),
    Scope(ExecScope),

    NumLit(NumLit),
    CharLit(CharLit),
    StrLit(StrLit),
    ArrayLit(ArrayLit),
    CompoundLit(CompoundLit),

    RefType(Rc<RefType>),
    ArrayType(Rc<ArrayType>),
    CompoundType(CompoundType),
    FnType(Rc<FnType>),
}

#[derive(Debug)]
pub enum LhsExpr {
    Alias(Alias),
    Dot(DotExpr),
    Destructure(DestructureExpr),
}

impl AstNode for VisScope {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        let token = tok.next_token()?;
        match token.tok {
            Token::Pkg => Ok(Self::Pkg),
            Token::Mod => Ok(Self::Mod),
            _ => Err(ParseErr::Syntax {
                pos: token.pos,
                msg: "expected 'pkg' or 'mod'",
            }),
        }
    }
}

impl AstNode for Access {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        let token = tok.next_token()?;
        match token.tok {
            Token::Get => Ok(Self::Get),
            Token::Set => Ok(Self::Set),
            _ => Err(ParseErr::Syntax {
                pos: token.pos,
                msg: "expected 'get' or 'set'",
            }),
        }
    }
}

impl AstNode for VisExpr {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        tok.expect_token(&Token::Pub)?;
        let mut expr = Self {
            vis: None,
            access: None,
        };

        loop {
            if tok.peek_token()?.tok != Token::Colon {
                return Ok(expr);
            }

            tok.expect_token(&Token::Colon)?;
            let token = tok.next_token()?;
            match token.tok {
                Token::Pkg | Token::Mod if expr.vis.is_none() => {
                    expr.vis = Some(VisScope::parse(tok)?)
                }
                Token::Get | Token::Set if expr.access.is_none() => {
                    expr.access = Some(Access::parse(tok)?)
                }
                _ => {
                    return Err(ParseErr::Syntax {
                        pos: token.pos,
                        msg: "expected unique 'pkg', 'mod', 'get', or 'set'",
                    })
                }
            };
        }
    }
}

impl AstNode for Alias {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        let token = tok.next_token()?;
        let alias = match token.tok {
            Token::Alias(str) => str.into(),
            _ => {
                return Err(ParseErr::Syntax {
                    pos: token.pos,
                    msg: "expected an alias",
                })
            }
        };

        Ok(Self { alias })
    }
}

impl AstNode for ScopeAlias {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        let scope = match tok.peek_token()?.tok {
            Token::Pkg => Self::Pkg,
            Token::Std => Self::Std,
            Token::Ext => Self::Ext,
            _ => return Ok(Self::Local(RhsExpr::parse(tok)?)),
        };

        tok.next_token()?;
        Ok(scope)
    }
}

impl AstNode for PathExpr {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        let scope = ScopeAlias::parse(tok)?;
        let mut path = Vec::new();

        loop {
            if tok.peek_token()?.tok == Token::DoubleColon {
                break;
            }

            path.push(Alias::parse(tok)?);
        }

        Ok(Self { scope, path })
    }
}

impl AstNode for Arg {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for DotExpr {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for ConstructExpr {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for SpreadExpr {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        tok.expect_token(&Token::Elipsis)?;

        Ok(Self {
            val: RhsExpr::parse(tok)?,
        })
    }
}

impl AstNode for DestructureExpr {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        tok.expect_token(&Token::LParen)?;
        let mut expr = Self { fields: Vec::new() };

        loop {
            if tok.peek_token()?.tok == Token::Rparen {
                break;
            }

            expr.fields.push(LhsExpr::parse(tok)?);

            let token = tok.peek_token()?;
            match token.tok {
                Token::Rparen => break,
                Token::Comma => tok.expect_token(&Token::Comma)?,
                _ => {
                    return Err(ParseErr::Syntax {
                        pos: token.pos,
                        msg: "expected ',' or ')'",
                    })
                }
            };
        }

        tok.expect_token(&Token::Rparen)?;
        Ok(expr)
    }
}

impl AstNode for RhsExpr {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}

impl AstNode for LhsExpr {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Self> {
        todo!()
    }
}
