use crate::expr::{Binary, Expr, Grouping, Literal, LiteralValue, Unary};

/// Printer that converts AST to human-readable strings
///
/// Uses Lisp-style parenthesized notation to clearly display expression structure.
///
/// # Examples
/// - `1 + 2` -> `(+ 1 2)`
/// - `-(123)` -> `(- 123)`
/// - `(1 + 2) * 3` -> `(* (group (+ 1 2)) 3)`
pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        Self
    }

    /// Convert an expression to a string
    ///
    /// # Example
    /// ```ignore
    /// let printer = AstPrinter::new();
    /// let result = printer.print(&expr);
    /// println!("{}", result); // "(+ 1 2)"
    /// ```
    pub fn print(&self, expr: &Expr) -> String {
        match expr {
            Expr::Binary(binary) => self.visit_binary(binary),
            Expr::Grouping(grouping) => self.visit_grouping(grouping),
            Expr::Literal(literal) => self.visit_literal(literal),
            Expr::Unary(unary) => self.visit_unary(unary),
            _ => String::from("(not implemented)"),
        }
    }

    /// Process binary operator expressions
    ///
    /// # Examples
    /// - '1 + 2' -> '(+ 1 2)'
    /// - '3 * 4' -> '(* 3 4)'
    fn visit_binary(&self, expr: &Binary) -> String {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }

    /// Process grouping (parentheses) expressions
    ///
    /// # Examples
    /// - '(1 + 2)' -> '(group (+ 1 2))'
    fn visit_grouping(&self, expr: &Grouping) -> String {
        self.parenthesize("group", &[&expr.expression])
    }

    /// Process literal value expressions
    ///
    /// # Examples
    /// - `123` -> `"123"`
    /// - `"hello"` -> `"hello"`
    /// - `true` -> `"true"`
    /// - `nil` -> `"nil"`
    fn visit_literal(&self, expr: &Literal) -> String {
        match &expr.value {
            LiteralValue::Number(n) => n.to_string(),
            LiteralValue::String(s) => s.clone(),
            LiteralValue::Bool(b) => b.to_string(),
            LiteralValue::Nil => String::from("nil"),
        }
    }

    /// Process unary operator expressions
    ///
    /// # Examples
    /// - '-5' -> '(- 5)'
    /// - '!true' -> '(! true)'
    fn visit_unary(&self, expr: &Unary) -> String {
        self.parenthesize(&expr.operator.lexeme, &[&expr.right])
    }

    /// Examples
    /// - 'parenthesize("+", &[1,2])' -> "(+ 1 2)"
    /// - 'parenthesize("group", &[expr]) -> "(group ...)"
    fn parenthesize(&self, name: &str, exprs: &[&Box<Expr>]) -> String {
        let mut result = String::new();

        result.push('(');
        result.push_str(name);

        for expr in exprs {
            result.push(' ');
            result.push_str(&self.print(expr));
        }

        result.push(')');
        result
    }
}

