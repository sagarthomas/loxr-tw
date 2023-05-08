use crate::scanner::token::{Token, TokenLiteral, TokenType};
use core::fmt;
use std::error::Error;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    errors: Vec<LexicalError>,
}

#[derive(Debug)]
pub struct LexicalError {
    line: usize,
    message: String,
}

impl Error for LexicalError {}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at line {}", self.message, self.line)
    }
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
            errors: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, &Vec<LexicalError>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::new(),
            literal: TokenLiteral::None,
            line: self.line,
        });

        if self.errors.len() != 0 {
            return Err(&self.errors);
        }
        return Ok(&self.tokens);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        // TODO: Not sure yet of unwrap or default is the way to go as we wont
        // index a character beyond bounds due to how the loop works.
        // Perhaps better to error here if we get a NOne
        let c = self.advance().unwrap_or_default();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => match self.match_next('=') {
                true => self.add_token(TokenType::BangEqual),
                false => self.add_token(TokenType::Bang),
            },
            '=' => match self.match_next('=') {
                true => self.add_token(TokenType::EqualEqual),
                false => self.add_token(TokenType::Equal),
            },
            '<' => match self.match_next('=') {
                true => self.add_token(TokenType::LessEqual),
                false => self.add_token(TokenType::Less),
            },
            '>' => match self.match_next('=') {
                true => self.add_token(TokenType::GreaterEqual),
                false => self.add_token(TokenType::Greater),
            },
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_next('*') {
                    self.parse_block_comment()
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            '"' => self.parse_string(),
            '0'..='9' => self.parse_number(),
            _ => {
                if self.is_alpha(c) {
                    self.parse_identifier()
                } else {
                    self.errors.push(LexicalError {
                        line: self.line,
                        message: String::from("Unrecognized character"),
                    })
                }
            }
        }
    }

    fn advance(&mut self) -> Option<char> {
        // TODO: might be better to save chars() iterator and then use .next()
        if let Some(c) = self.source.chars().nth(self.current) {
            self.current += 1;
            return Some(c);
        }
        None
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, TokenLiteral::None)
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: TokenLiteral) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type,
            lexeme: text.to_string(),
            literal,
            line: self.line,
        })
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if let Some(c) = self.source.chars().nth(self.current) {
            if c == expected {
                self.current += 1;
                return true;
            }
        }
        return false;
    }

    fn peek(&self) -> char {
        // The original implementation called is_at_end() first and return
        // '\0' if we consumed all characters
        match self.source.chars().nth(self.current) {
            Some(c) => return c,
            None => return '\0',
        }
    }

    fn peek_next(&self) -> char {
        match self.source.chars().nth(self.current + 1) {
            Some(c) => return c,
            None => return '\0',
        }
    }

    fn parse_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.errors.push(LexicalError {
                line: self.line,
                message: String::from("Unterminated string"),
            })
        }

        //Consume the closing '"'
        self.advance();

        //Trim quotes
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_with_literal(TokenType::String, TokenLiteral::String(value))
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c == '_')
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn parse_identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = self.source[self.start..self.current].to_string();
        match TokenType::parse(text) {
            Some(t) => self.add_token(t),
            None => self.add_token(TokenType::Identifier),
        }
    }

    fn parse_number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let value: usize = self.source[self.start..self.current]
            .to_string()
            .parse()
            .unwrap();

        self.add_token_with_literal(TokenType::Number, TokenLiteral::Number(value))
    }

    fn parse_block_comment(&mut self) {
        while (self.peek() != '*' || self.peek_next() != '/') && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.errors.push(LexicalError {
                line: self.line,
                message: String::from("Unterminated block comment"),
            })
        }
        // Consume the final */
        self.advance();
        self.advance();
    }
}
