pub enum TokenKind {
    None,
    String,
    Number,
}

pub struct Token {
    kind: TokenKind,
    lit: String,
    start: usize,
    ln: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lit: String, start: usize, ln: usize) -> Token {
        Token {
            kind,
            lit,
            start,
            ln,
        }
    }
}
