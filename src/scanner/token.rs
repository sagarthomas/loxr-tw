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

impl TokenType {
    pub fn parse(keyword: String) -> Option<TokenType> {
        match keyword.as_str() {
            "and" => Some(Self::And),
            "class" => Some(Self::Class),
            "else" => Some(Self::Else),
            "false" => Some(Self::False),
            "for" => Some(Self::For),
            "fun" => Some(Self::Fun),
            "if" => Some(Self::If),
            "nil" => Some(Self::Nil),
            "or" => Some(Self::Or),
            "print" => Some(Self::Print),
            "return" => Some(Self::Return),
            "super" => Some(Self::Super),
            "this" => Some(Self::This),
            "true" => Some(Self::This),
            "var" => Some(Self::Var),
            "while" => Some(Self::While),
            _ => None
        }
    }
}

#[derive(Debug)]
pub enum TokenLiteral {
    Number(usize),
    String(String),
    None
}

impl fmt::Display for TokenLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: TokenLiteral,
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
