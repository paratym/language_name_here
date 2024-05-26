use crate::lex::*;
use std::io::{self, BufRead};

pub struct Lexer {
    reader: Box<dyn BufRead>,
    pos: SrcPosition,
}

const NULL_CH: char = 0 as char;

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
        let ch = loop {
            let buf = self.peek_buf()?;
            let buf_len = buf.len();
            if buf_len == 0 {
                return Ok(None);
            }

            let mut start_ch = (buf_len, '\0');
            let mut offset = SrcPosition::default();
            for ch in buf.char_indices() {
                if !ch.1.is_whitespace() {
                    start_ch = ch;
                    break;
                } else if ch.1 == '\n' {
                    offset.line += 1;
                    offset.column = 0;
                } else {
                    offset.column += 1;
                }
            }

            self.reader.consume(start_ch.0);
            self.pos.line += offset.line;
            self.pos.column += offset.column;

            if start_ch.0 < buf_len {
                break start_ch.1;
            }
        };

        let pos = self.pos;
        let var_token = match ch {
            '#' => self.read_comment(),
            '"' => self.read_str_lit(),
            '\'' => self.read_char_lit(),
            '+' | '-' => self.read_num_lit(),
            ch if ch.is_ascii_digit() => self.read_num_lit(),
            '_' => self.read_ident(),
            ch if ch.is_ascii_alphabetic() => self.read_ident(),
            _ => Ok(None),
        }?;

        if let Some(tok) = var_token {
            if let Token::Comment(ref s)
            | Token::Ident(ref s)
            | Token::CharLit(ref s)
            | Token::StrLit(ref s)
            | Token::NumLit(ref s) = tok
            {
                if let Some(tok) = LEX_TOKENS.get_by_right(s.as_ref()) {
                    return Ok(Some(SrcToken {
                        tok: tok.clone(),
                        pos,
                    }));
                }
            }

            return Ok(Some(SrcToken { tok, pos }));
        }

        if let Some(tok) = var_token {
            Ok(Some(SrcToken { tok, pos }))
        } else if let Some(tok) = self.read_lex()? {
            Ok(Some(SrcToken { tok, pos }))
        } else {
            Err(LexErr::Syntax {
                msg: "unrecognized token",
                pos,
            })
        }
    }

    fn read_comment(&mut self) -> LexResult<Option<Token>> {
        let buf = self.peek_buf()?;
        if buf.bytes().nth(0) != Some(b'#') {
            return Ok(None);
        }

        let mut comment = String::new();
        self.reader.read_line(&mut comment)?;
        self.pos.line += 1;
        self.pos.column = 0;

        Ok(Some(Token::Comment(comment.into())))
    }

    fn read_ident(&mut self) -> LexResult<Option<Token>> {
        let mut ident = String::new();
        loop {
            let buf = self.peek_buf()?;
            let buf_byte_len = buf.len();
            if buf_byte_len == 0 {
                break;
            }

            let end_i = buf
                .char_indices()
                .find(|(i, c)| {
                    !(*c == '_'
                        || c.is_alphabetic()
                        || ((*i > 0 || ident.len() > 0) && c.is_numeric()))
                })
                .map(|x| x.0)
                .unwrap_or(buf_byte_len);

            ident.push_str(&buf[..end_i]);
            self.reader.consume(end_i);
            self.pos.column += end_i;

            if end_i < buf_byte_len {
                break;
            }
        }

        Ok(if ident.is_empty() {
            None
        } else {
            Some(Token::Ident(ident.into()))
        })
    }

    fn read_str_lit(&mut self) -> LexResult<Option<Token>> {
        let mut escaped = false;

        let mut lit = String::new();
        loop {
            let buf = self.peek_buf()?;
            let buf_byte_len = buf.len();
            if buf_byte_len == 0 {
                break;
            } else if lit.is_empty() && buf.bytes().nth(0) != Some(b'"') {
                return Ok(None);
            }

            let mut offset = SrcPosition::default();
            let mut end_i = buf_byte_len;

            for (i, c) in buf.char_indices() {
                if c == '\n' {
                    offset.line += 1;
                    offset.column = 0;
                } else {
                    offset.column += 1;
                }

                if c == '"' && !escaped {
                    end_i = i + 1;
                    break;
                }

                escaped = if c == '\\' && !escaped { true } else { false };
            }

            lit.push_str(&buf[..end_i]);
            self.reader.consume(end_i);
            self.pos.line += offset.line;
            self.pos.column += offset.column;

            if end_i < buf_byte_len {
                break;
            }
        }

        Ok(if lit.is_empty() {
            None
        } else {
            Some(Token::StrLit(lit.into()))
        })
    }

    fn read_char_lit(&mut self) -> LexResult<Option<Token>> {
        let mut escaped = false;

        let mut lit = String::new();
        loop {
            let buf = self.peek_buf()?;
            let buf_byte_len = buf.len();
            if buf_byte_len == 0 {
                break;
            } else if lit.is_empty() && buf.bytes().nth(0) != Some(b'\'') {
                return Ok(None);
            }

            let mut end_i = buf_byte_len;
            for (i, c) in buf.char_indices() {
                if c == '\'' && !escaped {
                    end_i = i + 1;
                    break;
                } else if c == '\n' {
                    self.reader.consume(i);
                    self.pos.column += i;
                    return Err(LexErr::Syntax {
                        msg: "unexpected new line",
                        pos: self.pos,
                    });
                }

                escaped = if c == '\\' && !escaped { true } else { false };
            }

            lit.push_str(&buf[..end_i]);
            self.reader.consume(end_i);
            self.pos.column += end_i;

            if end_i < buf_byte_len {
                break;
            }
        }

        Ok(if lit.is_empty() {
            None
        } else {
            Some(Token::CharLit(lit.into()))
        })
    }

    fn read_num_lit(&mut self) -> LexResult<Option<Token>> {
        let mut signed = false;
        let mut float = false;
        let mut first_digit = NULL_CH;
        let mut base_ch = NULL_CH;

        let mut lit = String::new();
        loop {
            let buf = self.peek_buf()?;
            let buf_byte_len = buf.len();
            if buf_byte_len == 0 {
                break;
            }

            let mut end_i = buf_byte_len;
            let mut chars = buf.char_indices().peekable();
            // todo: fix how unsuccessful peeks are handled
            while let Some((i, c)) = chars.next() {
                if is_valid_digit(base_ch, c) {
                    if first_digit == NULL_CH {
                        first_digit = c;
                    }

                    continue;
                }

                if (c == '+' || c == '-') && first_digit == NULL_CH {
                    if !is_valid_digit(base_ch, chars.peek().unwrap_or(&(0, NULL_CH)).1) {
                        return Ok(None);
                    }

                    signed = true;
                    continue;
                }

                let min_dec_i = signed as usize + ((base_ch != NULL_CH) as usize * 2) + 1;
                if c == '.' && !float && first_digit != NULL_CH && i >= min_dec_i {
                    if !is_valid_digit(base_ch, chars.peek().unwrap_or(&(0, NULL_CH)).1) {
                        end_i = i;
                        break;
                    }

                    float = true;
                    continue;
                }

                let base_ch_i = if signed { 2 } else { 1 };
                if i == base_ch_i && first_digit == '0' && (c == 'b' || c == 'o' || c == 'x') {
                    if !is_valid_digit(c, chars.peek().unwrap_or(&(0, NULL_CH)).1) {
                        end_i = i;
                        break;
                    }

                    base_ch = c;
                    continue;
                }

                end_i = i;
                break;
            }

            lit.push_str(&buf[..end_i]);
            self.reader.consume(end_i);
            self.pos.column += end_i;

            if end_i < buf_byte_len {
                break;
            }
        }

        Ok(if first_digit == NULL_CH {
            None
        } else {
            Some(Token::NumLit(lit.into()))
        })
    }

    fn read_lex(&mut self) -> LexResult<Option<Token>> {
        let buf = self.peek_buf()?;
        if buf.len() < *MAX_LEX_TOKEN_LEN {
            return Err(LexErr::Io(io::Error::new(
                io::ErrorKind::OutOfMemory,
                "lex buffer smaller than max token length",
            )));
        }

        for n in (1..*MAX_LEX_TOKEN_LEN).rev() {
            if let Some(tok) = LEX_TOKENS.get_by_right(&buf[..n]) {
                self.reader.consume(n);
                self.pos.column += n;
                return Ok(Some(tok.clone()));
            }
        }

        Ok(None)
    }
}

fn is_valid_digit(base: char, digit: char) -> bool {
    match (base, digit) {
        (NULL_CH, '0'..='9')
        | ('b', '0'..='1')
        | ('o', '0'..='7')
        | ('x', '0'..='9' | 'a'..='f' | 'A'..='F') => true,
        _ => false,
    }
}
