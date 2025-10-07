use crate::{
    expr::{Binary, Expr, LiteralValue, Unary},
    token::{Token, TokenType},
};

/// Recursive descent parser for Lox expressions
///
/// Grammar (in order of precedence, lowest to highest):
/// ```text
/// expression → equality
/// equality   → comparison ( ( "!=" | "==" ) comparison )*
/// comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )*
/// term       → factor ( ( "-" | "+" ) factor )*
/// factor     → unary ( ( "/" | "*" ) unary )*
/// unary      → ( "!" | "-" ) unary | primary
/// primary    → NUMBER | STRING | "true" | "false" | "nil"
///            | "(" expression ")"
/// ```
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

// ParseError 추가
#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub token: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    // === Public API ===

    /// Main entry point for parsing - returns Result instead of panicking
    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expression_result()
    }

    // === Helper methods ===

    // Check if current token match given type without consuming it
    pub fn check(&self, token_type: TokenType) -> bool {
        !self.is_at_end() && self.peek().token_type == token_type
    }

    // Consume current token and return it
    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    pub fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    // Error handling version
    fn expression_result(&mut self) -> Result<Expr, ParseError> {
        self.equality_result()
    }

    // Original panic version for backward compatibility
    pub fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality_result(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison_result()?;

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison_result()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        expr
    }

    pub fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for token_type in types.iter() {
            if self.check(token_type.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn comparison_result(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term_result()?;

        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term_result()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        Ok(expr)
    }

    pub fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }
        expr
    }

    fn term_result(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor_result()?;

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor_result()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    pub fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        expr
    }

    fn factor_result(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary_result()?;

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary_result()?;
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        Ok(expr)
    }

    pub fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }
        expr
    }

    fn unary_result(&mut self) -> Result<Expr, ParseError> {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary_result()?;
            return Ok(Expr::Unary(Unary {
                operator,
                right: Box::new(right),
            }));
        }
        self.primary_result()
    }

    pub fn unary(&mut self) -> Expr {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::Unary(Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.primary()
    }

    fn primary_result(&mut self) -> Result<Expr, ParseError> {
        if self.match_tokens(&[TokenType::False]) {
            return Ok(Expr::Literal(crate::expr::Literal {
                value: LiteralValue::Bool(false),
            }));
        }

        if self.match_tokens(&[TokenType::True]) {
            return Ok(Expr::Literal(crate::expr::Literal {
                value: LiteralValue::Bool(true),
            }));
        }

        if self.match_tokens(&[TokenType::Nil]) {
            return Ok(Expr::Literal(crate::expr::Literal {
                value: LiteralValue::Nil,
            }));
        }

        if self.match_tokens(&[TokenType::Number, TokenType::String]) {
            let token = self.previous();
            if let Some(literal) = &token.literal {
                return Ok(Expr::Literal(crate::expr::Literal {
                    value: match literal {
                        crate::token::Literal::Number(n) => LiteralValue::Number(*n),
                        crate::token::Literal::String(s) => LiteralValue::String(s.clone()),
                        _ => {
                            return Err(ParseError {
                                message: "Unexpected literal type".to_string(),
                                token: token.clone(),
                            });
                        }
                    },
                }));
            }
        }

        if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expression_result()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(crate::expr::Grouping {
                expression: Box::new(expr),
            }));
        }

        Err(ParseError {
            message: "Expect expression.".to_string(),
            token: self.peek().clone(),
        })
    }

    pub fn primary(&mut self) -> Expr {
        if self.match_tokens(&[TokenType::False]) {
            return Expr::Literal(crate::expr::Literal {
                value: LiteralValue::Bool(false),
            });
        }

        if self.match_tokens(&[TokenType::True]) {
            return Expr::Literal(crate::expr::Literal {
                value: LiteralValue::Bool(true),
            });
        }

        if self.match_tokens(&[TokenType::Nil]) {
            return Expr::Literal(crate::expr::Literal {
                value: LiteralValue::Nil,
            });
        }

        if self.match_tokens(&[TokenType::Number, TokenType::String]) {
            let token = self.previous();
            if let Some(literal) = &token.literal {
                return Expr::Literal(crate::expr::Literal {
                    value: match literal {
                        crate::token::Literal::Number(n) => LiteralValue::Number(*n),
                        crate::token::Literal::String(s) => LiteralValue::String(s.clone()),
                        _ => panic!("Unexpected literal type"),
                    },
                });
            }
        }

        if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Expr::Grouping(crate::expr::Grouping {
                expression: Box::new(expr),
            });
        }

        panic!("Expect expression.");
    }

    // Result version of consume
    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(ParseError {
                message: message.to_string(),
                token: self.peek().clone(),
            })
        }
    }

    // Error reporting
    fn error(&self, token: &Token, message: &str) -> ParseError {
        ParseError {
            message: message.to_string(),
            token: token.clone(),
        }
    }

    // Synchronization for error recovery
    pub fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }
}


