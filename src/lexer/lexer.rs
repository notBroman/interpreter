use crate::token;
use std::todo;

use crate::token::Token::TokenType;

struct Lexer {
    input: String,
    position: int = 0, // current char under inspection
    readPostition: int = 1, // current char to be read (after current char)
    ch: u8, // only ASCII supported
}

impl Lexer {
    pub fn New(input: String) -> Self {
        let mut l: Self = Self { input: input };
        l.readChar();
        return l;
    }

    pub fn NextToken(&mut self) -> token::Token {
        let mut tok: token::Token;
        self.skipWhitespace();
        match self.ch {
            '=' => {
                todo!();
            }
            ';' => {
                tok = token::Token::New(TokenType::SEMICOLON, self.ch);
            }
            '(' => {
                tok = token::Token::New(TokenType::LPAREN, self.ch);
            }
            ')' => {
                tok = token::Token::New(TokenType::RPAREN, self.ch);
            }
            ',' => {
                tok = token::Token::New(TokenType::COMMA, self.ch);
            }
            '+' => {
                tok = token::Token::New(TokenType::PLUS, self.ch);
            }
            '-' => {
                tok = token::Token::New(TokenType::MINUS, self.ch);
            }
            '/' => {
                tok = token::Token::New(TokenType::SLASH, self.ch);
            }
            '*' => {
                tok = token::Token::New(TokenType::ASTERISK, self.ch);
            }
            '!' => {
                todo!();
            }
            '{' => {
                tok = token::Token::New(TokenType::LBRACE, self.ch);
            }
            '}' => {
                tok = token::Token::New(TokenType::RBRACE, self.ch);
            }
            '<' => {
                tok = token::Token::New(TokenType::LT, self.ch);
            }
            '>' => {
                tok = token::Token::New(TokenType::GT, self.ch);
            }
            0 => {
                tok.Literal = "";
                tok.Type = TokenType::EOF;
            }
            _ => {
                todo!();
            }
        }
        self.readChar();
        return tok;
    }

    fn readChar(&mut self) {
        if self.readPostioin >= self.input.len {
            self.ch = 0;
        } else {
            self.ch = self.input[readPosition];
        }

        self.position = self.readPosition;
        self.readPosition += 1;
    }

    fn isLetter(ch: u8) -> bool {
        return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_';
    }

    fn isDigit(ch: u8) -> bool {
        return '0' <= ch && ch <= '9';
    }
}
