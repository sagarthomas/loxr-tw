use core::fmt;

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,

    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,

    Equal,
    EqualEqual,

    Greater,
    GreaterEqual,

    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,

    This,
    True,
    Var,
    While,

    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Type: {}, Lexeme: {}, Literal '{}', Line: {}]",
            self.token_type, self.lexeme, self.literal, self.line
        )
    }
}
