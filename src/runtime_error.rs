use core::fmt;

use crate::token::Token;

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl RuntimeError {
    pub fn new(token: &Token, message: &str) -> Self {
        Self {
            token: token.clone(),
            message: message.to_string(),
        }
    }

    pub fn format_error(&self) -> String {
        format!(
            "[line {}] Runtime Error at '{}': {}",
            self.token.line, self.token.lexeme, self.message
        )
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[line {}] Runtime Error at '{}': {}",
            self.token.line, self.token.lexeme, self.message
        )
    }
}

impl std::error::Error for RuntimeError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;

    // ===== Helper Functions =====

    fn make_token(token_type: TokenType, lexeme: &str, line: usize) -> Token {
        Token {
            token_type,
            lexeme: lexeme.to_string(),
            literal: None,
            line,
        }
    }

    // ===== 1. basic test =====

    #[test]
    fn test_new_creates_error_with_message() {
        // Given
        let token = make_token(TokenType::Plus, "+", 5);
        let message = "Operands must be numbers.";

        // When
        let error = RuntimeError::new(&token, message);

        // Then
        assert_eq!(error.message, message);
        assert_eq!(error.token.line, 5);
        assert_eq!(error.token.lexeme, "+");
    }

    #[test]
    fn test_new_stores_token_correctly() {
        // Given
        let token = make_token(TokenType::Minus, "-", 10);

        // When
        let error = RuntimeError::new(&token, "Test error");

        // Then
        assert_eq!(error.token.token_type, TokenType::Minus);
        assert_eq!(error.token.line, 10);
    }

    #[test]
    fn test_new_with_different_tokens() {
        let tokens = vec![
            make_token(TokenType::Plus, "+", 1),
            make_token(TokenType::Minus, "-", 2),
            make_token(TokenType::Star, "*", 3),
            make_token(TokenType::Slash, "/", 4),
        ];

        for token in tokens {
            let error = RuntimeError::new(&token, "Test");
            assert_eq!(error.token.token_type, token.token_type);
        }
    }

    // ===== 2. Display test =====

    #[test]
    fn test_display_contains_line_number() {
        // Given
        let token = make_token(TokenType::Plus, "+", 42);
        let error = RuntimeError::new(&token, "Test error");

        // When
        let display = error.to_string();

        // Then
        assert!(display.contains("line 42"));
    }

    #[test]
    fn test_display_contains_runtime_error_label() {
        // Given
        let token = make_token(TokenType::Plus, "+", 1);
        let error = RuntimeError::new(&token, "Test");

        // When
        let display = error.to_string();

        // Then
        assert!(display.contains("Runtime Error"));
    }

    #[test]
    fn test_display_contains_message() {
        // Given
        let token = make_token(TokenType::Plus, "+", 1);
        let message = "Operands must be numbers.";
        let error = RuntimeError::new(&token, message);

        // When
        let display = error.to_string();

        // Then
        assert!(display.contains(message));
    }

    #[test]
    fn test_display_format() {
        // Given
        let token = make_token(TokenType::Plus, "+", 5);
        let error = RuntimeError::new(&token, "Test error");

        // When
        let display = error.to_string();

        // Then
        assert_eq!(display, "[line 5] Runtime Error at '+': Test error");
    }

    // ===== 3. format_error test =====

    #[test]
    fn test_format_error_includes_lexeme() {
        // Given
        let token = make_token(TokenType::Plus, "+", 1);
        let error = RuntimeError::new(&token, "Test");

        // When
        let formatted = error.format_error();

        // Then
        assert!(formatted.contains("'+'"));
    }

    #[test]
    fn test_format_error_complete_format() {
        // Given
        let token = make_token(TokenType::Slash, "/", 7);
        let error = RuntimeError::new(&token, "Cannot divide");

        // When
        let formatted = error.format_error();

        // Then
        assert_eq!(
            formatted,
            "[line 7] Runtime Error at '/': Cannot divide"
        );
    }

    // ===== 4. Clone test =====

    #[test]
    fn test_clone_creates_independent_copy() {
        // Given
        let token = make_token(TokenType::Plus, "+", 1);
        let original = RuntimeError::new(&token, "Original");

        // When
        let cloned = original.clone();

        // Then
        assert_eq!(original.message, cloned.message);
        assert_eq!(original.token.line, cloned.token.line);
        assert_eq!(original.token.lexeme, cloned.token.lexeme);
    }

    #[test]
    fn test_clone_with_different_errors() {
        // Given
        let errors = vec![
            RuntimeError::new(&make_token(TokenType::Plus, "+", 1), "Error 1"),
            RuntimeError::new(&make_token(TokenType::Minus, "-", 2), "Error 2"),
        ];

        // When
        let cloned: Vec<_> = errors.iter().cloned().collect();

        // Then
        assert_eq!(errors.len(), cloned.len());
        for (original, cloned) in errors.iter().zip(cloned.iter()) {
            assert_eq!(original.message, cloned.message);
        }
    }

    // ===== 5. Error trait test =====

    #[test]
    fn test_implements_error_trait() {
        // Given
        let token = make_token(TokenType::Plus, "+", 1);
        let error = RuntimeError::new(&token, "Test");

        // When
        let error_trait: &dyn std::error::Error = &error;

        // Then
        assert!(error_trait.to_string().contains("Test"));
    }

    #[test]
    fn test_can_be_boxed_as_error() {
        // Given
        let token = make_token(TokenType::Plus, "+", 1);
        let error = RuntimeError::new(&token, "Test");

        // When
        let boxed: Box<dyn std::error::Error> = Box::new(error);

        // Then
        assert!(boxed.to_string().contains("Test"));
    }

    #[test]
    fn test_works_with_result() {
        // Given
        fn returns_error() -> Result<(), RuntimeError> {
            let token = make_token(TokenType::Plus, "+", 1);
            Err(RuntimeError::new(&token, "Test"))
        }

        // When
        let result = returns_error();

        // Then
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.message, "Test");
    }

    // ===== 6. Edge Cases =====

    #[test]
    fn test_empty_message() {
        // Given
        let token = make_token(TokenType::Plus, "+", 1);

        // When
        let error = RuntimeError::new(&token, "");

        // Then
        assert_eq!(error.message, "");
        assert!(error.to_string().contains("Runtime Error"));
    }

    #[test]
    fn test_empty_lexeme() {
        // Given
        let token = Token {
            token_type: TokenType::Eof,
            lexeme: String::new(),
            literal: None,
            line: 1,
        };

        // When
        let error = RuntimeError::new(&token, "Unexpected EOF");

        // Then
        assert_eq!(error.token.lexeme, "");
        assert!(error.to_string().contains("Unexpected EOF"));
    }

    #[test]
    fn test_long_message() {
        // Given
        let token = make_token(TokenType::Plus, "+", 1);
        let long_message = "This is a very long error message that \
                           describes in great detail what went wrong \
                           and how to fix it.";

        // When
        let error = RuntimeError::new(&token, long_message);

        // Then
        assert_eq!(error.message, long_message);
        assert!(error.to_string().contains(long_message));
    }

    #[test]
    fn test_message_with_newlines() {
        // Given
        let token = make_token(TokenType::Plus, "+", 1);
        let message = "Error on line 1\nSuggestion: check types";

        // When
        let error = RuntimeError::new(&token, message);

        // Then
        assert_eq!(error.message, message);
    }

    #[test]
    fn test_special_characters_in_message() {
        // Given
        let token = make_token(TokenType::Plus, "+", 1);
        let message = "Cannot use '\"' in expression";

        // When
        let error = RuntimeError::new(&token, message);

        // Then
        assert!(error.to_string().contains("Cannot use '\"'"));
    }

    #[test]
    fn test_unicode_in_message() {
        // Given
        let token = make_token(TokenType::Plus, "+", 1);
        let message = "타입 에러: 숫자가 필요합니다";

        // When
        let error = RuntimeError::new(&token, message);

        // Then
        assert!(error.to_string().contains("타입 에러"));
    }

    #[test]
    fn test_line_number_zero() {
        // Given
        let token = make_token(TokenType::Plus, "+", 0);

        // When
        let error = RuntimeError::new(&token, "Test");

        // Then
        assert_eq!(error.token.line, 0);
        assert!(error.to_string().contains("line 0"));
    }

    #[test]
    fn test_large_line_number() {
        // Given
        let token = make_token(TokenType::Plus, "+", 999999);

        // When
        let error = RuntimeError::new(&token, "Test");

        // Then
        assert_eq!(error.token.line, 999999);
        assert!(error.to_string().contains("line 999999"));
    }

    // ===== 7. Debug test =====

    #[test]
    fn test_debug_format() {
        // Given
        let token = make_token(TokenType::Plus, "+", 1);
        let error = RuntimeError::new(&token, "Test");

        // When
        let debug = format!("{:?}", error);

        // Then
        assert!(debug.contains("RuntimeError"));
        assert!(debug.contains("Test"));
    }

 
}