/// test codes
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ast_printer::{self, AstPrinter},
        expr, parser,
    };
    use std::vec;

    #[test]
    fn test_make_parser() {
        // Given
        let tokens = vec![
            Token {
                token_type: TokenType::Number,
                lexeme: "42".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                literal: None,
                line: 1,
            },
        ];

        // When
        let parser = Parser::new(tokens);

        // Then
        assert_eq!(parser.current, 0);
        assert_eq!(parser.peek().token_type, TokenType::Number);
    }

    #[test]
    fn test_eqality_bang_equal() {
        // Given
        // 3 != 4
        let mut parser = Parser::new(vec![
            Token {
                token_type: TokenType::Number,
                lexeme: "3".to_string(),
                literal: Some(crate::token::Literal::Number(3.0)),
                line: 0,
            },
            Token {
                token_type: TokenType::BangEqual,
                lexeme: "!=".to_string(),
                literal: None,
                line: 0,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "5".to_string(),
                literal: Some(crate::token::Literal::Number(5.0)),
                line: 0,
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                literal: None,
                line: 1,
            },
        ]);

        // When
        let expr: Expr = parser.expression();
        // Then
        let printer = AstPrinter::new();
        assert_eq!(printer.print(&expr), "(!= 3 5)");
    }

    #[test]
    fn test_eqality_equal_equal() {
        // Given
        // 3 == 3
        let mut parser = Parser::new(vec![
            Token {
                token_type: TokenType::Number,
                lexeme: "3".to_string(),
                literal: Some(crate::token::Literal::Number(3.0)),
                line: 1,
            },
            Token {
                token_type: TokenType::EqualEqual,
                lexeme: "==".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "3".to_string(),
                literal: Some(crate::token::Literal::Number(3.0)),
                line: 1,
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                literal: None,
                line: 1,
            },
        ]);
        // When
        let expr = parser.equality();

        // Then
        let printer = AstPrinter::new();
        assert_eq!(printer.print(&expr), "(== 3 3)");
    }

    #[test]
    fn test_comparison_greater() {
        // Given: 5 > 3
        let mut parser = Parser::new(vec![
            Token {
                token_type: TokenType::Number,
                lexeme: "5".to_string(),
                literal: Some(crate::token::Literal::Number(5.0)),
                line: 1,
            },
            Token {
                token_type: TokenType::Greater,
                lexeme: ">".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "3".to_string(),
                literal: Some(crate::token::Literal::Number(3.0)),
                line: 1,
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                literal: None,
                line: 1,
            },
        ]);

        // When
        let expr = parser.comparison();

        // Then
        let printer = AstPrinter::new();
        assert_eq!(printer.print(&expr), "(> 5 3)");
    }

    #[test]
    fn test_comparison_greater_equal() {
        // Given: 5 >= 5
        let mut parser = Parser::new(vec![
            Token {
                token_type: TokenType::Number,
                lexeme: "5".to_string(),
                literal: Some(crate::token::Literal::Number(5.0)),
                line: 1,
            },
            Token {
                token_type: TokenType::GreaterEqual,
                lexeme: ">=".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "5".to_string(),
                literal: Some(crate::token::Literal::Number(5.0)),
                line: 1,
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                literal: None,
                line: 1,
            },
        ]);

        // When
        let expr = parser.comparison();

        // Then
        let printer = AstPrinter::new();
        assert_eq!(printer.print(&expr), "(>= 5 5)");
    }
    #[test]
    fn test_comparison_less() {
        // Given
        // 3 < 5
        let mut parser = Parser::new(vec![
            Token {
                token_type: TokenType::Number,
                lexeme: "3".to_string(),
                literal: Some(crate::token::Literal::Number(3.0)),
                line: 1,
            },
            Token {
                token_type: TokenType::Less,
                lexeme: "<".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "5".to_string(),
                literal: Some(crate::token::Literal::Number(5.0)),
                line: 1,
            },
            Token {
                lexeme: "".to_string(),
                token_type: TokenType::Eof,
                literal: None,
                line: 1,
            },
        ]);

        // When
        let expr = parser.comparison();

        // Then
        let printer = AstPrinter::new();
        assert_eq!(printer.print(&expr), "(< 3 5)");
    }

    #[test]
    fn test_comparison_less_equal() {
        //Givem
        // 3 <= 3
        let mut parser = Parser::new(vec![
            Token {
                token_type: TokenType::Number,
                lexeme: "3".to_string(),
                literal: Some(crate::token::Literal::Number(3.0)),
                line: 1,
            },
            Token {
                token_type: TokenType::Less,
                lexeme: "<=".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "3".to_string(),
                literal: Some(crate::token::Literal::Number(3.0)),
                line: 1,
            },
            Token {
                lexeme: "".to_string(),
                token_type: TokenType::Eof,
                literal: None,
                line: 1,
            },
        ]);

        // When
        let expr = parser.comparison();

        // Then
        let printer = AstPrinter::new();
        assert_eq!(printer.print(&expr), "(<= 3 3)");
    }

    #[test]
    fn test_term_minus() {
        // Given
        // 5 - 3
        let mut parser = Parser::new(vec![
            Token {
                token_type: TokenType::Number,
                lexeme: "5".to_string(),
                literal: Some(crate::token::Literal::Number(5.0)),
                line: 1,
            },
            Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "3".to_string(),
                literal: Some(crate::token::Literal::Number(3.0)),
                line: 1,
            },
            Token {
                lexeme: "".to_string(),
                token_type: TokenType::Eof,
                literal: None,
                line: 1,
            },
        ]);

        // When
        let expr = parser.term();

        // Then
        let printer = AstPrinter::new();
        assert_eq!(printer.print(&expr), "(- 5 3)");
    }

    #[test]
    fn test_term_plus() {
        // Given
        // 5 + 3
        let mut parser = Parser::new(vec![
            Token {
                token_type: TokenType::Number,
                lexeme: "5".to_string(),
                literal: Some(crate::token::Literal::Number(5.0)),
                line: 1,
            },
            Token {
                token_type: TokenType::Plus,
                lexeme: "+".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "3".to_string(),
                literal: Some(crate::token::Literal::Number(3.0)),
                line: 1,
            },
            Token {
                lexeme: "".to_string(),
                token_type: TokenType::Eof,
                literal: None,
                line: 1,
            },
        ]);

        // When
        let expr = parser.term();

        // Then
        let printer = AstPrinter::new();
        assert_eq!(printer.print(&expr), "(+ 5 3)");
    }

    #[test]
    fn test_factor_slash() {
        // Given
        // 6 / 2
        let mut parser = Parser::new(vec![
            Token {
                token_type: TokenType::Number,
                lexeme: "6".to_string(),
                literal: Some(crate::token::Literal::Number(6.0)),
                line: 1,
            },
            Token {
                token_type: TokenType::Slash,
                lexeme: "/".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "2".to_string(),
                literal: Some(crate::token::Literal::Number(2.0)),
                line: 1,
            },
            Token {
                lexeme: "".to_string(),
                token_type: TokenType::Eof,
                literal: None,
                line: 1,
            },
        ]);

        // When
        let expr = parser.factor();

        // Then
        let printer = AstPrinter::new();
        assert_eq!(printer.print(&expr), "(/ 6 2)");
    }

    #[test]
    fn test_factor_star() {
        // Given
        // 3 * 4
        let mut parser = Parser::new(vec![
            Token {
                token_type: TokenType::Number,
                lexeme: "3".to_string(),
                literal: Some(crate::token::Literal::Number(3.0)),
                line: 1,
            },
            Token {
                token_type: TokenType::Star,
                lexeme: "*".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "4".to_string(),
                literal: Some(crate::token::Literal::Number(4.0)),
                line: 1,
            },
            Token {
                lexeme: "".to_string(),
                token_type: TokenType::Eof,
                literal: None,
                line: 1,
            },
        ]);

        // When
        let expr = parser.factor();

        // Then
        let printer = AstPrinter::new();
        assert_eq!(printer.print(&expr), "(* 3 4)");
    }

    #[test]
    fn test_unary_bang() {
        // Given
        // !true
        let mut parser = Parser::new(vec![
            Token {
                token_type: TokenType::Bang,
                lexeme: "!".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::True,
                lexeme: "true".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                lexeme: "".to_string(),
                token_type: TokenType::Eof,
                literal: None,
                line: 1,
            },
        ]);

        // When
        let expr = parser.unary();

        // Then
        let printer = AstPrinter::new();
        assert_eq!(printer.print(&expr), "(! true)");
    }

    #[test]
    fn test_unary_minus() {
        // Given
        // -5
        let mut parser = Parser::new(vec![
            Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "5".to_string(),
                literal: Some(crate::token::Literal::Number(5.0)),
                line: 1,
            },
            Token {
                lexeme: "".to_string(),
                token_type: TokenType::Eof,
                literal: None,
                line: 1,
            },
        ]);

        // When
        let expr = parser.unary();

        // Then
        let printer = AstPrinter::new();
        assert_eq!(printer.print(&expr), "(- 5)");
    }

    #[test]
    fn test_primary_number() {
        // Given
        let mut parser = Parser::new(vec![
            Token {
                token_type: TokenType::Number,
                lexeme: "42".to_string(),
                literal: Some(crate::token::Literal::Number(42.0)),
                line: 1,
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                literal: None,
                line: 1,
            },
        ]);

        // When
        let expr = parser.primary();

        // Then
        let printer = AstPrinter::new();
        assert_eq!(printer.print(&expr), "42");
    }

    #[test]
    fn test_primary_string() {
        // Given
        let mut parser = Parser::new(vec![
            Token {
                token_type: TokenType::String,
                lexeme: "\"hello\"".to_string(),
                literal: Some(crate::token::Literal::String("hello".to_string())),
                line: 1,
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                literal: None,
                line: 1,
            },
        ]);

        // When
        let expr = parser.primary();

        // Then
        let printer = AstPrinter::new();
        assert_eq!(printer.print(&expr), "hello");
    }

    #[test]
    fn test_primary_grouping() {
        // Given
        // (5)
        let mut parser = Parser::new(vec![
            Token {
                token_type: TokenType::LeftParen,
                lexeme: "(".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "5".to_string(),
                literal: Some(crate::token::Literal::Number(5.0)),
                line: 1,
            },
            Token {
                token_type: TokenType::RightParen,
                lexeme: ")".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                literal: None,
                line: 1,
            },
        ]);

        // When
        let expr = parser.primary();

        // Then
        let printer = AstPrinter::new();
        assert_eq!(printer.print(&expr), "(group 5)");
    }

    #[test]
    fn test_complex_expression() {
        // Given
        // -5 + 3 * 2 == 1
        let mut parser = Parser::new(vec![
            Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "5".to_string(),
                literal: Some(crate::token::Literal::Number(5.0)),
                line: 1,
            },
            Token {
                token_type: TokenType::Plus,
                lexeme: "+".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "3".to_string(),
                literal: Some(crate::token::Literal::Number(3.0)),
                line: 1,
            },
            Token {
                token_type: TokenType::Star,
                lexeme: "*".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "2".to_string(),
                literal: Some(crate::token::Literal::Number(2.0)),
                line: 1,
            },
            Token {
                token_type: TokenType::EqualEqual,
                lexeme: "==".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Number,
                lexeme: "1".to_string(),
                literal: Some(crate::token::Literal::Number(1.0)),
                line: 1,
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_string(),
                literal: None,
                line: 1,
            },
        ]);

        // When
        let expr = parser.expression();

        // Then
        let printer = AstPrinter::new();
        assert_eq!(printer.print(&expr), "(== (+ (- 5) (* 3 2)) 1)");
    }
}
