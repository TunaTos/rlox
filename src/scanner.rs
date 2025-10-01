use std::collections::HashMap;
use crate::token::{Literal, Token, TokenType};

pub struct Scanner {
    keywords: HashMap<String, TokenType>,
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            keywords: HashMap::from([
                ("and".to_string(), TokenType::AND),
                ("class".to_string(), TokenType::CLASS),
                ("else".to_string(), TokenType::ELSE),
                ("false".to_string(), TokenType::FALSE),
                ("for".to_string(), TokenType::FOR),
                ("fun".to_string(), TokenType::FUN),
                ("if".to_string(), TokenType::IF),
                ("nil".to_string(), TokenType::NIL),
                ("or".to_string(), TokenType::OR),
                ("print".to_string(), TokenType::PRINT),
                ("return".to_string(), TokenType::RETURN),
                ("super".to_string(), TokenType::SUPER),
                ("this".to_string(), TokenType::THIS),
                ("true".to_string(), TokenType::TRUE),
                ("var".to_string(), TokenType::VAR),
                ("while".to_string(), TokenType::WHILE),
            ]),
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::EOF,
            String::new(),
            None,
            self.line,
        ));

        self.tokens.clone()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None);
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(token_type, text, literal, self.line));
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),

            '!' => {
                let token_type = if self.token_match('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.token_match('=') {
                    TokenType::EQUAL_EQUAL
                } else {
                    TokenType::EQUAL
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.token_match('=') {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.token_match('=') {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                };
                self.add_token(token_type);
            }
            '/' => {
                if self.token_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH);
                }
            }

            ' ' | '\r' | '\t' => {}  
            '\n' => self.line += 1,

            '"' => self.string(),

            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    eprintln!("Unexpected character at line {}", self.line);
                }
            }
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            eprintln!("[line {}] Error: Unterminated string.", self.line);
            return;
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1].to_string();  
        self.add_token_literal(TokenType::STRING, Some(Literal::String(value)));
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let value: f64 = self.source[self.start..self.current].parse().unwrap();
        self.add_token_literal(TokenType::NUMBER, Some(Literal::Number(value)));
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = self.keywords
            .get(text)
            .cloned()
            .unwrap_or(TokenType::IDENTIFIER);

        self.add_token(token_type);
    }

    fn token_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {  
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()  
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {  
        self.is_alpha(c) || self.is_digit(c)
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::{scanner::{self, Scanner}, token::TokenType};

    #[test]
    fn test_keyword_recognition() {
        // Given
        let mut scanner = Scanner::new("var x".to_string());
        
        // When
        let tokens = scanner.scan_tokens();

        // Then
        assert_eq!(tokens[0].type_, TokenType::VAR)
    }

    
    
}
