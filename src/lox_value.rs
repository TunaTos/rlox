use core::fmt;
use std::fmt::Formatter;

use crate::expr::LiteralValue;
/// runtime value of Lox
///
/// Lox is dynamic type language
/// value for interpreter.rs
#[derive(Debug, Clone, PartialEq)]
pub enum LoxValue {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl LoxValue {
    // check whether LoxValue is truthy or not
    // bool and nil are false
    pub fn is_truthy(&self) -> bool {
        match self {
            LoxValue::Bool(false) | LoxValue::Nil => false,
            _ => true,
        }
    }

    // check type is Number and return value
    pub fn as_number(&self) -> Option<f64> {
        match self {
            LoxValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    // check type is string and return value
    pub fn as_string(&self) -> Option<&str> {
        match self {
            LoxValue::String(s) => Some(&s),
            _ => None,
        }
    }

    // check type is bool and return value
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            LoxValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    // check type is nil and return value
    pub fn as_nil(&self) -> bool {
        matches!(self, LoxValue::Nil)
    }
}

// change from LiteralValue to LoxValue
impl From<LiteralValue> for LoxValue {
    fn from(literal_value: LiteralValue) -> Self {
        match literal_value {
            LiteralValue::Number(n) => LoxValue::Number(n),
            LiteralValue::String(s) => LoxValue::String(s),
            LiteralValue::Bool(b) => LoxValue::Bool(b),
            LiteralValue::Nil => LoxValue::Nil,
        }
    }
}

impl fmt::Display for LoxValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LoxValue::Number(n) => {
                if n.fract() == 0.0 && n.is_finite() {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            LoxValue::String(s) => write!(f, "{}", s),
            LoxValue::Bool(b) => write!(f, "{}", b),
            LoxValue::Nil => write!(f, "nil"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lox_value::LoxValue;

    #[cfg(test)]
    mod test {
        use crate::expr::LiteralValue;
        use crate::lox_value::LoxValue;

        // ===== Truthiness Tests =====

        #[test]
        fn test_is_truthy_bool_false() {
            let bool_false = LoxValue::Bool(false);
            assert_eq!(bool_false.is_truthy(), false);
        }

        #[test]
        fn test_is_truthy_bool_true() {
            let bool_true = LoxValue::Bool(true);
            assert_eq!(bool_true.is_truthy(), true);
        }

        #[test]
        fn test_is_truthy_nil() {
            let nil_false = LoxValue::Nil;
            assert_eq!(nil_false.is_truthy(), false);
        }

        #[test]
        fn test_is_truthy_number_zero() {
            let number_zero = LoxValue::Number(0.0);
            assert_eq!(number_zero.is_truthy(), true);
        }

        #[test]
        fn test_is_truthy_number_positive() {
            let number_positive = LoxValue::Number(42.0);
            assert_eq!(number_positive.is_truthy(), true);
        }

        #[test]
        fn test_is_truthy_number_negative() {
            let number_negative = LoxValue::Number(-5.0);
            assert_eq!(number_negative.is_truthy(), true);
        }

        #[test]
        fn test_is_truthy_string_empty() {
            let string_empty = LoxValue::String(String::from(""));
            assert_eq!(string_empty.is_truthy(), true);
        }

        #[test]
        fn test_is_truthy_string_non_empty() {
            let string_non_empty = LoxValue::String(String::from("hello"));
            assert_eq!(string_non_empty.is_truthy(), true);
        }

        // ===== Type Checker Tests =====

        #[test]
        fn test_as_number_success() {
            let lox_number = LoxValue::Number(42.0);
            assert_eq!(lox_number.as_number(), Some(42.0));
        }

        #[test]
        fn test_as_number_failure() {
            let lox_string = LoxValue::String(String::from("not a number"));
            assert_eq!(lox_string.as_number(), None);

            let lox_bool = LoxValue::Bool(true);
            assert_eq!(lox_bool.as_number(), None);

            let lox_nil = LoxValue::Nil;
            assert_eq!(lox_nil.as_number(), None);
        }

        #[test]
        fn test_as_string_success() {
            let lox_string = LoxValue::String(String::from("Hello World"));
            assert_eq!(lox_string.as_string(), Some("Hello World"));
        }

        #[test]
        fn test_as_string_failure() {
            let lox_number = LoxValue::Number(42.0);
            assert_eq!(lox_number.as_string(), None);

            let lox_bool = LoxValue::Bool(true);
            assert_eq!(lox_bool.as_string(), None);

            let lox_nil = LoxValue::Nil;
            assert_eq!(lox_nil.as_string(), None);
        }

        #[test]
        fn test_as_bool_success() {
            let lox_bool_true = LoxValue::Bool(true);
            assert_eq!(lox_bool_true.as_bool(), Some(true));

            let lox_bool_false = LoxValue::Bool(false);
            assert_eq!(lox_bool_false.as_bool(), Some(false));
        }

        #[test]
        fn test_as_bool_failure() {
            let lox_number = LoxValue::Number(42.0);
            assert_eq!(lox_number.as_bool(), None);

            let lox_string = LoxValue::String(String::from("true"));
            assert_eq!(lox_string.as_bool(), None);

            let lox_nil = LoxValue::Nil;
            assert_eq!(lox_nil.as_bool(), None);
        }

        #[test]
        fn test_as_nil_success() {
            let lox_nil = LoxValue::Nil;
            assert_eq!(lox_nil.as_nil(), true);
        }

        #[test]
        fn test_as_nil_failure() {
            let lox_number = LoxValue::Number(42.0);
            assert_eq!(lox_number.as_nil(), false);

            let lox_string = LoxValue::String(String::from("nil"));
            assert_eq!(lox_string.as_nil(), false);

            let lox_bool = LoxValue::Bool(false);
            assert_eq!(lox_bool.as_nil(), false);
        }

        // ===== Display Tests =====

        #[test]
        fn test_display_number_integer() {
            assert_eq!(LoxValue::Number(42.0).to_string(), "42");
            assert_eq!(LoxValue::Number(0.0).to_string(), "0");
            assert_eq!(LoxValue::Number(-5.0).to_string(), "-5");
        }

        #[test]
        fn test_display_number_float() {
            assert_eq!(LoxValue::Number(3.14).to_string(), "3.14");
            assert_eq!(LoxValue::Number(0.5).to_string(), "0.5");
            assert_eq!(LoxValue::Number(-2.718).to_string(), "-2.718");
        }

        #[test]
        fn test_display_string() {
            assert_eq!(LoxValue::String(String::from("hello")).to_string(), "hello");
            assert_eq!(LoxValue::String(String::from("")).to_string(), "");
            assert_eq!(
                LoxValue::String(String::from("Hello World!")).to_string(),
                "Hello World!"
            );
        }

        #[test]
        fn test_display_bool() {
            assert_eq!(LoxValue::Bool(true).to_string(), "true");
            assert_eq!(LoxValue::Bool(false).to_string(), "false");
        }

        #[test]
        fn test_display_nil() {
            assert_eq!(LoxValue::Nil.to_string(), "nil");
        }

        // ===== From<LiteralValue> Tests =====

        #[test]
        fn test_from_literal_number() {
            let literal = LiteralValue::Number(42.0);
            let lox_value: LoxValue = literal.into();
            assert_eq!(lox_value, LoxValue::Number(42.0));
        }

        #[test]
        fn test_from_literal_string() {
            let literal = LiteralValue::String(String::from("test"));
            let lox_value: LoxValue = literal.into();
            assert_eq!(lox_value, LoxValue::String(String::from("test")));
        }

        #[test]
        fn test_from_literal_bool() {
            let literal_true = LiteralValue::Bool(true);
            let lox_value_true: LoxValue = literal_true.into();
            assert_eq!(lox_value_true, LoxValue::Bool(true));

            let literal_false = LiteralValue::Bool(false);
            let lox_value_false: LoxValue = literal_false.into();
            assert_eq!(lox_value_false, LoxValue::Bool(false));
        }

        #[test]
        fn test_from_literal_nil() {
            let literal = LiteralValue::Nil;
            let lox_value: LoxValue = literal.into();
            assert_eq!(lox_value, LoxValue::Nil);
        }

        // ===== Edge Cases =====

        #[test]
        fn test_number_special_values() {
            // Infinity
            let infinity = LoxValue::Number(f64::INFINITY);
            assert_eq!(infinity.to_string(), "inf");

            // Negative Infinity
            let neg_infinity = LoxValue::Number(f64::NEG_INFINITY);
            assert_eq!(neg_infinity.to_string(), "-inf");

            // NaN - is_finite()이 false이므로 그대로 출력
            let nan = LoxValue::Number(f64::NAN);
            assert_eq!(nan.to_string(), "NaN");
        }

        #[test]
        fn test_string_special_characters() {
            let newline = LoxValue::String(String::from("hello\nworld"));
            assert_eq!(newline.to_string(), "hello\nworld");

            let tab = LoxValue::String(String::from("hello\tworld"));
            assert_eq!(tab.to_string(), "hello\tworld");

            let quote = LoxValue::String(String::from("say \"hello\""));
            assert_eq!(quote.to_string(), "say \"hello\"");
        }

        // ===== Clone and PartialEq Tests =====

        #[test]
        fn test_clone() {
            let original = LoxValue::Number(42.0);
            let cloned = original.clone();
            assert_eq!(original, cloned);

            let string_original = LoxValue::String(String::from("hello"));
            let string_cloned = string_original.clone();
            assert_eq!(string_original, string_cloned);
        }

        #[test]
        fn test_equality() {
            // Same type, same value
            assert_eq!(LoxValue::Number(42.0), LoxValue::Number(42.0));
            assert_eq!(
                LoxValue::String(String::from("hello")),
                LoxValue::String(String::from("hello"))
            );
            assert_eq!(LoxValue::Bool(true), LoxValue::Bool(true));
            assert_eq!(LoxValue::Nil, LoxValue::Nil);

            // Different values
            assert_ne!(LoxValue::Number(42.0), LoxValue::Number(43.0));
            assert_ne!(
                LoxValue::String(String::from("hello")),
                LoxValue::String(String::from("world"))
            );
            assert_ne!(LoxValue::Bool(true), LoxValue::Bool(false));

            // Different types
            assert_ne!(LoxValue::Number(42.0), LoxValue::String(String::from("42")));
            assert_ne!(LoxValue::Bool(true), LoxValue::Number(1.0));
            assert_ne!(LoxValue::Nil, LoxValue::Bool(false));
        }
    }
}
