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
                ("and".to_string(), TokenType::And),
                ("class".to_string(), TokenType::Class),
                ("else".to_string(), TokenType::Else),
                ("false".to_string(), TokenType::False),
                ("for".to_string(), TokenType::For),
                ("fun".to_string(), TokenType::Fun),
                ("if".to_string(), TokenType::If),
                ("nil".to_string(), TokenType::Nil),
                ("or".to_string(), TokenType::Or),
                ("print".to_string(), TokenType::Print),
                ("return".to_string(), TokenType::Return),
                ("super".to_string(), TokenType::Super),
                ("this".to_string(), TokenType::This),
                ("true".to_string(), TokenType::True),
                ("var".to_string(), TokenType::Var),
                ("while".to_string(), TokenType::While),
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
            TokenType::Eof,
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

            '!' => {
                let token_type = if self.token_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.token_match('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.token_match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.token_match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type);
            }
            '/' => {
                if self.token_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
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
        self.add_token_literal(TokenType::String, Some(Literal::String(value)));
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
        self.add_token_literal(TokenType::Number, Some(Literal::Number(value)));
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = self.keywords
            .get(text)
            .cloned()
            .unwrap_or(TokenType::Identifier);

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
    use crate::{scanner::{self, Scanner}, token::{self, Literal, TokenType}};

    /**
     * Single Character Tokens 
     */
    #[test]
    fn scan_left_paren() {
        let mut scanner = Scanner::new("(".to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::LeftParen);
        assert_eq!(tokens[0].lexeme, "(");
    }

    #[test]
    fn scan_right_paren() {
        let mut scanner = Scanner::new(")".to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[0].token_type, TokenType::RightParen);
    }

    #[test]
    fn scan_braces() {
        let mut scanner = Scanner::new("{}".to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[0].token_type, TokenType::LeftBrace);
        assert_eq!(tokens[1].token_type, TokenType::RightBrace);
    }

    #[test]
    fn scan_punctuation() {
        let mut scanner = Scanner::new(",.;".to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[0].token_type, TokenType::Comma);
        assert_eq!(tokens[1].token_type, TokenType::Dot);
        assert_eq!(tokens[2].token_type, TokenType::Semicolon);
    }
    
    #[test]
    fn scan_arithmetic_operators() {
        let mut scanner = Scanner::new("+-*/".to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[0].token_type, TokenType::Plus);
        assert_eq!(tokens[1].token_type, TokenType::Minus);
        assert_eq!(tokens[2].token_type, TokenType::Star);
        assert_eq!(tokens[3].token_type, TokenType::Slash);
    }

    #[test]
    fn scan_all_single_char_tokens_in_sequence() {
        let mut scanner = Scanner::new("(){},.;-+*/".to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 12); // 10 tokens + EOF

        assert_eq!(tokens[0].token_type, TokenType::LeftParen);
        assert_eq!(tokens[1].token_type, TokenType::RightParen);
        assert_eq!(tokens[2].token_type, TokenType::LeftBrace);
        assert_eq!(tokens[3].token_type, TokenType::RightBrace);
        assert_eq!(tokens[4].token_type, TokenType::Comma);
        assert_eq!(tokens[5].token_type, TokenType::Dot);
        assert_eq!(tokens[6].token_type, TokenType::Semicolon);
        assert_eq!(tokens[7].token_type, TokenType::Minus);
        assert_eq!(tokens[8].token_type, TokenType::Plus);
        assert_eq!(tokens[9].token_type, TokenType::Star);
        assert_eq!(tokens[10].token_type, TokenType::Slash);
        assert_eq!(tokens[11].token_type, TokenType::Eof);
    }

    /**
     * Two Character Tokens
     */
    #[test]
    fn scan_bang_equal() {
        let mut scanner = Scanner::new("!=".to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[0].token_type, TokenType::BangEqual);
        assert_eq!(tokens[0].lexeme, "!=");
    }

    #[test]
    fn scan_equal_equal() {
        let mut scanner = Scanner::new("==".to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[0].token_type, TokenType::EqualEqual);
        assert_eq!(tokens[0].lexeme, "==");
    }

    #[test]
    fn scan_comparison_operators() {
        let mut scanner = Scanner::new("<=>=<>".to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[0].token_type, TokenType::LessEqual);
        assert_eq!(tokens[1].token_type, TokenType::GreaterEqual);
        assert_eq!(tokens[2].token_type, TokenType::Less);
        assert_eq!(tokens[3].token_type, TokenType::Greater);
    }
    #[test]
    fn distinguish_single_bang_from_bang_equal() {
        let mut scanner = Scanner::new("! !=".to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[0].token_type, TokenType::Bang);
        assert_eq!(tokens[1].token_type, TokenType::BangEqual);
    }

    #[test]
    fn distinguish_single_equal_from_equal_equal() {
        let mut scanner = Scanner::new("= ==".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].token_type, TokenType::Equal);
        assert_eq!(tokens[1].token_type, TokenType::EqualEqual);
    }

    /**
     * Comment
     */
    #[test]
    fn ignore_single_line_comment() {
        let mut scanner = Scanner::new("// This is a comment".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Eof);
    }

    #[test]
    fn scan_token_before_comment() {
        let mut scanner = Scanner::new("var // comment".to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Var);
    }

    #[test]
    fn scan_token_after_comment() {
        let mut scanner = Scanner::new("// comment\nvar".to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[0].token_type, TokenType::Var);
    }

    #[test]
    fn comment_does_not_affect_next_line() {
        let mut scanner = Scanner::new("var x // comment\nvar y".to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[0].token_type, TokenType::Var);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].token_type, TokenType::Var);
        assert_eq!(tokens[3].token_type, TokenType::Identifier);
    }

    /**
     * Whitespace
     */
    #[test]
    fn ignore_spaces() {
        let mut scanner = Scanner::new("      var  ".to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Var);
        assert_eq!(tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn ignore_tabs() {
        let mut scanner = Scanner::new("\t\tvar\t\t".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].token_type, TokenType::Var);
    }

    #[test]
    fn ignore_carriage_return() {
        let mut scanner = Scanner::new("\r\rvar\r\r".to_string());
        let tokens = scanner.scan_tokens();

        assert_eq!(tokens[0].token_type, TokenType::Var);
    }

     #[test]
    fn handle_multiple_whitespace_types() {
        let mut scanner = Scanner::new(" \t\r var \t\r ".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens.len(), 2);
    }
    
    /**
     * String Literals
     */
    #[test]
    fn scan_simple_string() {
        let mut scanner = Scanner::new("\"hello\"".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].token_type, TokenType::String);
        match &tokens[0].literal {
            Some(Literal::String(s)) => assert_eq!(s, "hello"),
            _ => panic!("Expected string literal"),
        }
    }

    #[test]
    fn scan_empty_string() {
        let mut scanner = Scanner::new("\"\"".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].token_type, TokenType::String);
        match &tokens[0].literal {
            Some(Literal::String(s)) => assert_eq!(s, ""),
            _ => panic!("Expected empty string literal"),
        }
    }

    #[test]
    fn scan_string_with_spaces() {
        let mut scanner = Scanner::new("\"hello world\"".to_string());
        let tokens = scanner.scan_tokens();
        
        match &tokens[0].literal {
            Some(Literal::String(s)) => assert_eq!(s, "hello world"),
            _ => panic!("Expected string with spaces"),
        }
    }

    #[test]
    fn scan_multiline_string() {
        let mut scanner = Scanner::new("\"hello\nworld\"".to_string());
        let tokens = scanner.scan_tokens();
        
        match &tokens[0].literal {
            Some(Literal::String(s)) => assert_eq!(s, "hello\nworld"),
            _ => panic!("Expected multiline string"),
        }
        assert_eq!(tokens[0].line, 2); 
    }

    #[test]
    fn track_line_number_in_multiline_string() {
        let mut scanner = Scanner::new("\"line1\nline2\"\nvar".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[1].token_type, TokenType::Var);
        assert_eq!(tokens[1].line, 3); // var is on line 3
    }

    // Edge case: Unterminated string
    #[test]
    fn handle_unterminated_string() {
        let mut scanner = Scanner::new("\"unterminated".to_string());
        let tokens = scanner.scan_tokens();
        
        // Should still produce EOF token
        assert!(tokens.last().unwrap().token_type == TokenType::Eof);
    }

    /**
     * Number Literals
     */

    #[test]
    fn scan_integer() {
        let mut scanner = Scanner::new("123".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].token_type, TokenType::Number);
        match tokens[0].literal {
            Some(Literal::Number(n)) => assert_eq!(n, 123.0),
            _ => panic!("Expected number literal"),
        }
    }

    #[test]
    fn scan_decimal_number() {
        let mut scanner = Scanner::new("123.456".to_string());
        let tokens = scanner.scan_tokens();
        
        match tokens[0].literal {
            Some(Literal::Number(n)) => assert_eq!(n, 123.456),
            _ => panic!("Expected decimal number"),
        }
    }

    #[test]
    fn scan_zero() {
        let mut scanner = Scanner::new("0".to_string());
        let tokens = scanner.scan_tokens();
        
        match tokens[0].literal {
            Some(Literal::Number(n)) => assert_eq!(n, 0.0),
            _ => panic!("Expected zero"),
        }
    }

    #[test]
    fn scan_decimal_starting_with_zero() {
        let mut scanner = Scanner::new("0.5".to_string());
        let tokens = scanner.scan_tokens();
        
        match tokens[0].literal {
            Some(Literal::Number(n)) => assert_eq!(n, 0.5),
            _ => panic!("Expected 0.5"),
        }
    }

    #[test]
    fn scan_multiple_numbers() {
        let mut scanner = Scanner::new("1 2.5 100".to_string());
        let tokens = scanner.scan_tokens();
        
        match tokens[0].literal {
            Some(Literal::Number(n)) => assert_eq!(n, 1.0),
            _ => panic!("Expected 1.0"),
        }
        match tokens[1].literal {
            Some(Literal::Number(n)) => assert_eq!(n, 2.5),
            _ => panic!("Expected 2.5"),
        }
        match tokens[2].literal {
            Some(Literal::Number(n)) => assert_eq!(n, 100.0),
            _ => panic!("Expected 100.0"),
        }
    }

    /**
     * Keywords
     */
    #[test]
    fn scan_var_keyword() {
        let mut scanner = Scanner::new("var".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].token_type, TokenType::Var);
    }

    #[test]
    fn scan_control_flow_keywords() {
        let mut scanner = Scanner::new("if else while for".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].token_type, TokenType::If);
        assert_eq!(tokens[1].token_type, TokenType::Else);
        assert_eq!(tokens[2].token_type, TokenType::While);
        assert_eq!(tokens[3].token_type, TokenType::For);
    }

    #[test]
    fn scan_boolean_keywords() {
        let mut scanner = Scanner::new("true false".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].token_type, TokenType::True);
        assert_eq!(tokens[1].token_type, TokenType::False);
    }

    #[test]
    fn scan_function_keywords() {
        let mut scanner = Scanner::new("fun return".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].token_type, TokenType::Fun);
        assert_eq!(tokens[1].token_type, TokenType::Return);
    }

    #[test]
    fn scan_class_keywords() {
        let mut scanner = Scanner::new("class this super".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].token_type, TokenType::Class);
        assert_eq!(tokens[1].token_type, TokenType::This);
        assert_eq!(tokens[2].token_type, TokenType::Super);
    }

    #[test]
    fn scan_all_keywords() {
        let source = "and class else false fun for if nil or print return super this true var while".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        
        let expected = vec![
            TokenType::And, TokenType::Class, TokenType::Else, TokenType::False,
            TokenType::Fun, TokenType::For, TokenType::If, TokenType::Nil,
            TokenType::Or, TokenType::Print, TokenType::Return, TokenType::Super,
            TokenType::This, TokenType::True, TokenType::Var, TokenType::While,
        ];
        
        for (i, expected_type) in expected.iter().enumerate() {
            assert_eq!(&tokens[i].token_type, expected_type);
        }
    }

    /**
     * Identifiers
     */
    #[test]
    fn scan_simple_identifier() {
        let mut scanner = Scanner::new("myVar".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].lexeme, "myVar");
    }

    #[test]
    fn scan_identifier_with_underscore() {
        let mut scanner = Scanner::new("_variable".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].lexeme, "_variable");
    }

    #[test]
    fn scan_identifier_with_numbers() {
        let mut scanner = Scanner::new("var123".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].lexeme, "var123");
    }

    #[test]
    fn scan_multiple_identifiers() {
        let mut scanner = Scanner::new("foo bar baz".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].lexeme, "foo");
        assert_eq!(tokens[1].lexeme, "bar");
        assert_eq!(tokens[2].lexeme, "baz");
    }

    #[test]
    fn distinguish_keyword_from_identifier() {
        let mut scanner = Scanner::new("var variable".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].token_type, TokenType::Var);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
    }

    #[test]
    fn keyword_prefix_is_identifier() {
        let mut scanner = Scanner::new("varsity".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].lexeme, "varsity");
    }

    #[test]
    fn keyword_suffix_is_identifier() {
        let mut scanner = Scanner::new("myvar".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
    }

    /**
     * Line Tracking
     */
    #[test]
    fn track_line_number_for_single_line() {
        let mut scanner = Scanner::new("var x".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[1].line, 1);
    }

    #[test]
    fn track_line_number_across_newlines() {
        let mut scanner = Scanner::new("var\n+\n-".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].line, 1); // var
        assert_eq!(tokens[1].line, 2); // +
        assert_eq!(tokens[2].line, 3); // -
    }

    #[test]
    fn track_line_number_with_empty_lines() {
        let mut scanner = Scanner::new("var\n\n\nx".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].line, 1); // var
        assert_eq!(tokens[1].line, 4); // x
    }

    /**
     * Complex Expressions
     */
    #[test]
    fn scan_variable_declaration() {
        let mut scanner = Scanner::new("var x = 10;".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].token_type, TokenType::Var);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[2].token_type, TokenType::Equal);
        assert_eq!(tokens[3].token_type, TokenType::Number);
        assert_eq!(tokens[4].token_type, TokenType::Semicolon);
    }

    #[test]
    fn scan_arithmetic_expression() {
        let mut scanner = Scanner::new("10 + 20 * 30".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[1].token_type, TokenType::Plus);
        assert_eq!(tokens[2].token_type, TokenType::Number);
        assert_eq!(tokens[3].token_type, TokenType::Star);
        assert_eq!(tokens[4].token_type, TokenType::Number);
    }

    #[test]
    fn scan_comparison_expression() {
        let mut scanner = Scanner::new("x >= 10".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].token_type, TokenType::GreaterEqual);
        assert_eq!(tokens[2].token_type, TokenType::Number);
    }

    #[test]
    fn scan_if_statement() {
        let source = "if (x > 5) { print \"big\"; }".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        
        let expected = vec![
            TokenType::If,
            TokenType::LeftParen,
            TokenType::Identifier,
            TokenType::Greater,
            TokenType::Number,
            TokenType::RightParen,
            TokenType::LeftBrace,
            TokenType::Print,
            TokenType::String,
            TokenType::Semicolon,
            TokenType::RightBrace,
            TokenType::Eof,
        ];
        
        for (i, expected_type) in expected.iter().enumerate() {
            assert_eq!(&tokens[i].token_type, expected_type, 
                      "Token {} mismatch", i);
        }
    }

    #[test]
    fn scan_function_declaration() {
        let source = "fun greet(name) { print name; }".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens[0].token_type, TokenType::Fun);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
        assert_eq!(tokens[1].lexeme, "greet");
    }

    #[test]
    fn scan_complete_program() {
        let source = r#"
            var x = 10;
            var y = 20;
            if (x < y) {
                print "x is smaller";
            }
        "#.to_string();
        
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        
        // Just verify it doesn't crash and produces reasonable tokens
        assert!(tokens.len() > 10);
        assert_eq!(tokens.last().unwrap().token_type, TokenType::Eof);
    }

    /**
     * Edge Cases
     */
    #[test]
    fn scan_empty_input() {
        let mut scanner = Scanner::new("".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Eof);
    }

    #[test]
    fn scan_only_whitespace() {
        let mut scanner = Scanner::new("   \n\t\r  ".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Eof);
    }

    #[test]
    fn scan_only_comments() {
        let mut scanner = Scanner::new("// comment\n// another".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Eof);
    }

    #[test]
    fn eof_token_always_present() {
        let mut scanner = Scanner::new("var".to_string());
        let tokens = scanner.scan_tokens();
        
        assert_eq!(tokens.last().unwrap().token_type, TokenType::Eof);
    }

    #[test]
    fn eof_line_number_matches_last_token() {
        let mut scanner = Scanner::new("var\n+\n".to_string());
        let tokens = scanner.scan_tokens();
        
        let eof = tokens.last().unwrap();
        assert_eq!(eof.token_type, TokenType::Eof);
        assert_eq!(eof.line, 3);
    }
  
}
