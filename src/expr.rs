use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Assign(Assign),
    Binary(Binary),
    Call(Call),
    Get(Get),
    Grouping(Grouping),
    Literal(Literal),
    Logical(Logical),
    Set(Set),
    Super(Super),
    This(This),
    Unary(Unary),
    Variable(Variable),
}

/// Assignment expression
///
/// # Examples
/// - `num = 1`
/// - `x = 5`
/// - `name = "Bob"`
#[derive(Debug, Clone, PartialEq)]
pub struct Assign {
    pub name: Token,
    pub value: Box<Expr>,
}

/// Binary expression
///
/// # Examples
/// - `3 + 4`
/// - `x + 3`
/// - `x + y`
/// - `a == b`
/// - `10 / 2`
#[derive(Debug, Clone, PartialEq)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

/// Function call expression
///
/// # Examples
/// - `print("hello")`
/// - `max(1, 2, 3)`
/// - `calculate()`
#[derive(Debug, Clone, PartialEq)]
pub struct Call {
    pub callee: Box<Expr>,
    pub paren: Token,
    pub arguments: Vec<Expr>,
}

/// Property access expression
///
/// # Examples
/// - `person.age`
/// - `car.color`
/// - `user.name`
#[derive(Debug, Clone, PartialEq)]
pub struct Get {
    pub object: Box<Expr>,
    pub name: Token,
}

/// Grouping expression (parentheses)
///
/// # Examples
/// - `(1 + 2)`
/// - `(x * y)`
#[derive(Debug, Clone, PartialEq)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

/// Literal value expression
///
/// # Examples
/// - `123`
/// - `"hello"`
/// - `true`
/// - `nil`
#[derive(Debug, Clone, PartialEq)]
pub struct Literal {
    pub value: LiteralValue,
}

/// Logical expression (and, or)
///
/// # Examples
/// - `x > 0 and x < 10`
/// - `age < 18 or hasPermit`
#[derive(Debug, Clone, PartialEq)]
pub struct Logical {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

/// Property assignment expression
///
/// # Examples
/// - `person.age = 25`
/// - `car.color = "red"`
#[derive(Debug, Clone, PartialEq)]
pub struct Set {
    pub object: Box<Expr>,
    pub name: Token,
    pub value: Box<Expr>,
}

/// Super expression (parent class method call)
///
/// # Examples
/// - `super.cook()`
/// - `super.init()`
#[derive(Debug, Clone, PartialEq)]
pub struct Super {
    pub keyword: Token,
    pub method: Token,
}

/// This expression (current object reference)
///
/// # Examples
/// - `this.name`
/// - `this.age`
#[derive(Debug, Clone, PartialEq)]
pub struct This {
    pub keyword: Token,
}

/// Unary expression (prefix operator)
///
/// # Examples
/// - `-5`
/// - `!true`
/// - `-x`
#[derive(Debug, Clone, PartialEq)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

/// Variable reference expression
///
/// # Examples
/// - `x`
/// - `count`
/// - `userName`
#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn test_assign_creation() {
        // num = 1
        let assign = Assign {
            name: Token {
                token_type: TokenType::Identifier,
                lexeme: "num".to_string(),
                literal: None,
                line: 1,
            },
            value: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Number(1.0),
            })),
        };
        assert_eq!(assign.name.lexeme, "num");
        assert_eq!(
            *assign.value,
            Expr::Literal(Literal {
                value: LiteralValue::Number(1.0)
            })
        );
    }

    #[test]
    fn test_binary_addition() {
        // 3 + 4
        let binary = Binary {
            left: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Number(3.0),
            })),
            operator: Token {
                token_type: TokenType::Plus,
                lexeme: "+".to_string(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Number(4.0),
            })),
        };

        assert_eq!(
            *binary.left,
            Expr::Literal(Literal {
                value: LiteralValue::Number(3.0)
            })
        );
        assert_eq!(binary.operator.lexeme, "+");
        assert_eq!(
            *binary.right,
            Expr::Literal(Literal {
                value: LiteralValue::Number(4.0)
            })
        );
    }

    #[test]
    fn test_binary_two_variables() {
        // x + y
        let binary = Binary {
            left: Box::new(Expr::Variable(Variable {
                name: Token {
                    token_type: TokenType::Identifier,
                    lexeme: "x".to_string(),
                    literal: None,
                    line: 1,
                },
            })),
            operator: Token {
                token_type: TokenType::Plus,
                lexeme: "+".to_string(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expr::Variable(Variable {
                name: Token {
                    token_type: TokenType::Identifier,
                    lexeme: "y".to_string(),
                    literal: None,
                    line: 1,
                },
            })),
        };

        assert_eq!(binary.operator.lexeme, "+");
        match (&*binary.left, &*binary.right) {
            (Expr::Variable(l), Expr::Variable(r)) => {
                assert_eq!(l.name.lexeme, "x");
                assert_eq!(r.name.lexeme, "y");
            }
            _ => panic!("Expected two variables"),
        }
    }

    #[test]
    fn test_unary_negation() {
        // -5
        let unary = Unary {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
            },
            right: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Number(5.0),
            })),
        };

        assert_eq!(unary.operator.lexeme, "-");
        assert_eq!(
            *unary.right,
            Expr::Literal(Literal {
                value: LiteralValue::Number(5.0)
            })
        );
    }

    #[test]
    fn test_grouping() {
        // (1 + 2)
        let grouping = Grouping {
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
        };

        match &*grouping.expression {
            Expr::Binary(b) => assert_eq!(b.operator.lexeme, "+"),
            _ => panic!("Expected binary expression"),
        }
    }

    #[test]
    fn test_literal_values() {
        let num = Literal {
            value: LiteralValue::Number(42.0),
        };
        let str = Literal {
            value: LiteralValue::String("hello".to_string()),
        };
        let bool_val = Literal {
            value: LiteralValue::Bool(true),
        };
        let nil = Literal {
            value: LiteralValue::Nil,
        };

        assert_eq!(num.value, LiteralValue::Number(42.0));
        assert_eq!(str.value, LiteralValue::String("hello".to_string()));
        assert_eq!(bool_val.value, LiteralValue::Bool(true));
        assert_eq!(nil.value, LiteralValue::Nil);
    }

    #[test]
    fn test_variable() {
        // x
        let var = Variable {
            name: Token {
                token_type: TokenType::Identifier,
                lexeme: "count".to_string(),
                literal: None,
                line: 1,
            },
        };

        assert_eq!(var.name.lexeme, "count");
    }

    #[test]
    fn test_call() {
        // print("hello")
        let call = Call {
            callee: Box::new(Expr::Variable(Variable {
                name: Token {
                    token_type: TokenType::Identifier,
                    lexeme: "print".to_string(),
                    literal: None,
                    line: 1,
                },
            })),
            paren: Token {
                token_type: TokenType::RightParen,
                lexeme: ")".to_string(),
                literal: None,
                line: 1,
            },
            arguments: vec![Expr::Literal(Literal {
                value: LiteralValue::String("hello".to_string()),
            })],
        };

        assert_eq!(call.arguments.len(), 1);
    }
}
