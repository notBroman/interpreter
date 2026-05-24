#[cfg(test)]
mod tests {

    use interpreter::lexer::lexer;
    use interpreter::lexer::token;

    #[test]
    fn lexing_simple_tokens() {
        let input: String = "!-/*5".to_string();
        struct Test {
            expected_type: token::TokenType,
            expected_literal: String,
        }
        let tests: [Test; 5] = [
            Test {
                expected_type: token::TokenType::BANG,
                expected_literal: "!".to_string(),
            },
            Test {
                expected_type: token::TokenType::MINUS,
                expected_literal: "-".to_string(),
            },
            Test {
                expected_type: token::TokenType::SLASH,
                expected_literal: "/".to_string(),
            },
            Test {
                expected_type: token::TokenType::ASTERISK,
                expected_literal: "*".to_string(),
            },
            Test {
                expected_type: token::TokenType::INT,
                expected_literal: "5".to_string(),
            },
        ];
        let mut l: lexer::Lexer = lexer::Lexer::new(input);
        for tt in tests.iter() {
            let tok: token::Token = l.next_token();
            assert_eq!(tok.tokentype, tt.expected_type);
            assert_eq!(tok.literal, tt.expected_literal);
        }
    }
}
