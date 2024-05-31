use crate::tokenizer::Token;
use std::rc::Rc;

pub struct ConstScope {
    pub decls: Vec<Decl>,
}

pub struct ExecScope {
    pub stmts: Vec<Stmt>,
}

pub struct ModDecl {
    pub alias: Ident,
    pub scope: ConstScope,
}

pub struct UseDecl {
    pub path: PathExpr,
}

pub struct ValueDecl {
    pub eval: Option<Token>,
    pub alias: Ident,
    pub assert: Option<TypeExpr>,
    pub rhs: Expr,
}

pub struct TypeDecl {
    pub alias: Ident,
    pub rhs: Option<TypeExpr>,
}

pub enum FnSlot {
    Type(TypeExpr),
    Destructure(DestructureExpr),
}

pub struct FnDecl {
    pub alias: Ident,
    pub rcv: Option<TypeExpr>,
    pub arg: FnSlot,
    pub ret: TypeExpr,
    pub body: Option<ExecScope>,
}

pub struct InterfaceDecl {
    pub alias: Ident,
    pub decls: Vec<FnDecl>,
}

pub struct ImplDecl {
    pub alias: Ident,
    pub reciever: TypeExpr,
    pub decls: Vec<FnDecl>,
}

pub enum Decl {
    Mod(ModDecl),
    Use(UseDecl),
    Value(ValueDecl),
    Type(TypeDecl),
    Fn(FnDecl),
    Interface(InterfaceDecl),
}

pub enum Assignable {
    Ident(Ident),
    Access(AccessExpr),
}

pub struct AssignStmt {
    pub lhs: Assignable,
    pub rhs: Expr,
}

pub struct CallStmt {
    pub proc: Expr,
    pub arg: Expr,
}

pub enum CtrlTok {
    Return,
    Defer,
    Break,
    Continue,
}

pub struct CtrlStmt {
    pub ctrl: CtrlTok,
    pub rhs: ValueExpr,
}

pub enum IfChain {
    If(Rc<IfStmt>),
    Else(ExecScope),
}

pub struct IfStmt {
    pub pred: Option<ValueDecl>,
    pub cond: ValueExpr,
    pub body: ExecScope,
    pub chain: Option<IfChain>,
}

pub struct WhileStmt {
    pub pred: Option<ValueDecl>,
    pub cond: ValueExpr,
    pub body: ExecScope,
}

pub struct MatchStmt {
    pub pred: Option<ValueDecl>,
    pub value: ValueExpr,
    pub branches: Vec<(PatternExpr, ExecScope)>,
}

pub enum Stmt {
    ExecScope(ExecScope),
    UseDecl(UseDecl),
    ValueDecl(ValueDecl),
    TypeDecl(TypeDecl),

    Assign(AssignStmt),
    Call(CallStmt),
    Ctrl(CtrlStmt),
    If(IfStmt),
    While(WhileStmt),
    Match(MatchStmt),
}

pub struct Ident {
    pub val: Box<str>,
}

pub struct PathExpr {
    pub segments: Vec<Ident>,
}

pub struct NumLit {
    pub negative: bool,
    pub typ: NumType,
    pub whole: u64,
    pub frac: u64,
}

pub struct CharLit {
    pub val: char,
}

pub struct StrLit {
    pub val: Box<str>,
}

pub enum CompoundEntry {
    Assign(AssignStmt),
    Spread(SpreadExpr),
}

pub struct CompoundLit {
    pub entries: Vec<CompoundEntry>,
}

pub enum ArrayEntry {
    Value(ValueExpr),
    Spread(SpreadExpr),
}

pub struct ArrayLit {
    pub entries: ArrayEntry,
}

pub enum NumType {
    Isize,
    I8,
    I16,
    I24,
    I32,
    I64,
    Usize,
    U8,
    U16,
    U24,
    U32,
    U64,
    F32,
    F64,
}

pub struct RefType {
    pub mutable: bool,
    pub deref: TypeExpr,
}

pub struct CompoundField {
    pub access: (),
    pub alias: Ident,
    pub typ: TypeExpr,
    pub default: ValueExpr,
}

pub struct CompoundType {}

pub struct ArrayType {}
pub struct FnType {}
pub struct ConstructExpr {}
pub struct DestructureExpr {}
pub struct AccessExpr {}
pub struct IndexExpr {}
pub struct PatternExpr {}
pub struct SpreadExpr {}

pub enum Expr {
    Scope(ExecScope),
    Call(Rc<CallStmt>),

    Ident(Ident),
    Path(PatternExpr),
    CompoundLit(CompoundLit),
    ArrayLit(ArrayLit),
    RefType(RefType),
    CompoundType(CompoundType),
    ArrayType(ArrayType),
    FnType(FnType),
    Construct(ConstructExpr),
    Destructure(DestructureExpr),
    Access(AccessExpr),
    Index(IndexExpr),
    Pattern(PatternExpr),
    Spread(SpreadExpr),
}

pub enum ValueExpr {
    Scope(ExecScope),
    Ident(Ident),
    Path(PathExpr),
    CompoundLit(Rc<CompoundLit>),
    ArrayLit(Rc<ArrayLit>),
    Access(AccessExpr),
    Index(IndexExpr),
}

pub enum TypeExpr {
    Ident(Ident),
    Compound(CompoundType),
    Array(ArrayType),
    Fn(FnType),
}

pub enum Lhs {
    Ident(Ident),
    Destructure(DestructureExpr),
}
