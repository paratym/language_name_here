use bimap::BiMap;
use lazy_static::lazy_static;
use std::{
    fmt::{self, Display},
    ops::{Add, AddAssign},
    sync::Arc,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Comment(Arc<str>),
    Alias(Arc<str>),
    Equal,
    As,
    Let,
    Var,
    Const,
    Asterisk,
    Ampersand,
    Caret,
    Pub,
    Mod,
    Pkg,
    Std,
    Ext,
    Use,
    DoubleColon,
    Type,
    Infer,
    Bool,
    True,
    False,
    Char,
    CharLit(Arc<str>),
    Str,
    StrLit(Arc<str>),
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
    NumLit(Arc<str>),
    LParen,
    Rparen,
    Union,
    Key,
    Colon,
    Comma,
    Dot,
    Hyphen,
    Underscore,
    Get,
    Set,
    LSqrBrace,
    RSqrBrace,
    Elipsis,
    Semicolon,
    Fn,
    Arrow,
    LCurlyBrace,
    RCurlyBrace,
    // Self,
    Arg,
    Return,
    Defer,
    If,
    Else,
    While,
    Break,
    Continue,
    Match,
    Iface,
    Impl,
    Bang,
    LAngleBrace,
    RAngleBrace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
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
    pub static ref LEX_TOKENS: BiMap<Token, &'static str> = {
        let mut map = BiMap::new();
        map.insert(Token::Equal, "=");
        map.insert(Token::As, "as");
        map.insert(Token::Let, "let");
        map.insert(Token::Var, "var");
        map.insert(Token::Const, "const");
        map.insert(Token::Asterisk, "*");
        map.insert(Token::Ampersand, "&");
        map.insert(Token::Caret, "^");
        map.insert(Token::Pub, "pub");
        map.insert(Token::Mod, "mod");
        map.insert(Token::Pkg, "pkg");
        map.insert(Token::Std, "std");
        map.insert(Token::Ext, "ext");
        map.insert(Token::Use, "use");
        map.insert(Token::DoubleColon, "::");
        map.insert(Token::Type, "type");
        map.insert(Token::Infer, "infer");
        map.insert(Token::Bool, "bool");
        map.insert(Token::True, "true");
        map.insert(Token::False, "false");
        map.insert(Token::Char, "char");
        map.insert(Token::Isize, "isize");
        map.insert(Token::I8, "i8");
        map.insert(Token::I16, "i16");
        map.insert(Token::I24, "i24");
        map.insert(Token::I32, "i32");
        map.insert(Token::I64, "i64");
        map.insert(Token::Usize, "usize");
        map.insert(Token::U8, "u8");
        map.insert(Token::U16, "u16");
        map.insert(Token::U24, "u24");
        map.insert(Token::U32, "u32");
        map.insert(Token::U64, "u64");
        map.insert(Token::F32, "f32");
        map.insert(Token::F64, "f64");
        map.insert(Token::LParen, "(");
        map.insert(Token::Rparen, ")");
        map.insert(Token::Union, "union");
        map.insert(Token::Key, "key");
        map.insert(Token::Colon, ":");
        map.insert(Token::Comma, ",");
        map.insert(Token::Dot, ".");
        map.insert(Token::Hyphen, "-");
        map.insert(Token::Underscore, "_");
        map.insert(Token::Get, "get");
        map.insert(Token::Set, "set");
        map.insert(Token::LSqrBrace, "[");
        map.insert(Token::RSqrBrace, "]");
        map.insert(Token::Elipsis, "...");
        map.insert(Token::Semicolon, ";");
        map.insert(Token::Fn, "fn");
        map.insert(Token::Arrow, "->");
        map.insert(Token::LCurlyBrace, "{");
        map.insert(Token::RCurlyBrace, "}");
        // map.insert(Token::Arg, "arg");
        map.insert(Token::Return, "return");
        map.insert(Token::Defer, "defer");
        map.insert(Token::If, "if");
        map.insert(Token::Else, "else");
        map.insert(Token::While, "while");
        map.insert(Token::Break, "break");
        map.insert(Token::Continue, "continue");
        map.insert(Token::Match, "match");
        map.insert(Token::Iface, "iface");
        map.insert(Token::Impl, "impl");
        map.insert(Token::Bang, "!");
        map.insert(Token::LAngleBrace, "<");
        map.insert(Token::RAngleBrace, ">");
        map
    };
    pub static ref MAX_LEX_TOKEN_LEN: usize =
        LEX_TOKENS.right_values().map(|t| t.len()).max().unwrap();
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Comment(val) => write!(f, "# {}", val),
            Self::Alias(val) => f.write_str(val),
            Self::CharLit(val) => write!(f, "'{}'", val),
            Self::StrLit(val) => write!(f, "\"{}\"", val),
            Self::NumLit(val) => f.write_str(val),
            _ => f.write_str(LEX_TOKENS.get_by_left(self).ok_or(fmt::Error)?),
        }
    }
}

impl AddAssign for SrcPosition {
    fn add_assign(&mut self, rhs: Self) {
        self.line += rhs.line;
        if rhs.line > 0 {
            self.column = rhs.column;
        } else {
            self.column += rhs.column;
        }
    }
}
