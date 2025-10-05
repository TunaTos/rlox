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

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
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

    pub fn expression(&mut self) -> Expr {
        self.equality()
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
        for (i, token_type) in types.iter().enumerate() {
            if self.check(token_type.clone()) {
                self.advance();
                return true;
            }
        }
        false
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

    pub fn unary(&mut self) -> Expr {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::Unary(Unary {
                operator,
                right: Box::new(right),
            });
        }
        return self.primary();
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

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if self.check(token_type) {
            self.advance();
            return;
        }
        panic!("{}", message);
    }
}

#[cfg(test)]
mod tests {
    use std::vec;
    use crate::{ast_printer::{self, AstPrinter}, parser};
    use super::*;

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
        let mut parser = Parser::new(
            vec![
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
                },Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line: 1,
        },
            ]
        );

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
        let mut parser = Parser::new(
            vec![
                Token {
                    token_type: TokenType::Number,
                    lexeme: "3".to_string(),
                    literal: Some(crate::token::Literal::Number(3.0)),
                    line: 1,
                },
                Token {
                    token_type:TokenType::EqualEqual,
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
            ]
        );
        // When
        let expr = parser.equality();
        
        // Then
        let printer = AstPrinter::new();
        assert_eq!(printer.print(&expr), "(== 3 3)");
    }
}