use crate::{
    ast::{
        first_match, first_match_chain, Alias, ArrayLit, ArrayType, AstNode, BoolLit, CallExpr,
        CharLit, DestructureExpr, EvalPath, ExecPath, ExecScope, FnType, NumLit, ParseResult,
        PrimitiveType, RefType, StrLit, StructDef,
    },
    tokenizer::Tokenizer,
};
use std::rc::Rc;

#[derive(Debug)]
pub enum RhsExpr {
    Scope(ExecScope),
    Alias(Alias),
    Destructure(DestructureExpr),
    EvalPath {
        rcv: Rc<RhsExpr>,
        path: Rc<EvalPath>,
    },
    ExecPath {
        rcv: Rc<RhsExpr>,
        path: ExecPath,
    },
    Call {
        rcv: Rc<RhsExpr>,
        arg: CallExpr,
    },

    BoolLit(BoolLit),
    NumLit(NumLit),
    CharLit(CharLit),
    StrLit(StrLit),
    ArrayLit(ArrayLit),

    PrimitiveType(PrimitiveType),
    RefType(Rc<RefType>),
    Struct(StructDef),
    ArrayType {
        typ: Rc<RhsExpr>,
        len: Rc<ArrayType>,
    },
    FnType {
        arg: Rc<RhsExpr>,
        ret: Rc<FnType>,
    },
}

impl AstNode for RhsExpr {
    fn parse(tok: &mut Tokenizer) -> ParseResult<Option<Self>> {
        let base = first_match!(
            tok,
            Self,
            ExecScope,
            Alias,
            DestructureExpr,
            BoolLit,
            NumLit,
            CharLit,
            StrLit,
            ArrayLit,
            PrimitiveType,
            RefType,
            StructDef
        );

        let mut expr = if let Some(expr) = base {
            expr
        } else {
            return Ok(None);
        };

        loop {
            match first_match_chain!(
                tok, Self, expr, EvalPath, ExecPath, ArrayType, CallExpr, FnType
            ) {
                (true, chain) => expr = chain,
                (false, chain) => break Ok(Some(chain)),
            }
        }
    }
}

impl From<ExecScope> for RhsExpr {
    fn from(value: ExecScope) -> Self {
        Self::Scope(value)
    }
}

impl From<Alias> for RhsExpr {
    fn from(value: Alias) -> Self {
        Self::Alias(value)
    }
}

impl From<DestructureExpr> for RhsExpr {
    fn from(value: DestructureExpr) -> Self {
        Self::Destructure(value)
    }
}

impl From<(Self, EvalPath)> for RhsExpr {
    fn from(value: (Self, EvalPath)) -> Self {
        Self::EvalPath {
            rcv: value.0.into(),
            path: value.1.into(),
        }
    }
}

impl From<(Self, ExecPath)> for RhsExpr {
    fn from(value: (Self, ExecPath)) -> Self {
        Self::ExecPath {
            rcv: value.0.into(),
            path: value.1,
        }
    }
}
impl From<(Self, CallExpr)> for RhsExpr {
    fn from(value: (Self, CallExpr)) -> Self {
        Self::Call {
            rcv: value.0.into(),
            arg: value.1,
        }
    }
}

impl From<BoolLit> for RhsExpr {
    fn from(value: BoolLit) -> Self {
        Self::BoolLit(value)
    }
}

impl From<NumLit> for RhsExpr {
    fn from(value: NumLit) -> Self {
        Self::NumLit(value)
    }
}

impl From<CharLit> for RhsExpr {
    fn from(value: CharLit) -> Self {
        Self::CharLit(value)
    }
}

impl From<StrLit> for RhsExpr {
    fn from(value: StrLit) -> Self {
        Self::StrLit(value)
    }
}

impl From<ArrayLit> for RhsExpr {
    fn from(value: ArrayLit) -> Self {
        Self::ArrayLit(value)
    }
}

impl From<PrimitiveType> for RhsExpr {
    fn from(value: PrimitiveType) -> Self {
        Self::PrimitiveType(value)
    }
}

impl From<RefType> for RhsExpr {
    fn from(value: RefType) -> Self {
        Self::RefType(value.into())
    }
}

impl From<StructDef> for RhsExpr {
    fn from(value: StructDef) -> Self {
        Self::Struct(value)
    }
}

impl From<(Self, ArrayType)> for RhsExpr {
    fn from(value: (Self, ArrayType)) -> Self {
        Self::ArrayType {
            typ: value.0.into(),
            len: value.1.into(),
        }
    }
}

impl From<(Self, FnType)> for RhsExpr {
    fn from(value: (Self, FnType)) -> Self {
        Self::FnType {
            arg: value.0.into(),
            ret: value.1.into(),
        }
    }
}
