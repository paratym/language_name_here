use bimap::BiMap;
use lazy_static::lazy_static;
use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt::{self, Display},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LexToken {
    Equal,
    As,
    Let,
    Mut,
    Const,
    Static,
    Asterisk,
    Ampersand,
    Pub,
    Mod,
    Pkg,
    Use,
    DoubleColon,
    Type,
    Bool,
    True,
    False,
    Char,
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
    LParen,
    Rparen,
    Colon,
    Comma,
    Dot,
    Hyphen,
    Underscore,
    Get,
    Set,
    Union,
    LSqrBrace,
    RSqrBrace,
    Elipsis,
    Semicolon,
    Fn,
    Arrow,
    LCurlyBrace,
    RCurlyBrace,
    Return,
    Defer,
    If,
    Else,
    While,
    Break,
    Continue,
    Match,
    Interface,
    Impl,
    Bang,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Char(char),
    Str(Box<str>),
    Int(i64),
    Uint(u64),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Lex(LexToken),
    Lit(Literal),
    Ident(Box<str>),
    Comment(Box<str>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SrcPosition {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SrcToken {
    pub tok: Token,
    pub pos: SrcPosition,
}

lazy_static! {
    pub static ref LEX_TOKENS: BiMap<LexToken, &'static str> = {
        let mut map = BiMap::new();
        map.insert(LexToken::Equal, "=");
        map.insert(LexToken::As, "as");
        map.insert(LexToken::Let, "let");
        map.insert(LexToken::Mut, "mut");
        map.insert(LexToken::Const, "const");
        map.insert(LexToken::Static, "static");
        map.insert(LexToken::Asterisk, "*");
        map.insert(LexToken::Ampersand, "&");
        map.insert(LexToken::Pub, "pub");
        map.insert(LexToken::Mod, "mod");
        map.insert(LexToken::Pkg, "pkg");
        map.insert(LexToken::Use, "use");
        map.insert(LexToken::DoubleColon, "::");
        map.insert(LexToken::Type, "type");
        map.insert(LexToken::Bool, "bool");
        map.insert(LexToken::True, "true");
        map.insert(LexToken::False, "false");
        map.insert(LexToken::Char, "char");
        map.insert(LexToken::Isize, "isize");
        map.insert(LexToken::I8, "i8");
        map.insert(LexToken::I16, "i16");
        map.insert(LexToken::I24, "i24");
        map.insert(LexToken::I32, "i32");
        map.insert(LexToken::I64, "i64");
        map.insert(LexToken::Usize, "usize");
        map.insert(LexToken::U8, "u8");
        map.insert(LexToken::U16, "u16");
        map.insert(LexToken::U24, "u24");
        map.insert(LexToken::U32, "u32");
        map.insert(LexToken::U64, "u64");
        map.insert(LexToken::F32, "f32");
        map.insert(LexToken::F64, "f64");
        map.insert(LexToken::LParen, "(");
        map.insert(LexToken::Rparen, ")");
        map.insert(LexToken::Colon, ":");
        map.insert(LexToken::Comma, ",");
        map.insert(LexToken::Dot, ".");
        map.insert(LexToken::Hyphen, "-");
        map.insert(LexToken::Underscore, "_");
        map.insert(LexToken::Get, "get");
        map.insert(LexToken::Set, "set");
        map.insert(LexToken::Union, "union");
        map.insert(LexToken::LSqrBrace, "[");
        map.insert(LexToken::RSqrBrace, "]");
        map.insert(LexToken::Elipsis, "...");
        map.insert(LexToken::Semicolon, ";");
        map.insert(LexToken::Fn, "fn");
        map.insert(LexToken::Arrow, "->");
        map.insert(LexToken::LCurlyBrace, "{");
        map.insert(LexToken::RCurlyBrace, "}");
        map.insert(LexToken::Return, "return");
        map.insert(LexToken::Defer, "defer");
        map.insert(LexToken::If, "if");
        map.insert(LexToken::Else, "else");
        map.insert(LexToken::While, "while");
        map.insert(LexToken::Break, "break");
        map.insert(LexToken::Continue, "continue");
        map.insert(LexToken::Match, "match");
        map.insert(LexToken::Interface, "interface");
        map.insert(LexToken::Impl, "impl");
        map.insert(LexToken::Bang, "!");
        map
    };
    pub static ref MAX_LEX_TOKEN_LEN: usize = LEX_TOKENS
        .right_values()
        .reduce(|a, b| if b.len() > a.len() { b } else { a })
        .map(|t| t.len())
        .expect("missing tokens");
    pub static ref LIT_PRFIXES: HashSet<char> = {
        let mut set = HashSet::new();
        set.insert('"');
        set.insert('\'');
        set.insert('+');
        set.insert('-');
        set.insert('0');
        set.insert('1');
        set.insert('2');
        set.insert('3');
        set.insert('4');
        set.insert('5');
        set.insert('6');
        set.insert('7');
        set.insert('8');
        set.insert('9');
        set
    };
}

impl From<LexToken> for Token {
    fn from(lex: LexToken) -> Self {
        Self::Lex(lex)
    }
}

impl From<Literal> for Token {
    fn from(lit: Literal) -> Self {
        Self::Lit(lit)
    }
}

impl Display for LexToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(
            LEX_TOKENS
                .get_by_left(self)
                .expect("missing token definition"),
        )
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Char(value) => write!(f, "'{}'", value),
            Literal::Str(value) => write!(f, "\"{}\"", value),
            Literal::Int(value) => value.fmt(f),
            Literal::Uint(value) => value.fmt(f),
            Literal::Float(value) => value.fmt(f),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Lex(tok) => tok.fmt(f),
            Token::Lit(tok) => tok.fmt(f),
            Token::Ident(tok) => tok.fmt(f),
            Token::Comment(tok) => tok.fmt(f),
        }
    }
}
