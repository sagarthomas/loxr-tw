use crate::scanner::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token>{
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::new(),
            literal: String::new(),
            line: self.line,
        });
        return &self.tokens

    }

    fn is_at_end(&self) -> bool {
        self.current > self.source.len()
    }

    fn scan_token(&self) {
    }

    fn advance(&self) -> Option<char> {
        // TODO: might be better to save chars() iterator and then use .next()
        if let Some(c) = self.source.chars().nth(self.current) {
            return Some(c);
        } 
        None
    }
}
