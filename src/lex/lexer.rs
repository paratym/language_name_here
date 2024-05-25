use crate::lex::*;
use std::io::{self, BufRead};

pub struct Lexer {
    reader: Box<dyn BufRead>,
    pos: SrcPosition,
}

impl Lexer {
    pub fn new(reader: Box<dyn BufRead>) -> Self {
        Self {
            reader,
            pos: SrcPosition { line: 0, column: 0 },
        }
    }

    fn peek_buf(&mut self) -> LexResult<&str> {
        let buf = self.reader.fill_buf()?;
        let mut end_i = buf.len();
        loop {
            match std::str::from_utf8(&buf[..end_i]) {
                Ok(s) => return Ok(s),
                Err(e) => {
                    end_i = e.valid_up_to();
                    if end_i == 0 {
                        return Err(LexErr::Syntax {
                            pos: self.pos,
                            msg: "invalid utf8 character",
                        });
                    }
                }
            }
        }
    }

    pub fn next_token(&mut self) -> LexResult<Option<SrcToken>> {
        let chr: char;
        loop {
            let buf = self.peek_buf()?;
            let buf_len = buf.len();
            if buf_len == 0 {
                return Ok(None);
            }

            let (i, c) = buf
                .char_indices()
                .find(|c| !c.1.is_ascii_whitespace())
                .unwrap_or((buf_len, '\0'));

            self.reader.consume(i);
            if i < buf_len {
                chr = c;
                break;
            }
        }

        let pos = self.pos;
        if chr == '#' {
            if let Some(comment) = self.read_comment()? {
                return Ok(Some(SrcToken {
                    tok: Token::Comment(comment.into()),
                    pos,
                }));
            }
        }

        if LIT_PRFIXES.contains(&chr) {
            if let Some(lit) = self.read_lit()? {
                return Ok(Some(SrcToken {
                    tok: lit.into(),
                    pos,
                }));
            }
        }

        if chr == '_' || chr.is_alphabetic() {
            if let Some(ident) = self.read_ident()? {
                let tok = if let Some(lex) = LEX_TOKENS.get_by_right(ident.as_str()) {
                    Token::Lex(*lex)
                } else {
                    Token::Ident(ident.into())
                };

                return Ok(Some(SrcToken { tok, pos }));
            }
        }

        if let Some(lex) = self.read_lex()? {
            return Ok(Some(SrcToken {
                tok: lex.into(),
                pos,
            }));
        }

        Err(LexErr::Syntax {
            msg: "unrecognized token",
            pos,
        })
    }

    fn read_comment(&mut self) -> LexResult<Option<String>> {
        let buf = self.peek_buf()?;
        if buf.bytes().nth(0) != Some(b'#') {
            return Ok(None);
        }

        let mut comment = String::new();
        self.reader.read_line(&mut comment)?;
        self.pos.column = 0;
        Ok(Some(comment))
    }

    fn read_ident(&mut self) -> LexResult<Option<String>> {
        let mut ident = String::new();

        loop {
            let buf = self.peek_buf()?;
            println!("{}", buf);
            let buf_len = buf.len();

            let buf_end_i = buf
                .char_indices()
                .find(|(i, c)| {
                    !(*c == '_'
                        || c.is_alphabetic()
                        || ((*i > 0 || ident.len() > 0) && c.is_numeric()))
                })
                .map(|(i, _)| i)
                .unwrap_or(buf_len);

            ident.push_str(&buf[..buf_end_i]);
            self.reader.consume(buf_end_i);

            if buf_end_i < buf_len {
                break;
            }
        }

        if ident.is_empty() {
            return Ok(None);
        }

        self.pos.column += ident.chars().count();
        Ok(Some(ident))
    }

    fn read_lex(&mut self) -> LexResult<Option<LexToken>> {
        let buf = self.peek_buf()?;
        if buf.len() < *MAX_LEX_TOKEN_LEN {
            return Err(LexErr::Io(io::Error::new(
                io::ErrorKind::OutOfMemory,
                "lex buffer smaller than max token length",
            )));
        }

        for n in (1..*MAX_LEX_TOKEN_LEN).rev() {
            if let Some(tok) = LEX_TOKENS.get_by_right(&buf[..n]) {
                return Ok(Some(*tok));
            }
        }

        Ok(None)
    }

    fn read_lit(&mut self) -> LexResult<Option<Literal>> {
        todo!()
    }
}
