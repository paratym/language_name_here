use crate::tokenizer::*;
use std::io::BufRead;

pub struct Tokenizer {
    reader: Box<dyn BufRead>,
    pos: SrcPosition,
    cached: Option<SrcToken>,
}

const NULL_CH: char = 0 as char;

impl Tokenizer {
    pub fn new(reader: Box<dyn BufRead>) -> Self {
        Self {
            reader,
            pos: SrcPosition { line: 0, column: 0 },
            cached: None,
        }
    }

    fn peek_buf(&mut self) -> TokResult<&str> {
        let buf = self.reader.fill_buf()?;
        let mut end_i = buf.len();
        loop {
            match std::str::from_utf8(&buf[..end_i]) {
                Ok(s) => return Ok(s),
                Err(e) => {
                    end_i = e.valid_up_to();
                    if end_i == 0 {
                        return Err(TokErr::Syntax {
                            pos: self.pos,
                            msg: "invalid utf8 character",
                        });
                    }
                }
            }
        }
    }

    pub fn pos(&self) -> &SrcPosition {
        &self.pos
    }

    pub fn peek_token(&mut self) -> TokResult<&SrcToken> {
        if let Some(ref tok) = self.cached {
            return Ok(tok);
        }

        self.cached = match self.next_token() {
            Ok(tok) => Some(tok),
            Err(TokErr::ReaderEmpty) => None,
            Err(e) => return Err(e),
        };

        return Ok(self.cached.as_ref().unwrap());
    }

    pub fn next_token(&mut self) -> TokResult<SrcToken> {
        if let Some(ref tok) = self.cached {
            let token = tok.clone();
            self.cached = None;
            return Ok(token);
        }

        let ch = loop {
            let buf = self.peek_buf()?;
            let buf_len = buf.len();
            if buf_len == 0 {
                return Err(TokErr::ReaderEmpty);
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
            '#' => Some(self.read_comment()),
            '"' => Some(self.read_str_lit()),
            '\'' => Some(self.read_char_lit()),
            '+' | '-' => Some(self.read_num_lit()),
            ch if ch.is_ascii_digit() => Some(self.read_num_lit()),
            '_' => Some(self.read_alias()),
            ch if ch.is_ascii_alphabetic() => Some(self.read_alias()),
            _ => None,
        };

        let tok = match var_token.transpose() {
            Ok(Some(tok)) => LEX_TOKENS
                .get_by_right(tok.to_string().as_str())
                .cloned()
                .unwrap_or(tok),
            Ok(None) => self.read_lex()?,
            Err(e) => return Err(e),
        };

        Ok(SrcToken { tok, pos })
    }

    pub fn expect_token(&mut self, tok: &Token) -> TokResult<SrcToken> {
        let token = self.next_token()?;
        if &token.tok != tok {
            return Err(TokErr::Syntax {
                pos: token.pos,
                msg: "unexpected token",
            });
        }

        Ok(token)
    }

    fn read_comment(&mut self) -> TokResult<Token> {
        let buf = self.peek_buf()?;
        if buf.as_bytes().first() != Some(&b'#') {
            return Err(TokErr::Syntax {
                pos: self.pos,
                msg: "expected comment",
            });
        }

        let mut comment = String::new();
        self.reader.read_line(&mut comment)?;
        self.pos.line += 1;
        self.pos.column = 0;

        Ok(Token::Comment(comment.into()))
    }

    fn read_alias(&mut self) -> TokResult<Token> {
        let mut alias = String::new();
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
                        || ((*i > 0 || !alias.is_empty()) && c.is_numeric()))
                })
                .map(|x| x.0)
                .unwrap_or(buf_byte_len);

            alias.push_str(&buf[..end_i]);
            self.reader.consume(end_i);
            self.pos.column += end_i;

            if end_i < buf_byte_len {
                break;
            }
        }

        if alias.is_empty() {
            return Err(TokErr::Syntax {
                pos: self.pos,
                msg: "expected alias",
            });
        }

        Ok(Token::Alias(alias.into()))
    }

    fn read_str_lit(&mut self) -> TokResult<Token> {
        let mut escaped = true;

        let mut lit = String::new();
        loop {
            let buf = self.peek_buf()?;
            let buf_byte_len = buf.len();
            if buf_byte_len == 0 {
                break;
            } else if lit.is_empty() && buf.as_bytes().first() != Some(&b'"') {
                return Err(TokErr::Syntax {
                    pos: self.pos,
                    msg: "expected string literal",
                });
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

                escaped = c == '\\' && !escaped;
            }

            lit.push_str(&buf[..end_i]);
            self.reader.consume(end_i);
            self.pos.line += offset.line;
            self.pos.column += offset.column;

            if end_i < buf_byte_len {
                break;
            }
        }

        Ok(Token::StrLit(lit.into()))
    }

    fn read_char_lit(&mut self) -> TokResult<Token> {
        let mut escaped = true;

        let mut lit = String::new();
        loop {
            let buf = self.peek_buf()?;
            let buf_byte_len = buf.len();
            if buf_byte_len == 0 {
                break;
            } else if lit.is_empty() && buf.as_bytes().first() != Some(&b'\'') {
                return Err(TokErr::Syntax {
                    pos: self.pos,
                    msg: "expected character literal",
                });
            }

            let mut end_i = buf_byte_len;
            for (i, c) in buf.char_indices() {
                if c == '\'' && !escaped {
                    end_i = i + 1;
                    break;
                } else if c == '\n' {
                    self.reader.consume(i);
                    self.pos.column += i;
                    return Err(TokErr::Syntax {
                        msg: "unexpected new line",
                        pos: self.pos,
                    });
                }

                escaped = c == '\\' && !escaped;
            }

            lit.push_str(&buf[..end_i]);
            self.reader.consume(end_i);
            self.pos.column += end_i;

            if end_i < buf_byte_len {
                break;
            }
        }

        Ok(Token::CharLit(lit.into()))
    }

    fn read_num_lit(&mut self) -> TokResult<Token> {
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
                        return Err(TokErr::Syntax {
                            pos: self.pos,
                            msg: "signs can only be declared in integer literals",
                        });
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

                let base_ch_i = signed as usize + 1;
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

        if first_digit == NULL_CH {
            return Err(TokErr::Syntax {
                pos: self.pos,
                msg: "expected numeric literal",
            });
        }

        Ok(Token::NumLit(lit.into()))
    }

    fn read_lex(&mut self) -> TokResult<Token> {
        let buf = self.peek_buf()?;
        let max = MAX_LEX_TOKEN_LEN.min(buf.len());
        for n in (1..max).rev() {
            if let Some(tok) = LEX_TOKENS.get_by_right(&buf[..n]) {
                self.reader.consume(n);
                self.pos.column += n;
                return Ok(tok.clone());
            }
        }

        Err(TokErr::Syntax {
            pos: self.pos,
            msg: "expected lexical token",
        })
    }
}

fn is_valid_digit(base: char, digit: char) -> bool {
    matches!((base, digit),
        (NULL_CH, '0'..='9')
        | ('b', '0'..='1')
        | ('o', '0'..='7')
        | ('x', '0'..='9' | 'a'..='f' | 'A'..='F')
    )
}
