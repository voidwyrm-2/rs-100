use crate::common::Error;

use super::token::Token;

pub struct Lexer {
    text: Vec<u8>,
    init: bool,
    idx: usize,
    col: usize,
    ln: usize,
    ch: char,
}

impl Lexer {
    pub fn new(text: String) -> Lexer {
        let mut c = Lexer {
            text: text.as_bytes().to_vec(),
            init: false,
            idx: 0,
            col: 1,
            ln: 1,
            ch: '\0',
        };
        c.advance();
        c
    }

    fn advance(&mut self) {
        if self.init {
            self.init = false;
        } else {
            self.idx += 1;
            self.col += 1;
        }

        self.ch = if self.idx < self.text.len() {
            self.text[self.idx].into()
        } else {
            '\0'
        };

        if self.ch == '\n' {
            self.ln += 1;
            self.col = 1;
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, Error> {
        Ok(Vec::new())
    }
}
