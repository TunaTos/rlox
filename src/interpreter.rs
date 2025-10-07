use crate::expr::{Binary, Expr, Grouping, Literal, Unary};
use crate::lox_value::LoxValue;
use crate::runtime_error::RuntimeError;
use crate::token::TokenType;

/// Lox Interpreter
/// 
pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<LoxValue, RuntimeError> {
        match expr {
            Expr::Literal(lit) => Ok(self.visit_literal(lit)),
            Expr::Grouping(grp) => self.visit_grouping(grp),
            Expr::Unary(un) => self.visit_unary(un),
            Expr::Binary(bin) => self.visit_binary(bin),
            _ => {
                let dummy_token = crate::token::Token {
                    token_type: crate::token::TokenType::Eof,
                    lexeme: String::new(),
                    literal: None,
                    line: 0,
                };
                Err(RuntimeError::new(
                    &dummy_token,
                    "This expression type is not yet implemented",
                ))
            }
        }
    }

    fn visit_literal(&mut self, literal: &Literal) -> LoxValue {
        literal.value.clone().into()
    }

    fn visit_grouping(&mut self, grouping: &Grouping) -> Result<LoxValue, RuntimeError> {
        self.evaluate(&grouping.expression)
    }

    fn visit_unary(&mut self, unary: &Unary) -> Result<LoxValue, RuntimeError> {
        let right = self.evaluate(&unary.right)?;

        match unary.operator.token_type {
            TokenType::Minus => {
                match right.as_number() {
                    Some(n) => Ok(LoxValue::Number(-n)),
                    None => Err(RuntimeError::new(
                        &unary.operator,
                        "Operand must be a number.",
                    )),
                }
            }
            TokenType::Bang => {
                Ok(LoxValue::Bool(!right.is_truthy()))
            }
            _ => Err(RuntimeError::new(
                &unary.operator,
                "Invalid unary operator.",
            )),
        }
    }

    fn visit_binary(&mut self, binary: &Binary) -> Result<LoxValue, RuntimeError> {
        let left = self.evaluate(&binary.left)?;
        let right = self.evaluate(&binary.right)?;

        match binary.operator.token_type {
            TokenType::Minus => {
                match (left.as_number(), right.as_number()) {
                    (Some(l), Some(r)) => Ok(LoxValue::Number(l - r)),
                    _ => Err(RuntimeError::new(
                        &binary.operator,
                        "Operands must be numbers.",
                    )),
                }
            }
            TokenType::Star => {
                match (left.as_number(), right.as_number()) {
                    (Some(l), Some(r)) => Ok(LoxValue::Number(l * r)),
                    _ => Err(RuntimeError::new(
                        &binary.operator,
                        "Operands must be numbers.",
                    )),
                }
            }
            TokenType::Slash => {
                match (left.as_number(), right.as_number()) {
                    (Some(l), Some(r)) => {
                        if r == 0.0 {
                            Err(RuntimeError::new(
                                &binary.operator,
                                "Division by zero.",
                            ))
                        } else {
                            Ok(LoxValue::Number(l / r))
                        }
                    }
                    _ => Err(RuntimeError::new(
                        &binary.operator,
                        "Operands must be numbers.",
                    )),
                }
            }
            TokenType::Plus => {
                match (left.as_number(), right.as_number()) {
                    (Some(l), Some(r)) => Ok(LoxValue::Number(l + r)),
                    _ => match (left.as_string(), right.as_string()) {
                        (Some(l), Some(r)) => {
                            Ok(LoxValue::String(format!("{}{}", l, r)))
                        }
                        _ => Err(RuntimeError::new(
                            &binary.operator,
                            "Operands must be two numbers or two strings.",
                        )),
                    },
                }
            }

            TokenType::Greater => {
                match (left.as_number(), right.as_number()) {
                    (Some(l), Some(r)) => Ok(LoxValue::Bool(l > r)),
                    _ => Err(RuntimeError::new(
                        &binary.operator,
                        "Operands must be numbers.",
                    )),
                }
            }
            TokenType::GreaterEqual => {
                match (left.as_number(), right.as_number()) {
                    (Some(l), Some(r)) => Ok(LoxValue::Bool(l >= r)),
                    _ => Err(RuntimeError::new(
                        &binary.operator,
                        "Operands must be numbers.",
                    )),
                }
            }
            TokenType::Less => {
                match (left.as_number(), right.as_number()) {
                    (Some(l), Some(r)) => Ok(LoxValue::Bool(l < r)),
                    _ => Err(RuntimeError::new(
                        &binary.operator,
                        "Operands must be numbers.",
                    )),
                }
            }
            TokenType::LessEqual => {
                match (left.as_number(), right.as_number()) {
                    (Some(l), Some(r)) => Ok(LoxValue::Bool(l <= r)),
                    _ => Err(RuntimeError::new(
                        &binary.operator,
                        "Operands must be numbers.",
                    )),
                }
            }

            TokenType::EqualEqual => {
                Ok(LoxValue::Bool(self.is_equal(&left, &right)))
            }
            TokenType::BangEqual => {
                Ok(LoxValue::Bool(!self.is_equal(&left, &right)))
            }

            _ => Err(RuntimeError::new(
                &binary.operator,
                "Invalid binary operator.",
            )),
        }
    }

    fn is_equal(&self, left: &LoxValue, right: &LoxValue) -> bool {
        left == right
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::LiteralValue;

    #[test]
    fn test_evaluate_literal_number() {
        let mut interpreter = Interpreter::new();
        let expr = Expr::Literal(Literal {
            value: LiteralValue::Number(42.0),
        });
        let result = interpreter.evaluate(&expr).unwrap();
        assert_eq!(result, LoxValue::Number(42.0));
    }

    #[test]
    fn test_evaluate_unary_minus() {
        let mut interpreter = Interpreter::new();
        let token = crate::token::Token {
            token_type: TokenType::Minus,
            lexeme: "-".to_string(),
            literal: None,
            line: 1,
        };
        let expr = Expr::Unary(Unary {
            operator: token,
            right: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Number(5.0),
            })),
        });
        let result = interpreter.evaluate(&expr).unwrap();
        assert_eq!(result, LoxValue::Number(-5.0));
    }

    #[test]
    fn test_evaluate_binary_addition() {
        let mut interpreter = Interpreter::new();
        let token = crate::token::Token {
            token_type: TokenType::Plus,
            lexeme: "+".to_string(),
            literal: None,
            line: 1,
        };
        let expr = Expr::Binary(Binary {
            left: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Number(1.0),
            })),
            operator: token,
            right: Box::new(Expr::Literal(Literal {
                value: LiteralValue::Number(2.0),
            })),
        });
        let result = interpreter.evaluate(&expr).unwrap();
        assert_eq!(result, LoxValue::Number(3.0));
    }
}