// test codes
#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::{Token, TokenType};

    // Literal Type tests
    #[test]
    fn test_literal_number() {
        // number : 42
        // Given
        let expr = Expr::Literal(Literal {
            value: LiteralValue::Number(42.0),
        });

        // When
        let printer = AstPrinter::new();
        let result = printer.print(&expr);

        assert_eq!(result, "42");
    }
    #[test]
    fn test_literal_string() {
        // "hello world"
        // Given
        let expr = Expr::Literal(Literal {
            value: LiteralValue::String("hello world".to_string()),
        });

        // When
        let printer = AstPrinter::new();
        let result = printer.print(&expr);

        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_literal_bool() {
        // Given
        let true_expr = Expr::Literal(Literal {
            value: LiteralValue::Bool((true)),
        });

        let false_expr = Expr::Literal(Literal {
            value: LiteralValue::Bool((false)),
        });

        // When
        let printer = AstPrinter::new();
        let true_result = printer.print(&true_expr);
        let false_result = printer.print(&false_expr);

        // Then
        assert_eq!(true_result, "true");
        assert_eq!(false_result, "false");
    }

    #[test]
    fn test_literal_nil() {
        // Given
        let expr = Expr::Literal(Literal {
            value: LiteralValue::Nil,
        });

        // When
        let printer = AstPrinter::new();
        let result = printer.print(&expr);

        // Then
        assert_eq!(result, "nil");
    }

    // Binary Type tests
    #[test]
    fn test_simple_binary() {
        //  Given: Expression : 1 + 2
        let expr = Expr::Binary(Binary {
            left: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Number(1.0),
            })),
            operator: Token {
                token_type: TokenType::Plus,
                lexeme: "+".to_string(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Number(2.0),
            })),
        });

        //When
        let printer = AstPrinter::new();
        let result = printer.print(&expr);

        // Then
        assert_eq!(result, "(+ 1 2)");
    }

    #[test]
    fn test_binary_comparison() {
        // 5 == 5
        // Given
        let expr = Expr::Binary(Binary {
            left: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Number(5.0),
            })),
            operator: Token {
                token_type: TokenType::EqualEqual,
                lexeme: "==".to_string(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Number(5.0),
            })),
        });

        // When
        let printer = AstPrinter::new();
        let result = printer.print(&expr);

        // Then
        assert_eq!(result, "(== 5 5)");
    }

    #[test]
    fn test_unary_minus() {
        // Given: -5
        let expr = Expr::Unary(Unary {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Number(5.0),
            })),
        });

        // When
        let printer = AstPrinter::new();
        let result = printer.print(&expr);

        assert_eq!(result, "(- 5)");
    }

    #[test]
    fn test_unary_not() {
        // Given: !true
        let expr = Expr::Unary(Unary {
            operator: Token {
                token_type: TokenType::Bang,
                literal: None,
                lexeme: "!".to_string(),
                line: 1,
            },
            right: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Bool(true),
            })),
        });

        // When
        let printer = AstPrinter::new();
        let result = printer.print(&expr);

        assert_eq!(result, "(! true)");
    }

    #[test]
    fn test_grouping() {
        // Given: (42)
        let expr = Expr::Grouping(Grouping {
            expression: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Number(42.0),
            })),
        });

        // When
        let printer = AstPrinter::new();
        let result = printer.print(&expr);

        // Then
        assert_eq!(result, "(group 42)");
    }

    #[test]
    fn test_nested_unary() {
        // Given: --5
        let expr = Expr::Unary(Unary {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expr::Unary(Unary {
                operator: Token {
                    token_type: TokenType::Minus,
                    lexeme: "-".to_string(),
                    literal: None,
                    line: 1,
                },
                right: Box::new(Expr::Literal(Literal {
                    value: LiteralValue::Number(5.0),
                })),
            })),
        });

        // When
        let printer = AstPrinter::new();
        let result = printer.print(&expr);

        // Then
        assert_eq!(result, "(- (- 5))");
    }

    #[test]
    fn test_complex_nested() {
        // Given: -(1 + 2)
        let expr = Expr::Unary(Unary {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expr::Grouping(Grouping {
                expression: Box::new(Expr::Binary(Binary {
                    left: Box::new(Expr::Literal(Literal {
                        value: LiteralValue::Number(1.0),
                    })),
                    operator: Token {
                        token_type: TokenType::Plus,
                        lexeme: "+".to_string(),
                        literal: None,
                        line: 1,
                    },
                    right: Box::new(Expr::Literal(Literal {
                        value: LiteralValue::Number(2.0),
                    })),
                })),
            })),
        });

        // When
        let printer = AstPrinter::new();
        let result = printer.print(&expr);

        // Then
        assert_eq!(result, "(- (group (+ 1 2)))");
    }

    #[test]
    fn test_book_example() {
        // Given: -123 * (45.67) - exmaple in book
        let expr = Expr::Binary(Binary {
            left: Box::new(Expr::Unary(Unary {
                operator: Token {
                    token_type: TokenType::Minus,
                    lexeme: "-".to_string(),
                    literal: None,
                    line: 1,
                },
                right: Box::new(Expr::Literal(Literal {
                    value: LiteralValue::Number(123.0),
                })),
            })),
            operator: Token {
                token_type: TokenType::Star,
                lexeme: "*".to_string(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expr::Grouping(Grouping {
                expression: Box::new(Expr::Literal(Literal {
                    value: LiteralValue::Number(45.67),
                })),
            })),
        });

        // When
        let printer = AstPrinter::new();
        let result = printer.print(&expr);

        // Then
        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}
